use std::collections::HashMap;

use normalized_data::hydrated_normalized_data::{HydratedNormalizedPlayer, NormalizedBracket, NormalizedPlayer};

use super::{stats::TourneyStoreAction, store_utils::Store};

pub type EventKey = String;
pub type LocalPlayerId = usize;

/// Store state 
#[derive(Clone, Debug)]
pub struct TourneyStore {
    tourney_name: String,
    brackets: HashMap<EventKey, NormalizedBracket>,
    registered_players: HashMap<LocalPlayerId, NormalizedPlayer>,
}

impl TourneyStore {
    // TODO: Pass in provider here?
    pub fn new() -> Self {
        Self {
            tourney_name: "Test Tourney".to_string(),
            brackets: HashMap::default(),
            registered_players: HashMap::default(),
        }
    }

    pub fn tourney_name<'a>(&'a self) -> &'a str {
        &self.tourney_name
    }

    pub fn get_total_players(&self) -> usize {
        self.registered_players.len()
    }
}

impl Store for TourneyStore {
    type Action = TourneyStoreAction;

    fn update(&mut self, action: Self::Action) {
        match action {}
    }
}