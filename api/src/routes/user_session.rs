use crate::user_session::crud::{get_all_user_session, insert_user_session};
use crate::user_session::{errors::UserSessionError, UserSession};

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

#[tracing::instrument(name = "create user session handler")]
pub async fn create_user_session_handler(
    Json(user_session): Json<UserSession>,
    Extension(connection_pool_writable): Extension<PgPool>,
) -> eyre::Result<impl IntoResponse, UserSessionError> {
    let us = &user_session;

    if us.has_consented && us.has_accepted_terms_conditions {
        if us.health_data.is_none() {
            return Ok((StatusCode::BAD_REQUEST, "user consented but no health data was supplied"));
        }
        if us.user_interactions.is_none() {
            return Ok((
                StatusCode::BAD_REQUEST,
                "user consented but no user interaction was supplied",
            ));
        }
    }

    if !us.has_consented || !us.has_accepted_terms_conditions {
        if us.health_data.is_some() {
            return Ok((
                StatusCode::BAD_REQUEST,
                "user did not consent or accept t&c but health data was provided",
            ));
        }
        if us.user_interactions.is_some() {
            return Ok((
                StatusCode::BAD_REQUEST,
                "user did not consent or accept t&c but user interactions data was provided",
            ));
        }
    }

    insert_user_session(&connection_pool_writable, us).await?;

    Ok((StatusCode::CREATED, ""))
}

#[tracing::instrument(name = "list user sessions handler")]
pub async fn list_user_session_handler(
    Extension(connection_pool): Extension<PgPool>,
) -> eyre::Result<impl IntoResponse, UserSessionError> {
    let uss: Vec<UserSession> = get_all_user_session(&connection_pool).await?;

    Ok((StatusCode::OK, Json(uss)))
}
