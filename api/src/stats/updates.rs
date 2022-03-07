use super::{get::get_all_stats_per_kiosk_location, Stats};
use log::{error, warn};
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot, watch};

pub async fn init(
    pool: &PgPool,
    pool_notify: &PgPool,
) -> eyre::Result<(mpsc::Sender<StatsActorMessage>, watch::Receiver<HashMap<String, Stats>>)> {
    let (tx, rx) = mpsc::channel::<StatsActorMessage>(10);

    // get stats
    let stats = get_all_stats_per_kiosk_location(pool).await?;

    // update the watcher when rx listen receive updates
    let (watch_sender, watch_receiver) = watch::channel(stats.clone());

    run_listen_stats_changes(pool, pool_notify, tx.clone()).await?;

    let sa = StatsActor::new(stats, rx, watch_sender).await;

    tokio::spawn(async move {
        run_my_stats_actor(sa).await;
    });

    Ok((tx, watch_receiver))
}

struct StatsActor {
    receiver: mpsc::Receiver<StatsActorMessage>,
    stats: HashMap<String, Stats>,
    watch_sender: watch::Sender<HashMap<String, Stats>>,
}

#[derive(Debug)]
pub enum StatsActorMessage {
    GetStats { respond_to: oneshot::Sender<HashMap<String, Stats>> },
    SetStats(HashMap<String, Stats>),
}

impl StatsActor {
    async fn new(
        stats: HashMap<String, Stats>,
        receiver: mpsc::Receiver<StatsActorMessage>,
        watch_sender: watch::Sender<HashMap<String, Stats>>,
    ) -> Self {
        Self { stats, receiver, watch_sender }
    }

    fn handle_message(&mut self, msg: StatsActorMessage) {
        match msg {
            StatsActorMessage::GetStats { respond_to } => {
                let _ = respond_to.send(self.stats.clone());
            }
            StatsActorMessage::SetStats(stats) => {
                self.stats = stats.clone();
                self.watch_sender.send(stats).unwrap();
            }
        }
    }
}

async fn run_my_stats_actor(mut actor: StatsActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

// use tokio::time::{sleep, Duration};

// async fn run_listen_stats_changes_fake(pool: &PgPool, sender: mpsc::Sender<StatsActorMessage>) {
//     let pool_clone = pool.clone();
//     tokio::spawn(async move {
//         loop {
//             sleep(Duration::from_secs(2)).await;
//             match get_db(&pool_clone).await {
//                 Err(e) => error!("{:?}", e),
//                 Ok(stats) => {
//                     println!("sending stats");
//                     if let Err(e) = sender.send(StatsActorMessage::SetStats(stats)).await {
//                         error!("error sending stats actor message with stats: {:?}", e);
//                     };
//                 }
//             }
//         }
//     });
// }

use sqlx::postgres::PgListener;
use tokio::time::{self, Duration};

async fn run_listen_stats_changes(
    pool: &PgPool,
    pool_notify: &PgPool,
    sender: mpsc::Sender<StatsActorMessage>,
) -> eyre::Result<()> {
    let pool_notify = pool_notify.clone();
    let mut listener = PgListener::connect_with(&pool_notify).await?;
    listener.listen("user_session_inserted").await?;

    let pool_request = pool.clone();
    let (tx_stats_throttler, mut rx_stats_throttler) = mpsc::channel::<()>(1);
    tokio::spawn(async move {
        while rx_stats_throttler.recv().await.is_some() {
            match get_all_stats_per_kiosk_location(&pool_request).await {
                Err(e) => error!("{:?}", e),
                Ok(stats) => {
                    if let Err(e) = sender.send(StatsActorMessage::SetStats(stats)).await {
                        error!("error sending stats actor message with stats: {:?}", e);
                    };
                }
            }
            time::sleep(Duration::from_secs(1)).await;
        }
    });

    tokio::spawn(async move {
        while listener.recv().await.is_ok() {
            // we don't care if it fails, that's the whole point of the throttler
            match tx_stats_throttler.send(()).await {
                Err(_) => warn!("could not send to stats refresh"),
                _ => (),
            }
        }
    });

    Ok(())
}
