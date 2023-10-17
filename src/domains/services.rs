use crate::consts::DbPool;

use super::auth::services::auth_service::AuthService;
use super::chat_room::services::chat_room_service::ChatRoomService;
use super::chat_room::services::chat_room_user_service::ChatRoomUserService;
use super::message::services::message_service::MessageService;
use super::user::services::user_service::UserService;

pub struct AppStateServices {
    pub pool: DbPool,
}

impl AppStateServices {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn user_service(&self) -> UserService {
        UserService::new(self.pool.clone())
    }

    pub fn auth_service(&self) -> AuthService {
        AuthService::new(self.pool.clone())
    }
    pub fn chat_room_service(&self) -> ChatRoomService {
        ChatRoomService::new(self.pool.clone())
    }

    pub fn chat_room_user_service(&self) -> ChatRoomUserService {
        ChatRoomUserService::new(self.pool.clone())
    }

    pub fn message_service(&self) -> MessageService {
        MessageService::new(self.pool.clone())
    }
}
