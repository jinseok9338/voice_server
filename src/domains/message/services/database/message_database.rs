use diesel::{PgConnection, RunQueryDsl};

use uuid::Uuid;

use crate::domains::message::dto::message_dto::{Message, NewMessage, Pagination, TotalElement};

pub fn read_all_by_chat_room_ids(
    conn: &mut PgConnection,
    chat_room_id: Uuid,
    page: i32,
    size: Option<i32>,
    sort_by: Option<(&str, &str)>,
) -> Pagination<Message> {
    let size = size.unwrap_or(20);
    let (sort_column, sort_direction) = sort_by.unwrap_or(("id", "desc"));

    let offset = page * size;

    let chat_room_id_str = chat_room_id.to_string();

    let sql = format!(
        "SELECT * FROM messages WHERE chat_room_id = $1::uuid ORDER BY {} {} LIMIT $2 OFFSET $3",
        sort_column, sort_direction
    );

    let messages_query = diesel::sql_query(sql)
        .bind::<diesel::sql_types::Text, _>(&chat_room_id_str)
        .bind::<diesel::sql_types::Integer, _>(size)
        .bind::<diesel::sql_types::Integer, _>(offset);
    let messages: Vec<Message> = messages_query.load(conn).expect("Error loading messages");

    let sql = "SELECT COUNT(*) as count FROM messages WHERE chat_room_id = $1::uuid";

    let total_elements_query =
        diesel::sql_query(sql).bind::<diesel::sql_types::Text, _>(&chat_room_id_str);

    let total_elements: Vec<TotalElement> = total_elements_query
        .load(conn)
        .expect("Error loading total elements count")
        .into_iter()
        .collect();
    let total_elements = total_elements[0].count;

    let total_pages = (total_elements as f64 / size as f64).ceil() as i32;

    Pagination {
        content: messages,
        total_elements: total_elements as i32,
        total_pages,
        page,
        size,
        has_next: page < total_pages - 1,
        has_previous: page > 0,
    }
}

pub fn create_message_to_database(
    conn: &mut PgConnection,
    chat_room_id: Uuid,
    new_message: &NewMessage,
    user_id: Uuid,
) -> Message {
    let new_message = Message::new(
        chat_room_id,
        new_message.message.clone(),
        user_id,
    );
    let message = diesel::insert_into(crate::schema::messages::table)
        .values(&new_message)
        .get_result::<Message>(conn)
        .expect("Error saving new message");

    message
}
