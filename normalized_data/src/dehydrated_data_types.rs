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
    type NormalizableData;

    fn hydrate_to_normalized(self) -> Self::NormalizableData;
}

impl<T, N> HydratableNormalized for T
where
    T: Hydratable<Hydrated = N>,
    N: NormalizableData,
{
    type NormalizableData = N;

    fn hydrate_to_normalized(self) -> Self::NormalizableData {
        todo!()
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
