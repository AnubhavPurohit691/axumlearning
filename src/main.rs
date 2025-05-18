use std::sync::Arc;
use axum::{response::IntoResponse, routing::get, serve::Listener, Json, Router};
use tokio::net::TcpListener;
use sqlx::{postgres::PgPoolOptions,Pool,Postgres};
use dotenv::dotenv;

pub struct AppState{
    db:Pool<Postgres>
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url=std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await
                {
                    Ok(pool)=>{
                        println!("Connection to the database is successful!");
                        pool
                    }
                    Err(err)=>{
                        println!("failed to connect to the database :{:?}",err);
                        std::process::exit(1)
                    }
                };
    
    let app_state = Arc::new(AppState{db:pool.clone()});

    let app= Router::new().route("/api/healthcheck", get(heathcheckhandler));
    println!("server started successfully at 0.0.0.0:8080");
    let listener=TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener,app.into_make_service()).await.unwrap();
}





pub async fn heathcheckhandler()->impl IntoResponse{
    const MESSAGE:&str="API Service";
    let json_response=serde_json::json!({
        "status":"ok",
        "message":MESSAGE
    });
    Json(json_response)
}
