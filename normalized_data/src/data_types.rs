use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub struct NormalizedOrigin();
impl DataOrigin for NormalizedOrigin {}

pub trait Hydratable {
    type Origin: DataOrigin;
    type Hydrated;

    fn hydrate(self) -> Self::Hydrated;
}

pub trait Dehydrateable {
    type Origin: DataOrigin;
    type Dehydrated;

    fn dehydrate(&self) -> Self::Dehydrated;
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

pub trait DataOrigin {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HydratableType<D, H>
where
    D: Hydratable<Hydrated = H> + Hydratable,
    H: Dehydrateable,
{
    Dehydrated(D),
    Hydrated(H),
}
