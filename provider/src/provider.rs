use thiserror::Error;

pub trait Provider {
    type Key;
}

#[derive(Clone, Debug, Error)]
pub enum ProviderError {}
