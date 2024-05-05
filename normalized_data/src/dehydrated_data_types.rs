use serde::{Deserialize, Serialize};

pub trait Hydratable {
    type Hydrated;

    fn hydrate(self) -> Self::Hydrated;
}

pub trait NormalizableData {
    type NormalizedData;

    fn normalize(&self) -> Self::NormalizedData;
}

pub trait HydratableNormalized {
    type NormalizedData;

    fn hydrate_to_normalized(self) -> Self::NormalizedData;
}

impl<T, H> HydratableNormalized for T
where
    T: Hydratable<Hydrated = H>,
    H: NormalizableData,
{
    type NormalizedData = H::NormalizedData;

    fn hydrate_to_normalized(self) -> Self::NormalizedData {
        self.hydrate().normalize()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedTournament<I> {
    t_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedBracket<I> {
    b_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedSet<I> {
    s_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedGame<I> {
    g_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedPlayerGameInfo<I> {
    pg_id: I,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DehydratedPlayer<I> {
    p_id: I,
}
