#[derive(Clone, Debug)]
pub enum StoreAction {}

#[derive(Clone, Debug)]
pub struct StoreActionProcessingOutput {
    pub rerender_needed: bool,
}

/// To start with, we're going to keep this fairly monolithic and not worry about decoupling. We're going to try to keep this fairly opaque
/// and keep the interface stable though.
#[derive(Debug)]
pub struct Stores {}

impl Default for Stores {
    fn default() -> Self {
        Self::new()
    }
}

impl Stores {
    pub fn new() -> Self {
        todo!()
    }

    pub fn process_store_action(action: StoreAction) -> StoreActionProcessingOutput {
        todo!()
    }
}
