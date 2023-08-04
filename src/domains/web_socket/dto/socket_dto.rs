use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]

pub struct WebSocketQuery {
    pub user_id: Uuid,
}
