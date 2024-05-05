use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament,
    },
    hydrated_data_types::{HydratedBracket, HydratedGame, HydratedPlayer, HydratedPlayerGameInfo, HydratedSet, HydratedTournament},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Tournament<I> {
    Dehydrated(DehydratedTournament<I>),
    Hydrated(HydratedTournament<I>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Bracket<I> {
    Dehydrated(DehydratedBracket<I>),
    Hydrated(HydratedBracket<I>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Set<I> {
    Dehydrated(DehydratedSet<I>),
    Hydrated(HydratedSet<I>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Game<I> {
    Dehydrated(DehydratedGame<I>),
    Hydrated(HydratedGame<I>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlayerGameInfo<I> {
    Dehydrated(DehydratedPlayerGameInfo<I>),
    Hydrated(HydratedPlayerGameInfo<I>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Player<I> {
    Dehydrated(DehydratedPlayer<I>),
    Hydrated(HydratedPlayer<I>),
}
