#[macro_use]
extern crate diesel;
extern crate dotenv;


use std::io;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{web,App,HttpRequest,HttpResponse,HttpServer,Error,Responder,middleware};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod db;
mod schemas;
// mod grqphal_schema;
mod schema;

use crate::db::establish_connection;
use crate::schemas::root::{create_schema,Schema,Ctx};
// use crate::grqphal_schema::{create_schema,Schema,Ctx};


async fn playground()-> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8082/graphql", None);
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<Ctx>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute_sync(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

async fn index(req:HttpRequest)-> impl Responder{
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!",&name)
}

#[actix_web::main]
async fn main()-> io::Result<()>{
    dotenv::dotenv().ok();
    std::env::set_var("Rust_LOG","actix_web=Info");
    env_logger::init();

    // DB Pool
    let pool = establish_connection();
    let schema_context = Ctx {db: pool.clone() };


    // Create Juniper Schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move||{
        App::new()
            .data(schema.clone())
            .data(schema_context.clone())
            .wrap(middleware::Logger::default())
            .wrap(Cors::new()
                    .allowed_methods(vec!["POST","GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish()
                )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(playground)))
            .route("/",web::get().to(index))
            .route("/{name}",web::get().to(index))
    })
    .bind(("127.0.0.1",8082))?
    .run()
    .await
}
