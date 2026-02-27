use std::collections::HashMap;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User, services::HashmapUserStore};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}

// Use Axum's state extractor to pass in AppState
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let user: User = User::new(request.email, request.password, request.requires_2fa);

    // Acquire a write lock on the shared user store
    let mut user_store = state.user_store.write().await;

    // Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    let _ = user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
}

// pub async fn signup(Json(request): Json<SignupRequest>) -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }
