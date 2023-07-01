first_mod_rs_content = """\
pub mod controllers;
pub mod dto;
pub mod module;
pub mod services;
"""


def controller_rs_content(name):
    return f"""\
use actix_web::{{delete, get, post, put, web, HttpResponse, Responder}};
use chrono::Utc;
use serde_json::json;
use crate::errors::base_error_messages::BaseError;


#[get("/{name}")]
async fn get_{name}s() -> impl Responder {{
    let mut conn = Db::connect_to_db();
    // Rest of the code
}}

#[post("/{name}")]
async fn create_{name}({name}: web::Json<{name.capitalize()}>) -> Result<impl Responder, BaseError> {{
    let mut conn = Db::connect_to_db();
    // Rest of the code
}}

#[get("/{name}/{{id}}")]
async fn get_{name}(path: web::Path<i32>) -> Result<impl Responder, BaseError> {{
    let mut conn = Db::connect_to_db();
    // Rest of the code
}}

#[put("/{name}/{{id}}")]
async fn update_{name}(path: web::Path<i32>, {name}: web::Json<{name.capitalize()}>) -> Result<impl Responder, BaseError> {{
    let mut conn = Db::connect_to_db();
    // Rest of the code
}}

#[delete("/{name}/{{id}}")]
async fn delete_{name}(path: web::Path<i32>) ->  Result<impl Responder, BaseError> {{
    // Rest of the code
}}
"""


def dto_rs_content(dto_name, data_base_name):
    return f"""\
use chrono::NaiveDateTime;
use diesel::{{Insertable, Queryable, AsChangeset}};
use serde::{{Deserialize, Serialize}};

use crate::schema::{data_base_name};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = {data_base_name})]
pub struct {dto_name} {{
    // Add fields here
}}
"""


def service_rs_content(name: str):
    return f"""\
use diesel::pg::PgConnection;

pub struct {name.capitalize()}Service<'a> {{
    pub conn: &'a mut PgConnection,
}}

impl<'a> {name.capitalize()}Service<'a> {{
    pub fn new(conn: &'a mut PgConnection) -> Self {{
        Self {{ conn }}
    }}

    pub fn create_{name}(&mut self, {name}: &{name.capitalize()}) -> {name.capitalize()} {{
        create(self.conn, {name})
    }}

    pub fn read_{name}s(&mut self) -> Vec<{name.capitalize()}> {{
        read(self.conn)
    }}

    pub fn read_one_{name}(&mut self, id: i32) -> Option<{name.capitalize()}> {{
        read_one(self.conn, id)
    }}

    pub fn update_{name}(&mut self, {name}: &{name.capitalize()}) -> {name.capitalize()} {{
        update_one(self.conn, {name}.id, {name})
    }}

    pub fn delete_{name}(&mut self, id: i32) -> usize {{
        delete_one(self.conn, id)
    }}
}}
"""


def database_rs_content(name: str):
    return f"""\
use diesel::prelude::*;
use diesel::PgConnection;

pub fn create(conn: &mut PgConnection, {name}: &{name.capitalize()}) -> {name.capitalize()} {{
    // Implement your create logic here
}}

pub fn read(conn: &mut PgConnection) -> Vec<{name.capitalize()}> {{
    // Implement your read logic here
}}

pub fn read_one(conn: &mut PgConnection, id: i32) -> Option<{name.capitalize()}> {{
    // Implement your read_one logic here
}}

pub fn update_one(conn: &mut PgConnection, id: i32, {name}: &{name.capitalize()}) -> {name.capitalize()} {{
    // Implement your update_one logic here
}}

pub fn delete_one(conn: &mut PgConnection, id: i32) -> usize {{
    // Implement your delete_one logic here
}}
"""
