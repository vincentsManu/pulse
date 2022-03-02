use crate::user_session::health_data::get_health_data;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Headers, IntoResponse},
};

use sqlx::PgPool;

#[tracing::instrument(name = "get health data handler")]
pub async fn get_health_data_handler(
    Extension(connection_pool): Extension<PgPool>,
) -> impl IntoResponse {
    let hds = get_health_data(&connection_pool).await.unwrap();

    let mut wtr = csv::Writer::from_writer(vec![]);
    for hd in hds {
        wtr.serialize(hd).unwrap();
    }

    let ser = wtr.into_inner().unwrap();
    let res = String::from_utf8(ser).unwrap();

    let headers = Headers(vec![
        ("Content-Type", "text/csv; name=\"health_data.csv\""),
        ("Content-Disposition", "attachment; filename=\"health_data.csv\""),
    ]);

    (StatusCode::OK, headers, res)
}
