use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};

use crate::{error::AppError, handlers::AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = headers.get("x-api-key").and_then(|x| x.to_str().ok());

    if let Some(auth_header) = auth_header
        && auth_header == state.api_key
    {
        return Ok(next.run(request).await);
    }

    Err(AppError::Unauthorized)
}
