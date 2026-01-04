use axum;
use axum::extract::State;
use axum::{Router, routing::get};
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

async fn pool_connection() -> Pool<Postgres> {
    Pool::<Postgres>::connect("postgresql://luka@localhost:5432/MyBookStore")
        .await
        .unwrap()
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
