use std::{sync::Arc, thread};

use async_std::channel::{unbounded, Receiver, Sender};
use log::error;

use crate::{
    effect::Effect,
    runner::{tick_task, RunnerContext},
    utils::collection::slices,
};

pub fn setup_workers(context: &RunnerContext) -> Vec<(Sender<()>, Receiver<Vec<Effect>>)> {
    let workers_count = num_cpus::get();
    let mut channels = vec![];

    for i in 0..workers_count {
        let (start_work_sender, start_work_receiver) = unbounded();
        let (results_sender, results_receiver) = unbounded();

        channels.push((start_work_sender, results_receiver));

        let state = Arc::clone(&context.state);
        let context = context.clone();
        thread::spawn(move || {
            while start_work_receiver.recv_blocking().is_ok() {
                let state = state.read().expect("Assume state is always accessible");
                let frame = *state.frame();
                let tasks = state.tasks();
                let tasks_count = tasks.len();
                let slices = slices(tasks_count, workers_count);
                let (start, end) = slices[i];
                let mut effects = vec![];

                for task in &tasks[start..end] {
                    match tick_task(&context, task, &frame) {
                        Ok(effects_) => effects.extend(effects_),
                        Err(e) => {
                            eprintln!("Error when tasks execution: {}. Abort.", e);
                            context.context.require_stop();
                            return;
                        }
                    };
                }

                if results_sender.send_blocking(effects).is_err() {
                    error!("Channel closed in tasks scope: abort");
                    return;
                }
            }
        });
    }

    channels
}
