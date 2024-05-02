use serde::{Deserialize, Serialize};

use crate::core_data_types::{
    AdminAndPrivilegeLevel, Bracket, BracketType, CoreBracketId, CoreGameId, Game, GameType, GameWinningSide, Set, Tournament,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedTournament {
    name: String,
    brackets: Vec<DehydratedBracket>,
    admins: Vec<AdminAndPrivilegeLevel>,
}

impl DehydratedTournament {
    pub fn hydrate(self) -> Tournament {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedBracket {
    id: CoreBracketId,
    b_type: BracketType,
    sets: Vec<DehydratedSet>,
}

impl DehydratedBracket {
    pub fn hydrate(self) -> Bracket {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedSet {
    games: Vec<DehydratedGame>,
}

impl DehydratedSet {
    pub fn hydrate(self) -> Set {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedGame {
    id: CoreGameId,
    g_type: GameType,
    winning_side: GameWinningSide,
}

impl DehydratedGame {
    pub fn hydrate(self) -> Game {
        todo!()
    }
}
