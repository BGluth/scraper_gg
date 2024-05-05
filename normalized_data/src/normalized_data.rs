use crate::{
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament,
    },
    hydrated_data_types::{HydratedBracket, HydratedGame, HydratedPlayer, HydratedPlayerGameInfo, HydratedSet, HydratedTournament},
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
