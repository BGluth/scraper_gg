//! Core data types that are needed to re-create all of the data scraped. Note that there are still other "intermediate" datatypes that are
//! created (and cached?) for ease-of-use.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::data_types::{Bracket, Game, PlayerGameInfo, Set};

// TODO: No idea how to represent this data. This needs to be game agnostic, so
// this might be a bit tricky...
pub type PlayerGameMetaInfo = u64;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedTournament<I> {
    t_id: I,
    name: String,
    brackets: Vec<Bracket<I>>,
    admins: Vec<AdminAndPrivilegeLevel<I>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAndPrivilegeLevel<I> {
    p_id: I,
    p_level: AdminPrivilegeLevel,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AdminPrivilegeLevel {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedBracket<I> {
    b_id: I,
    b_type: BracketType,
    sets: Vec<Set<I>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BracketType {
    DoubleElim,
    RoundRobin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedSet<I> {
    s_id: I,
    games: Vec<Game<I>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedGame<I> {
    g_id: I,
    g_type: GameType<I>,
    winning_side: GameWinningSide,
}

/// Kind of a weird way to do this, but we need a bit to indicate which side one
/// in a game. Might need to make this more complicated down the road for when
/// we support more games.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameWinningSide {
    Left = 0,
    Right = 1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameType<I> {
    OneVOne(PlayerGameInfo<I>, PlayerGameInfo<I>),
    MultiVsMulti(Vec<PlayerGameInfo<I>>, Vec<PlayerGameInfo<I>>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedPlayerGameInfo<I> {
    p_id: I,
    meta: PlayerGameMetaInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedPlayer<I> {
    p_id: I,
    name: String,
    prefix: String,
}
