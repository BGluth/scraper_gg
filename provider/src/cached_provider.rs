use std::time::Duration;

use bytesize::ByteSize;
use serde::Deserialize;
use thiserror::Error;

use crate::provider::{Provider, ProviderError};

const DEFAULT_MEM_CACHE_SIZE: ByteSize = ByteSize::gb(2);
const DEFAULT_DISK_CACHE_SIZE: ByteSize = ByteSize::gb(50);

pub trait CacheableProvider: Provider {
    /// This is pretty unstable and may change.
    ///
    /// The idea is all providers may store whatever data they want. However, we want to somewhat unify the way it's stored. For now, we're
    /// going to say that a piece of data can be retrieved by the data type and the id.
    fn get_cache_key(&self, k: &Self::Key) -> CacheKey;
}

type CacheableProviderResult<T> = Result<T, CacheableProviderError>;

#[derive(Clone, Debug, Error)]
pub enum CacheableProviderError {
    #[error(transparent)]
    Cache(#[from] CacheError),

    #[error(transparent)]
    Provider(#[from] ProviderError),
}

#[derive(Clone, Debug, Error)]
pub enum CacheError {}

/// A wrapper around a provider that always attempts to check the cache before the provider.
///
/// The provider is generally going to be an upstream, but this is not guaranteed.
#[derive(Debug)]
pub struct CachedProviderBuilder<P: CacheableProvider + Provider> {
    remote_p: P,
    max_mem_cache_size: ByteSize,
    max_disk_cache_size: ByteSize,
}

impl<T: CacheableProvider + Provider> CachedProviderBuilder<T> {
    pub fn new(remote_p: T) -> Self {
        Self {
            remote_p,
            max_disk_cache_size: DEFAULT_MEM_CACHE_SIZE,
            max_mem_cache_size: DEFAULT_DISK_CACHE_SIZE,
        }
    }

    pub fn max_mem_cache_size(mut self, max_mem_cache_size: ByteSize) -> Self {
        self.max_mem_cache_size = max_mem_cache_size;
        self
    }

    pub fn max_disk_cache_size(mut self, max_disk_cache_size: ByteSize) -> Self {
        self.max_disk_cache_size = max_disk_cache_size;
        self
    }

    pub fn build(self) -> CachedProvider<T> {
        todo!()
    }
}

impl<T: CacheableProvider + Provider> From<CachedProviderBuilder<T>> for CachedProvider<T> {
    fn from(v: CachedProviderBuilder<T>) -> Self {
        v.build()
    }
}

#[derive(Clone, Debug)]
pub struct CachedProvider<P> {
    remote_p: P,

    /// The amount of time until a record is considered stale.
    stale_time: Duration,
}

impl<P: CacheableProvider + Provider> CachedProvider<P> {
    /// Attempts to get/fetch a value for a key.
    ///
    /// Always checks the cache first, and otherwise fetches from the provider. Returns `None` if the values does not exist upstream.
    fn get<'a, V: Deserialize<'a>>(&'a self, k: P::Key) -> CacheableProviderResult<Option<V>> {
        todo!()
    }
}

// TODO: Remove the use of `String`s...
#[derive(Clone, Debug)]
pub struct CacheKey {
    d_type: String,
    id: String,
}
