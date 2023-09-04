mod model;
mod schema;
mod handler;
mod route;

use std::sync::Arc;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use route::create_router;
use tower_http::cors::CorsLayer;

pub struct AppState {
	db: MySqlPool,
}

use axum::http::{
	header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
	HeaderValue, Method,
};

#[tokio::main]
async fn main() {
	dotenv().ok();

	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let pool = match MySqlPoolOptions::new()
		.max_connections(10)
		.connect(&database_url)
		.await
	{
		Ok(pool) => {
			println!("✅Connection to the database is successful!");
			pool
		},
		Err(err) => {
			println!("🔥 Failed to connect to the database: {:?}", err);
			std::process::exit(1);
		}
	};

	let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

	println!("🚀 Server started successfully");

	axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
	.serve(app.into_make_service())
	.await
	.unwrap();
}
