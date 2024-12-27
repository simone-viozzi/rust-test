use axum::{
    extract::State,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

// Shared state type
type UserDb = Arc<Mutex<HashMap<u64, User>>>;

#[tokio::main]
async fn main() {
    // Initialize tracing for better debugging
    tracing_subscriber::fmt::init();

    // Create shared state
    let state: UserDb = Arc::new(Mutex::new(HashMap::new()));

    // Build the application with routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/list", get(list_users))
        .with_state(state);

    // Define the address for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Bind to the address and create a listener
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);

    // Serve the application using the listener
    axum::serve(listener, app).await.unwrap();
}

// Basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn list_users(
    State(state): State<UserDb>,
) -> (StatusCode, Json<Vec<User>>) {
    let db = state.lock().unwrap();
    let users = db.values().cloned().collect();
    (StatusCode::OK, Json(users))
}

// Handler for creating a user
async fn create_user(
    State(state): State<UserDb>, // Extract shared state
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // Lock the shared state to modify it
    let mut db = state.lock().unwrap();

    // Generate a new user ID
    let id = (db.len() as u64) + 1;

    // Create a new user and insert it into the database
    let user = User {
        id,
        username: payload.username,
    };
    db.insert(id, user.clone());

    // Respond with `201 Created` and the new user as JSON
    (StatusCode::CREATED, Json(user))
}

// The input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// The output to our `create_user` handler
#[derive(Serialize, Clone)]
struct User {
    id: u64,
    username: String,
}
