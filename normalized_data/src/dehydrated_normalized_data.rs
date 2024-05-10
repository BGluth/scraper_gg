use crate::{
    data_types::{Hydratable, NormalizedOrigin},
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament,
    },
    hydrated_normalized_data::{
        HydratedNormalizedBracket, HydratedNormalizedGame, HydratedNormalizedPlayer, HydratedNormalizedPlayerGameInfo,
        HydratedNormalizedSet, HydratedNormalizedTournament, NormalizedId,
    },
};

pub type DehydratedNormalizedTournament = DehydratedTournament<NormalizedId>;
pub type DehydratedNormalizedBracket = DehydratedBracket<NormalizedId>;
pub type DehydratedNormalizedSet = DehydratedSet<NormalizedId>;
pub type DehydratedNormalizedGame = DehydratedGame<NormalizedId>;
pub type DehydratedNormalizedPlayerGameInfo = DehydratedPlayerGameInfo<NormalizedId>;
pub type DehydratedNormalizedPlayer = DehydratedPlayer<NormalizedId>;

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
