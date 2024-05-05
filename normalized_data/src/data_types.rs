use std::fmt::Debug;

use paste::paste;
use serde::{Deserialize, Serialize};

use crate::{
    dehydrated_data_types::{
        DehydratedBracket, DehydratedGame, DehydratedPlayer, DehydratedPlayerGameInfo, DehydratedSet, DehydratedTournament,
    },
    hydrated_data_types::{HydratedBracket, HydratedGame, HydratedPlayer, HydratedPlayerGameInfo, HydratedSet, HydratedTournament},
};

macro_rules! define_hydrated_dehydrated_enum {
    ($type_name:ident) => {
        paste! {
            #[derive(Clone, Debug, Deserialize, Serialize)]
            pub enum [<$type_name>]<I> {
                Dehydrated([<Dehydrated$type_name>]<I>),
                Hydrated([<Hydrated$type_name>]<I>),
            }
        }
    };
}

define_hydrated_dehydrated_enum!(Tournament);
define_hydrated_dehydrated_enum!(Bracket);
define_hydrated_dehydrated_enum!(Set);
define_hydrated_dehydrated_enum!(Game);
define_hydrated_dehydrated_enum!(PlayerGameInfo);
define_hydrated_dehydrated_enum!(Player);
