//! Monitoring and managing of cranks. Mainly used for periodic cleanup of
//! finished cranks and state persistence.

use std::{fs, sync::Arc};

use tokio::{
    sync::Mutex,
    task::JoinHandle,
    time::{sleep, Duration},
};

use crate::state::State;

pub struct Monitor(Arc<Mutex<State>>);

impl Monitor {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self(state)
    }

    pub async fn init(mut self) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            loop {
                sleep(Duration::from_secs(10)).await;
                self.cleanup_finished().await;
                self.persist_state().await;
            }
        })
    }

    async fn cleanup_finished(&mut self) {
        let mut state = self.0.lock().await;

        let finished_cranks = state
            .cranks
            .iter()
            .filter_map(|(market_id, crank)| {
                let start_time = crank.start_time?;
                let should_run_for = crank.should_run_for?;
                if start_time.elapsed().as_secs() / 60 >= should_run_for {
                    Some(market_id.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        for market_id in finished_cranks {
            tracing::info!(
                "cranking for market {market_id} finished, cleaning up..."
            );
            if let Some(crank) = state.cranks.get_mut(&market_id) {
                if let Err(e) = crank.halt() {
                    tracing::error!(
                        "cannot halt crank for market {market_id}: {e}"
                    );
                };
            } else {
                tracing::error!("cannot find crank for market {market_id}");
            };
            let _ = state.cranks.remove(&market_id);
        }
    }

    async fn persist_state(&mut self) {
        let state = self.0.lock().await;

        tracing::trace!("persisting full state:\n{state:?}");

        if let Err(e) = fs::create_dir_all(&state.params.persist) {
            tracing::error!("could not create persistence folder: {e}");
            return;
        }

        let json_path = state.params.persist.join("state.json");
        let json_string = match serde_json::to_string_pretty(&*state) {
            Ok(json) => json,
            Err(e) => {
                tracing::error!("could not serialize state: {e}");
                return;
            }
        };

        if let Err(e) = std::fs::write(&json_path, json_string) {
            tracing::error!("could not write to file {json_path:?}: {e}");
        }
    }
}
