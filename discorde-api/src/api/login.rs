use crate::api::DiscordeState;
use crate::models::creds::{Credentials, Login};
use crate::models::user::UserInput;
use axum::body::Body;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use libc::user;
use std::sync::Arc;
use tracing::{error, info};

#[axum::debug_handler]
async fn login(State(state): State<Arc<DiscordeState>>, Json(user): Json<Login>) -> Response<Body> {
    let db_user = match state.db.get_user(user.username.clone()).await {
        Ok(Some(user)) => user,
        Err(error) => {
            error!(?error);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        Ok(None) => return StatusCode::BAD_REQUEST.into_response(),
    };

    if db_user.password == user.password {
        Json(Credentials {
            token: db_user.username.clone(),
            user: db_user.into_view(),
        })
        .into_response()
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}

pub fn routes() -> Router<Arc<DiscordeState>> {
    Router::new().route("/", post(login))
}
