//! Core data types that are needed to re-create all of the data scraped. Note that there are still other "intermediate" datatypes that are
//! created (and cached?) for ease-of-use.

use serde::{Deserialize, Serialize};

pub type CoreTournamentId = u64;
pub type CoreBracketId = u64;
pub type CoreSetId = u64;
pub type CoreGameId = u64;
pub type CorePlayerId = u64;

// TODO: No idea how to represent this data. This needs to be game agnostic, so
// this might be a bit tricky...
pub type PlayerGameMetaInfo = u64;

#[derive(Debug, Deserialize, Serialize)]
pub struct HydratedTournament {
    t_id: CoreTournamentId,
    name: String,
    brackets: Vec<HydratedBracket>,
    admins: Vec<AdminAndPrivilegeLevel>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAndPrivilegeLevel {
    p_id: CorePlayerId,
    p_level: AdminPrivilegeLevel,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AdminPrivilegeLevel {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedBracket {
    b_id: CoreBracketId,
    b_type: BracketType,
    sets: Vec<HydratedSet>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BracketType {
    DoubleElim,
    RoundRobin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedSet {
    games: Vec<HydratedGame>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedGame {
    g_id: CoreGameId,
    g_type: GameType,
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
pub enum GameType {
    OneVOne(HydratedPlayerGameInfo, HydratedPlayerGameInfo),
    MultiVsMulti(Vec<HydratedPlayerGameInfo>, Vec<HydratedPlayerGameInfo>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedPlayerGameInfo {
    p_id: CorePlayerId,
    meta: PlayerGameMetaInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedPlayer {
    p_id: CorePlayerId,
    name: String,
    prefix: String,
}
