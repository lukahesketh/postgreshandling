use axum;
use axum::Json;
use axum::extract::State;
use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::postgres::Postgres;
use std::sync::Arc;
use tokio::net;

#[derive(sqlx::FromRow, Debug)]
struct BookData {
    title: String,
}

#[derive(Clone)]
struct AppState {
    db_pool: Pool<Postgres>,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub price: i32,
    pub description: Option<String>,
}

async fn pool_connection() -> Pool<Postgres> {
    Pool::<Postgres>::connect("postgresql://luka@localhost:5432/MyBookStore")
        .await
        .unwrap()
}

async fn send_post_book(
    State(pool_data): State<Arc<AppState>>,
    Json(bookpost): Json<Book>,
) -> String {
    println!("{}", bookpost.title);

    sqlx::query("INSERT INTO books (title, author, price, description) VALUES ($1, $2, $3, $4)")
        .bind(&bookpost.title);
}

async fn client_request_books(State(pool_data): State<Arc<AppState>>) -> String {
    println!("Quick test");

    let status: BookData = sqlx::query_as("SELECT title FROM books")
        .fetch_one(&pool_data.db_pool)
        .await
        .unwrap();

    println!("{}", status.title);
    status.title
}

fn main_client_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/clientpostbook", post(send_post_book))
        .with_state(state)
        .route("/clientrequestbooks", get(client_request_books))
        .with_state(state)
}
// 0.0.0.0:3069/clientrequestbooks
#[tokio::main]
async fn main() {
    let pool = pool_connection().await;

    let the_state = Arc::new(AppState { db_pool: pool });

    let app = main_client_router(the_state);

    let listener = net::TcpListener::bind("0.0.0.0:3069").await.unwrap();

    let server_addy = listener.local_addr().unwrap();

    println!("{}", server_addy);

    axum::serve(listener, app).await.unwrap();
}
