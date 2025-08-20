use std::thread;

use async_std::channel::{unbounded, Receiver, Sender};
use common::game::GameFrame;
use log::error;

use crate::{
    effect::Effect,
    runner::{tick_client, tick_task, RunnerContext},
    utils::collection::slices,
};

pub fn setup_task_workers(context: &RunnerContext) -> Vec<(Sender<()>, Receiver<Vec<Effect>>)> {
    let workers_count = num_cpus::get();
    let mut channels = vec![];

    for i in 0..workers_count {
        let (start_work_sender, start_work_receiver) = unbounded();
        let (results_sender, results_receiver) = unbounded();

        let context = context.clone();
        thread::spawn(move || {
            let results_sender_ = results_sender.clone();
            while start_work_receiver.recv_blocking().is_ok() {
                let lock = context.lock.read().unwrap();
                // let state = context.state();
                // let frame = *state.frame();
                // let tasks = state.tasks();
                // deal(
                //     &context,
                //     workers_count,
                //     tasks,
                //     tick_task,
                //     frame,
                //     i,
                //     &results_sender_,
                // );
                drop(lock);
            }
        });

        channels.push((start_work_sender, results_receiver));
    }

    channels
}

fn deal<T, F, E: std::error::Error>(
    context: &RunnerContext,
    workers_count: usize,
    items: &[T],
    executor: F,
    frame: GameFrame,
    worker_index: usize,
    sender: &Sender<Vec<Effect>>,
) where
    F: Fn(&RunnerContext, &T, &GameFrame) -> Result<Vec<Effect>, E> + Clone + Send + 'static,
    T: std::fmt::Debug,
{
    let tasks_count = items.len();
    let slices = slices(tasks_count, workers_count);
    let (start, end) = slices[worker_index];
    let mut effects = vec![];

    for item in &items[start..end] {
        match executor(context, item, &frame) {
            Ok(effects_) => effects.extend(effects_),
            Err(e) => {
                eprintln!("Error when tasks execution: {}. Abort.", e);
                context.context.require_stop();
                return;
            }
        };
    }

    if sender.send_blocking(effects).is_err() {
        error!("Channel closed in tasks scope: abort")
    }
}

pub fn setup_client_workers(context: &RunnerContext) -> Vec<Receiver<Vec<Effect>>> {
    let workers_count = num_cpus::get();
    let mut channels = vec![];

    for _ in 0..workers_count {
        let context = context.clone();
        let (results_sender, results_receiver) = unbounded();

        thread::spawn(move || {
            while let Ok((client, message)) = context.from_clients_receiver.recv_blocking() {
                let effects = tick_client(&context, &client, &message);
                if results_sender.send_blocking(effects).is_err() {
                    return;
                }
            }
        });

        channels.push(results_receiver);
    }

    channels
}
