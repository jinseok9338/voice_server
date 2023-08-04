use actix::Message;

pub struct WebSocketMessage(pub String);

impl Message for WebSocketMessage {
    type Result = ();
}