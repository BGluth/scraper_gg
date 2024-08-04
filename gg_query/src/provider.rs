use provider::provider::Provider;

use crate::types::GGRestToken;

#[derive(Clone, Debug)]
pub struct GGProvider {
    token: GGRestToken,
}

impl GGProvider {
    pub fn new(gg_token: GGRestToken) -> Self {
        todo!()
    }
}

impl Provider for GGProvider {
    type Key = u64;
}
