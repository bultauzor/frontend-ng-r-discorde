use crate::api::DiscordeState;
use crate::models::user::{User, UserInput, UserView};
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{middleware, Json, Router};
use libc::user;
use std::fmt::Debug;
use std::io::Error;
use std::sync::Arc;
use tracing::{error, info};

#[axum::debug_handler]
async fn create_user(
    State(state): State<Arc<DiscordeState>>,
    Json(user): Json<UserInput>,
) -> StatusCode {
    info!(?user);

    match state.db.get_user(user.username.clone()).await {
        Ok(Some(_)) => return StatusCode::BAD_REQUEST,
        Err(error) => {
            error!(?error);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        Ok(None) => {}
    }

    match state.db.insert_user(user.into_user()).await {
        Ok(_) => StatusCode::CREATED,
        Err(error) => {
            error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[axum::debug_handler]
async fn get_users(State(state): State<Arc<DiscordeState>>) -> Json<Vec<UserView>> {
    Json(
        state
            .db
            .get_users()
            .await
            .into_iter()
            .map(|user| user.into_view())
            .collect(),
    )
}

#[axum::debug_handler]
async fn get_user(
    State(state): State<Arc<DiscordeState>>,
    Path(id): Path<String>,
) -> Response<Body> {
    match state.db.get_user(id).await {
        Ok(user) => match user {
            None => StatusCode::NOT_FOUND.into_response(),
            Some(user) => Json(user.into_view()).into_response(),
        },
        Err(error) => {
            error!(?error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn routes(state: Arc<DiscordeState>) -> Router<Arc<DiscordeState>> {
    Router::new()
        .merge(
            Router::new()
                .route("/", get(get_users))
                .route("/:id", get(get_user))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    super::middleware,
                )),
        )
        .route("/", post(create_user))
}
