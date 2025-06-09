#[macro_export]
macro_rules! to_server {
    ($commands:expr, $msg:expr) => {
        $commands.trigger($crate::bridge::SendMessageToServerEvent($msg.into()));
    };
}

#[macro_export]
macro_rules! send_unit_msg {
    ($commands:expr, $unit_id:expr, $msg:expr) => {
        $commands.trigger($crate::bridge::SendMessageToServerEvent(
            ClientToServerInGameMessage::Unit($unit_id, $msg).into(),
        ))
    };
}
