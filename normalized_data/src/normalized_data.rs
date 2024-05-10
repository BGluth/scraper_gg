use crate::{
    data_types::{DataOrigin, Hydratable, HydratableType},
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament,
    },
    normalized_hydrated_data_types::{
        HydratedNormalizedBracket, HydratedNormalizedGame, HydratedNormalizedPlayer, HydratedNormalizedPlayerGameInfo,
        HydratedNormalizedSet, HydratedNormalizedTournament,
    },
};

pub type NormalizedId = u64;

pub type NormalizedTournament = HydratableType<DehydratedTournament<NormalizedId>, HydratedNormalizedTournament>;
pub type NormalizedBracket = HydratableType<DehydratedBracket<NormalizedId>, HydratedNormalizedBracket>;
pub type NormalizedSet = HydratableType<DehydratedSet<NormalizedId>, HydratedNormalizedSet>;
pub type NormalizedGame = HydratableType<DehydratedGame<NormalizedId>, HydratedNormalizedGame>;
pub type NormalizedPlayerGameInfo = HydratableType<DehydratedPlayerGameInfo<NormalizedId>, HydratedNormalizedPlayerGameInfo>;
pub type NormalizedPlayer = HydratableType<DehydratedPlayer<NormalizedId>, HydratedNormalizedPlayer>;

pub type DehydratedNormalizedTournament = DehydratedTournament<NormalizedId>;
pub type DehydratedNormalizedBracket = DehydratedBracket<NormalizedId>;
pub type DehydratedNormalizedSet = DehydratedSet<NormalizedId>;
pub type DehydratedNormalizedGame = DehydratedGame<NormalizedId>;
pub type DehydratedNormalizedPlayerGameInfo = DehydratedPlayerGameInfo<NormalizedId>;
pub type DehydratedNormalizedPlayer = DehydratedPlayer<NormalizedId>;

pub struct NormalizedOrigin();
impl DataOrigin for NormalizedOrigin {}

impl Hydratable for DehydratedNormalizedTournament {
    type Origin = NormalizedOrigin;
    type Hydrated = HydratedNormalizedTournament;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for DehydratedNormalizedBracket {
    type Origin = NormalizedOrigin;
    type Hydrated = HydratedNormalizedBracket;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for DehydratedNormalizedSet {
    type Origin = NormalizedOrigin;
    type Hydrated = HydratedNormalizedSet;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for DehydratedNormalizedGame {
    type Origin = NormalizedOrigin;
    type Hydrated = HydratedNormalizedGame;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for DehydratedNormalizedPlayerGameInfo {
    type Origin = NormalizedOrigin;
    type Hydrated = HydratedNormalizedPlayerGameInfo;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for DehydratedNormalizedPlayer {
    type Origin = NormalizedOrigin;
    type Hydrated = HydratedNormalizedPlayer;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}
