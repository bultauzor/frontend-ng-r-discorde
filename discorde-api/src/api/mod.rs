use crate::chat::ChatSvc;
use crate::db::Database;
use axum::body::Body;
pub use axum::extract::{Request, State};
use axum::http::StatusCode;
pub use axum::middleware::Next;
pub use axum::response::{IntoResponse, Response};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::{AllowCredentials, Any};
use tracing::error;

mod chats;
mod login;
mod user;

pub struct DiscordeState {
    pub db: Arc<Database>,
    pub chat: ChatSvc,
}

async fn middleware(
    State(state): State<Arc<DiscordeState>>,
    mut request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let username = match request
        .headers()
        .get("Authorization")
        .and_then(|e| e.to_str().ok())
        .and_then(|authorization| {
            authorization
                .to_string()
                .strip_prefix("Bearer ")
                .map(ToString::to_string)
        }) {
        None => return StatusCode::UNAUTHORIZED.into_response(),
        Some(bearer) => bearer,
    };

    let user = match state.db.get_user(username).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(error) => {
            error!(?error);
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    request.extensions_mut().insert(user);

    next.run(request).await
}

pub fn routes(discorde_state: DiscordeState) -> Router {
    let discorde_state = Arc::new(discorde_state);
    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_origin(Any)  // Open access to selected route
        .allow_methods(Any)
        .allow_headers(Any);
    Router::new()
        .nest("/users", user::routes(discorde_state.clone()))
        .nest("/chats", chats::routes(discorde_state.clone()))
        .nest("/login", login::routes())
        .with_state(discorde_state)
        .layer(cors_layer)
}
