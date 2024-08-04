use crate::provider::Provider;

pub trait CacheableProvider {
    /// This is pretty unstable and may change.
    ///
    /// The idea is all providers may store whatever data they want. However, we want to somewhat unify the way it's stored. For now, we're
    /// going to say that a piece of data can be retrieved by the data type and the id.
    fn get_cache_key(&self, d_type: &str, id: &str) -> CacheKey;
}

/// A wrapper around a provider that always attempts to check the cache before the provider.
///
/// The provider is generally going to be an upstream, but this is not guaranteed.
#[derive(Debug)]
pub struct CachedProviderBuilder<P: CacheableProvider + Provider> {
    remote_p: P,
}

impl<T: CacheableProvider + Provider> CachedProviderBuilder<T> {
    pub fn new(remote_p: T) -> Self {
        todo!()
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
pub struct CachedProvider<T: CacheableProvider> {
    remote_p: T,
}

#[derive(Clone, Debug)]
pub struct CacheKey {}
