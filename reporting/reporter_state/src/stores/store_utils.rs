use std::{ops::Deref, sync::Arc};

use tokio::sync::RwLock;

#[derive(Debug)]
pub struct StoreData<T>(RwLock<Arc<T>>)
where
    T: Clone;

impl<T> From<T> for StoreData<T>
where
    T: Clone,
{
    fn from(v: T) -> Self {
        Self::new(v)
    }
}

impl<T> StoreData<T>
where
    T: Clone,
{
    pub fn new(v: T) -> Self {
        Self(RwLock::new(Arc::new(v)))
    }

    pub async fn create_ref(&self) -> StoreDataRef<T> {
        StoreDataRef(self.0.read().await.clone())
    }
}

/// Immutable copy of store data. Intended to be passed to views to render.
#[derive(Debug)]
pub struct StoreDataRef<T>(Arc<T>)
where
    T: Clone;

impl<T> Deref for StoreDataRef<T>
where
    T: Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Store {
    type Action;

    /// Update the store from an action.
    fn update(&mut self, action: Self::Action);
}
