use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = match jar.get(&JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    // TODO: Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken is validation fails.
    let token = cookie.value().to_owned();
    if validate_token(&token).await.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    (jar, Ok(StatusCode::OK))
}
