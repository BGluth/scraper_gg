use reporter_state::stores::store_action_processing::{StoreAction, Stores};
use tokio::sync::mpsc::{channel, Receiver, Sender};

const ACTION_BUF_SIZE: usize = 1000;

#[derive(Debug)]
pub(crate) enum TuiAction {
    Store(StoreAction),

    /// Not sure if this will be used often in practice, but I'll add this in for now as a way to force the TUI re-render.
    ForceRefresh,
}

pub(crate) struct ProgState {
    stores: Stores,

    msg_rx: Receiver<TuiAction>,
}

impl ProgState {
    pub(crate) fn init() -> (Self, Sender<TuiAction>) {
        let (tx, rx) = channel(ACTION_BUF_SIZE);

        let state = Self {
            stores: Stores::new(),
            msg_rx: rx,
        };

        (state, tx)
    }

    pub(crate) async fn process_messages(mut self) {
        while let Some(msg) = self.msg_rx.recv().await {
            match msg {
                TuiAction::Store(_) => todo!(),
                TuiAction::ForceRefresh => todo!(),
            }
        }
    }
}
