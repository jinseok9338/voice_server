use std::sync::{Arc, Mutex};

use redis::{Client, Connection, RedisError};

use crate::{
    database::process_message_service::{
        json_string_to_process_incoming_message_enum, process_incoming_message,
    },
    domains::web_socket::services::{
        web_socket_message::WebSocketMessage, web_socket_service::MyWebSocket,
    },
};
use actix::Addr;

pub fn create_redis_client() -> Result<Client, RedisError> {
    dotenv::dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    Client::open(redis_url)
}

pub struct RedisActor {
    client: Client,
    conn: Arc<Mutex<Connection>>,
    web_socket_addr: Addr<MyWebSocket>,
}

impl RedisActor {
    pub fn new(web_socket_addr: Addr<MyWebSocket>) -> Self {
        let client = create_redis_client().expect("Error creating redis client");
        let conn = client
            .get_connection()
            .expect("Error getting redis connection");
        Self {
            client,
            conn: Arc::new(Mutex::new(conn)),
            web_socket_addr,
        }
    }

    pub fn subscribe(&self, channel: &str) {
        let conn = Arc::clone(&self.conn);
        let channel = channel.to_string();
        let web_socket_addr = self.web_socket_addr.clone();

        std::thread::spawn(move || {
            let mut conn = conn.lock().unwrap();
            let mut pubsub = conn.as_pubsub();

            pubsub.subscribe(&channel).unwrap();

            loop {
                let msg = pubsub.get_message().unwrap();
                let payload: String = msg.get_payload().unwrap();
                // when the payload arrives Make new message to the chat_rooms
                let message_to_payload = json_string_to_process_incoming_message_enum(&payload);
                let result = process_incoming_message(message_to_payload);
                println!("Got message: {}", result);
                // process the message then send it to the web_socket_addr

                web_socket_addr.do_send(WebSocketMessage(result));
            }
        });
    }

    pub fn publish(&mut self, channel: &str, message: &str) -> redis::RedisResult<()> {
        let conn = self.client.get_connection();
        let mut conn = match conn {
            Ok(conn) => conn,
            Err(err) => return Err(err),
        };
        redis::cmd("PUBLISH")
            .arg(channel)
            .arg(message)
            .execute(&mut conn);
        Ok(())
    }

    pub fn unsubscribe(&mut self, channel: &str) -> redis::RedisResult<()> {
        let conn = Arc::clone(&self.conn);
        let mut conn = conn.lock().unwrap();
        let mut pubsub = conn.as_pubsub();
        pubsub.unsubscribe(channel)
    }
}
