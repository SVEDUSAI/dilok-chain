use axum::{
    routing::{get, post},
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
};
use crate::core::types::{UserInfo, FaceData, VerificationResult};
use crate::DilokDid;

pub fn create_router(did_system: DilokDid) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/verify/:user_id", post(verify_user))
        .route("/user/:user_id", get(get_user_info))
        .with_state(did_system)
}

async fn register_user(
    State(did_system): State<DilokDid>,
    Json(user_info): Json<UserInfo>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement user registration
    Ok(StatusCode::OK)
}

async fn verify_user(
    State(did_system): State<DilokDid>,
    Path(user_id): Path<String>,
) -> Result<Json<VerificationResult>, StatusCode> {
    // TODO: Implement user verification
    Ok(Json(VerificationResult {
        is_verified: true,
        confidence_score: 0.95,
        timestamp: chrono::Utc::now(),
    }))
}

async fn get_user_info(
    State(did_system): State<DilokDid>,
    Path(user_id): Path<String>,
) -> Result<Json<UserInfo>, StatusCode> {
    // TODO: Implement user info retrieval
    Err(StatusCode::NOT_IMPLEMENTED)
} 