use crate::{
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament, Hydratable,
    },
    normalized_hydrated_data_types::{
        HydratedBracket, HydratedGame, HydratedPlayer, HydratedPlayerGameInfo, HydratedSet, HydratedTournament,
    },
};

pub type NormalizedId = u64;

pub type NormalizedHydratedTournament = HydratedTournament<NormalizedId>;
pub type NormalizedHydratedBracket = HydratedBracket<NormalizedId>;
pub type NormalizedHydratedSet = HydratedSet<NormalizedId>;
pub type NormalizedHydratedGame = HydratedGame<NormalizedId>;
pub type NormalizedHydratedPlayerGameInfo = HydratedPlayerGameInfo<NormalizedId>;
pub type NormalizedHydratedPlayer = HydratedPlayer<NormalizedId>;

pub type NormalizedDehydratedTournament = DehydratedTournament<NormalizedId>;
pub type NormalizedDehydratedBracket = DehydratedBracket<NormalizedId>;
pub type NormalizedDehydratedSet = DehydratedSet<NormalizedId>;
pub type NormalizedDehydratedGame = DehydratedGame<NormalizedId>;
pub type NormalizedDehydratedPlayerGameInfo = DehydratedPlayerGameInfo<NormalizedId>;
pub type NormalizedDehydratedPlayer = DehydratedPlayer<NormalizedId>;

impl Hydratable for NormalizedDehydratedTournament {
    type Hydrated = NormalizedHydratedTournament;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for NormalizedDehydratedBracket {
    type Hydrated = NormalizedHydratedBracket;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for NormalizedDehydratedSet {
    type Hydrated = NormalizedHydratedSet;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for NormalizedDehydratedGame {
    type Hydrated = NormalizedHydratedGame;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for NormalizedDehydratedPlayerGameInfo {
    type Hydrated = NormalizedHydratedPlayerGameInfo;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}

impl Hydratable for NormalizedDehydratedPlayer {
    type Hydrated = NormalizedHydratedPlayer;

    fn hydrate(self) -> Self::Hydrated {
        todo!()
    }
}
