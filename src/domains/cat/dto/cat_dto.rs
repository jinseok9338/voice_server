// this is where we define cat interface and model

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub breed: String,
    pub color: String,
    pub weight: f32,
    pub image: String,
    pub created_at: String,
    pub updated_at: String,
}

// basic CRUD interface to the data base

