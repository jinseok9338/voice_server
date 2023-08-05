use std::time::{Duration, Instant};

use actix::prelude::*;

use actix_web_actors::ws;
use uuid::Uuid;

//use crate::database::postgres_pool::Db;
use crate::database::{
    process_message_service::{sending_request_to_redis, SendingRequestToRedis},
    redis::RedisActor,
};

// use crate::domains::message::services::message_service::MessageService;

use super::web_socket_message::WebSocketMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct MyWebSocket {
    hb: Instant,
    user_id: Uuid,
    chat_room_id: Uuid,
}

impl MyWebSocket {
    pub fn new(user_id: Uuid, chat_room_id: Uuid) -> Self {
        Self {
            hb: Instant::now(),
            user_id,
            chat_room_id,
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        // Clone the chat_room_id so it can be moved into the async block
        let chat_room_id = self.chat_room_id.clone();
        let addr = ctx.address();

        // Spawn a new task to subscribe to the Redis channel
        actix_rt::spawn(async move {
            let redis_actor = RedisActor::new(addr);
            // Subscribe to the channel
            let channel_name = format!("channel_{}", chat_room_id);

            redis_actor.subscribe(&channel_name);
            log::debug!("Subscribed to channel {}", channel_name);
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let addr = ctx.address();
                let mut conn = RedisActor::new(addr);
                let channel_name = format!("channel_{}", self.chat_room_id);
                let request = SendingRequestToRedis::Chat {
                    user_id: self.user_id,
                    chat_room_id: self.chat_room_id,
                    message: text.clone().to_string(),
                };
                let message = sending_request_to_redis(request);
                let publish = conn.publish(&channel_name, &message);

                match publish {
                    Ok(_) => {
                        log::debug!("Published message to channel {}", channel_name);
                    }
                    Err(err) => {
                        log::error!("Error publishing message to channel {}", err);
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                let addr: Addr<MyWebSocket> = ctx.address();
                let mut conn = RedisActor::new(addr);
                let channel_name = format!("channel_{}", self.chat_room_id);
                let unsubscribe: Result<(), redis::RedisError> = conn.unsubscribe(&channel_name);
                match unsubscribe {
                    Ok(_) => {
                        log::debug!("Unsubscribed from channel {}", channel_name);
                    }
                    Err(err) => {
                        log::error!("Error unsubscribing from channel {}", err);
                    }
                }
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<WebSocketMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WebSocketMessage, ctx: &mut Self::Context) {
        // Send the payload message to the WebSocket client
        ctx.text(msg.0);
    }
}
