use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn verifytoken() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
