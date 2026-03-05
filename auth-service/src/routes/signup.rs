use axum::{
    extract::{self, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
};

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

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // return AuthAPIError::InvalidCredentials if:
    // email is empty
    if email.is_empty() {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // email does not contain '@'
    if !email.contains("@") {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // password is less than 8 characters
    if password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);
    let mut user_store = state.user_store.write().await;

    // return AuthAPIError::UserAlreadyExists if email exists in user_store.
    if user_store.get_user(&user.email).is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    // return AuthAPIError::UnexpectedError if add_user() fails.
    if user_store.add_user(user).is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}
