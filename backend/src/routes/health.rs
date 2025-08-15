// TODO: Learn and implement health check best practices
// use crate::AppState;
// use axum::{extract::State, response::Json};
// use serde_json::{json, Value};
// use tracing::{info, instrument};
//
// #[instrument(skip(state))]
// pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
//     info!("Health check requested");
//
//     Json(json!({
//         "status": "healthy",
//         "service": "axum-tracing-example",
//         "timestamp": chrono::Utc::now().to_rfc3339(),
//         "metrics": {
//             "user_count": user_count
//         }
//     }))
// }
