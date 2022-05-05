use actix_cors::Cors;
use actix_web::{
    get, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::{io, sync::Arc};

mod models;
mod schema;

use crate::schema::{create_schema, Schema};

use once_cell::sync::OnceCell;

static DATABASE: OnceCell<mongodb::Database> = OnceCell::new();
static REDIS: OnceCell<redis::Client> = OnceCell::new();

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let result = data.execute(&st, &()).await;
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let schema = Arc::new(create_schema());

    let client: mongodb::Client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    DATABASE.set(client.database("aisuru")).unwrap();

    let redis = redis::Client::open("redis://localhost:6379").unwrap();
    REDIS.set(redis).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
