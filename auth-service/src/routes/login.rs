use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, UserStore},
};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Early return on invalid input
    let auth_email: Email = Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let auth_password: Password =
        Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // Now check if the user is acually present in the userstore
    let user_store = &state.user_store.read().await;

    // TODO: call `user_store.validate_user` and return `AuthAPIError::IncorrectCredentials` if valudation fails.
    if user_store.validate_user(&auth_email, &auth_password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    // TODO: call `user_store.get_user`. Return AuthAPIError::IncorrectCredentials if the operation fails.
    if user_store.get_user(auth_email.clone()).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let response = Json(LoginResponse {
        message: "User successfully authenticated!".to_string(),
    });

    Ok(StatusCode::OK.into_response())
}

// pub async fn signup(
//     State(state): State<AppState>,
//     Json(request): Json<SignupRequest>,
// ) -> Result<impl IntoResponse, AuthAPIError> {
//     let email =
//         Email::parse(request.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
//     let password =
//         Password::parse(request.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

//     let user = User::new(email, password, request.requires_2fa);
//     let mut user_store = state.user_store.write().await;

//     // return AuthAPIError::UserAlreadyExists if email exists in user_store.
//     if user_store.get_user(user.email.clone()).await.is_ok() {
//         return Err(AuthAPIError::UserAlreadyExists);
//     }

//     // return AuthAPIError::UnexpectedError if add_user() fails.
//     if user_store.add_user(user).await.is_err() {
//         return Err(AuthAPIError::UnexpectedError);
//     }

//     let response = Json(SignupResponse {
//         message: "User created successfully!".to_string(),
//     });

//     Ok((StatusCode::CREATED, response))
// }
