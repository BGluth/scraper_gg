//! Core data types that are needed to re-create all of the data scraped. Note that there are still other "intermediate" datatypes that are
//! created (and cached?) for ease-of-use.

use serde::{Deserialize, Serialize};

pub type CoreBracketId = u64;
pub type CoreSetId = u64;
pub type CoreGameId = u64;
pub type CorePlayerId = u64;

// TODO: No idea how to represent this data. This needs to be game agnostic, so
// this might be a bit tricky...
pub type PlayerGameMetaInfo = u64;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tournament {
    name: String,
    brackets: Vec<CoreBracketId>,
    admins: Vec<AdminAndPrivilegeLevel>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminAndPrivilegeLevel {
    p_id: CorePlayerId,
    p_level: AdminPrivilegeLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AdminPrivilegeLevel {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bracket {
    id: CoreBracketId,
    b_type: BracketType,
    sets: Vec<CoreSetId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum BracketType {
    DoubleElim,
    RoundRobin,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Set {
    games: Vec<CoreGameId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    id: CoreGameId,
    g_type: GameType,
    winning_side: GameWinningSide,
}

/// Kind of a weird way to do this, but we need a bit to indicate which side one
/// in a game. Might need to make this more complicated down the road for when
/// we support more games.
#[derive(Debug, Deserialize, Serialize)]
pub enum GameWinningSide {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GameType {
    OneVOne(PlayerGameInfo, PlayerGameInfo),
    MultiVsMulti(Vec<PlayerGameInfo>, Vec<PlayerGameInfo>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerGameInfo {
    p_id: CorePlayerId,
    meta: PlayerGameMetaInfo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    name: String,
    prefix: String,
}
