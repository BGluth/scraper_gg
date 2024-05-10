//! Core data types that are needed to re-create all of the data scraped. Note that there are still other "intermediate" datatypes that are
//! created (and cached?) for ease-of-use.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
    data_types::{Dehydrateable, HydratableType, NormalizedOrigin},
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament,
    },
    dehydrated_normalized_data::{
        DehydratedNormalizedBracket, DehydratedNormalizedGame, DehydratedNormalizedPlayerGameInfo, DehydratedNormalizedSet,
        DehydratedNormalizedTournament,
    },
};

pub type NormalizedId = u64;

pub type NormalizedTournament = HydratableType<DehydratedTournament<NormalizedId>, HydratedNormalizedTournament>;
pub type NormalizedBracket = HydratableType<DehydratedBracket<NormalizedId>, HydratedNormalizedBracket>;
pub type NormalizedSet = HydratableType<DehydratedSet<NormalizedId>, HydratedNormalizedSet>;
pub type NormalizedGame = HydratableType<DehydratedGame<NormalizedId>, HydratedNormalizedGame>;
pub type NormalizedPlayerGameInfo = HydratableType<DehydratedPlayerGameInfo<NormalizedId>, HydratedNormalizedPlayerGameInfo>;
pub type NormalizedPlayer = HydratableType<DehydratedPlayer<NormalizedId>, HydratedNormalizedPlayer>;

// TODO: No idea how to represent this data. This needs to be game agnostic, so
// this might be a bit tricky...
pub type PlayerGameMetaInfo = u64;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedNormalizedTournament {
    t_id: NormalizedId,
    name: String,
    brackets: Vec<NormalizedBracket>,
    admins: Vec<AdminAndPrivilegeLevel>,
}

impl Dehydrateable for HydratedNormalizedTournament {
    type Origin = NormalizedOrigin;
    type Dehydrated = DehydratedNormalizedTournament;

    fn dehydrate(&self) -> Self::Dehydrated {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAndPrivilegeLevel {
    p_id: NormalizedId,
    p_level: AdminPrivilegeLevel,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AdminPrivilegeLevel {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedNormalizedBracket {
    b_id: NormalizedId,
    b_type: BracketNormalizedType,
    sets: Vec<NormalizedSet>,
}

impl Dehydrateable for HydratedNormalizedBracket {
    type Origin = NormalizedOrigin;
    type Dehydrated = DehydratedNormalizedBracket;

    fn dehydrate(&self) -> Self::Dehydrated {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BracketNormalizedType {
    DoubleElim,
    RoundRobin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedNormalizedSet {
    s_id: NormalizedId,
    games: Vec<NormalizedGame>,
}

impl Dehydrateable for HydratedNormalizedSet {
    type Origin = NormalizedOrigin;
    type Dehydrated = DehydratedNormalizedSet;

    fn dehydrate(&self) -> Self::Dehydrated {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedNormalizedGame {
    g_id: NormalizedId,
    g_type: GameType,
    winning_side: GameWinningSide,
}

impl Dehydrateable for HydratedNormalizedGame {
    type Origin = NormalizedOrigin;
    type Dehydrated = DehydratedNormalizedGame;

    fn dehydrate(&self) -> Self::Dehydrated {
        todo!()
    }
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
    OneVOne(NormalizedPlayerGameInfo, NormalizedPlayerGameInfo),
    MultiVsMulti(Vec<NormalizedPlayerGameInfo>, Vec<NormalizedPlayerGameInfo>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedNormalizedPlayerGameInfo {
    p_id: NormalizedId,
    meta: PlayerGameMetaInfo,
}

impl Dehydrateable for HydratedNormalizedPlayerGameInfo {
    type Origin = NormalizedOrigin;
    type Dehydrated = DehydratedNormalizedPlayerGameInfo;

    fn dehydrate(&self) -> Self::Dehydrated {
        todo!()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HydratedNormalizedPlayer {
    p_id: NormalizedId,
    name: String,
    prefix: String,
}
