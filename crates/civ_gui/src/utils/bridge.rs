#[macro_export]
macro_rules! to_server {
    ($commands:expr, $msg:expr) => {
        $commands.trigger(crate::bridge::SendMessageToServerEvent($msg.into()));
    };
}
