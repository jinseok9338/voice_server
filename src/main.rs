mod domains;

use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use tokio_postgres::{NoTls, Error};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn connect_to_db() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost port=5432 user=postgres password=password", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(())
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };
    match connect_to_db().await {
        Ok(_) => println!("Connected to database!"),
        Err(e) => println!("Error connecting to database: {}", e),
    }


    Ok(config.into())
}
