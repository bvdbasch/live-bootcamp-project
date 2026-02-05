use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn verify2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
