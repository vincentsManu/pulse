use crate::stats::updates::StatsActorMessage;
use crate::stats::Stats;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    response::IntoResponse,
    Json,
};

use tokio::sync::{mpsc, oneshot, watch};
use tokio_stream::wrappers::WatchStream;
use tokio_stream::{Stream, StreamExt as _};

use std::{collections::HashMap, convert::Infallible, time::Duration};

#[tracing::instrument(name = "get stats handler")]
pub async fn get_stats_handler(
    Extension(tx_get_stats): Extension<mpsc::Sender<StatsActorMessage>>,
) -> impl IntoResponse {
    let (os_send, os_receive) = oneshot::channel::<HashMap<String, Stats>>();
    tx_get_stats.send(StatsActorMessage::GetStats { respond_to: os_send }).await.unwrap();

    let stats = os_receive.await.unwrap();
    (StatusCode::OK, Json(stats))
}

#[tracing::instrument(name = "get stats sse", skip(watch_stats))]
pub async fn get_stats_sse_handler(
    Extension(watch_stats): Extension<watch::Receiver<HashMap<String, Stats>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = WatchStream::new(watch_stats)
        .map(|stats| Ok(Event::default().data(serde_json::to_string(&stats).unwrap())));

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(20)))
}
