use async_std::task::sleep;
use bevy::{app::PanicHandlerPlugin, log::LogPlugin, prelude::*};
use bevy_async_task::AsyncTaskRunner;
use common::{
    game::PlayerId,
    network::{
        message::{ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage},
        Client, ClientId,
    },
};
use crossbeam::channel::{Receiver, Sender};
use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Resource)]
pub struct ClientToServerReceiverResource(pub Receiver<ClientToServerMessage>);

#[derive(Resource)]
pub struct ClientToServerSenderResource(pub Sender<ClientToServerMessage>);

#[derive(Resource)]
pub struct ServerToClientReceiverResource(pub Receiver<ServerToClientMessage>);

#[derive(Resource)]
pub struct ServerToClientSenderResource(pub Sender<ServerToClientMessage>);

// This function is strongly inspired from https://rustwasm.github.io/wasm-bindgen/examples/websockets.html
async fn websocket(
    url: &str,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<ServerToClientMessage>,
) {
    let ws = WebSocket::new(url).unwrap();
    // For small binary messages, like CBOR: Arraybuffer. Else: Blob.
    ws.set_binary_type(web_sys::BinaryType::Blob);

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            console_log!("message event, received blob: {:?}", blob);
            // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            // create onLoadEnd callback
            let from_server_sender_ = from_server_sender.clone();
            let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
                let message: ServerToClientMessage = bincode::deserialize(&array.to_vec()).unwrap();
                console_log!("{:?}", message);
                from_server_sender_.send(message).unwrap();
            });
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");

        cloned_ws
            .send_with_u8_array(
                &bincode::serialize(&ClientToServerMessage::Network(
                    ClientToServerNetworkMessage::Hello(Client::new(
                        ClientId::default(),
                        PlayerId::default(),
                    )),
                ))
                .unwrap(),
            )
            .unwrap();

        // loop {
        //     sleep(Duration::from_millis(1000)).await;
        // }

        // while let Ok(message) = to_server_receiver.recv() {
        //     let bytes = bincode::serialize(&message).unwrap();
        //     match cloned_ws.send_with_u8_array(&bytes) {
        //         Ok(_) => console_log!("binary message successfully sent"),
        //         Err(err) => console_log!("error sending message: {:?}", err),
        //     }
        // }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    console_log!("end");
}

pub fn setup_network(
    mut task_runner: AsyncTaskRunner<'_, ()>,
    to_server_receiver: Res<ClientToServerReceiverResource>,
    from_server_sender: Res<ServerToClientSenderResource>,
) {
    task_runner.start(websocket(
        "ws://127.0.0.1:9877",
        to_server_receiver.0.clone(),
        from_server_sender.0.clone(),
    ));
}
