use super::{
    stats::{StatsStore, StatsStoreAction},
    store_utils::Store,
};

#[derive(Clone, Debug)]
pub enum StoreAction {
    Stats(StatsStoreAction),
}

#[derive(Clone, Debug)]
pub struct StoreActionProcessingOutput {
    pub rerender_needed: bool,
}

/// To start with, we're going to keep this fairly monolithic and not worry about decoupling. We're going to try to keep this fairly opaque
/// and keep the interface stable though.
#[derive(Debug)]
pub struct Stores {
    stats: StatsStore,
}

impl Default for Stores {
    fn default() -> Self {
        Self::new()
    }
}

impl Stores {
    pub fn new() -> Self {
        todo!()
    }

    pub fn process_store_action(&mut self, action: StoreAction) -> StoreActionProcessingOutput {
        match action {
            StoreAction::Stats(a) => self.stats.update(a),
        };

        // For now, always assume that we need to redraw.
        StoreActionProcessingOutput { rerender_needed: true }
    }
}
