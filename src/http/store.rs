use memory_cache::MemoryCache;

use crate::http::response::ApiResponse;
use std::{sync::RwLock, time};

pub(crate) trait InMemStore {
    fn get(&self, id: &str) -> Option<ApiResponse>;
    fn contains_key(&self, id: &str) -> bool;
    fn insert(
        &self,
        id: String,
        val: ApiResponse,
        lifetime: Option<time::Duration>,
    ) -> Option<ApiResponse>;
}

pub(crate) struct Store(RwLock<MemoryCache<String, ApiResponse>>);

impl Store {
    pub fn new(scan_interval: time::Duration) -> Self {
        Self(RwLock::new(MemoryCache::with_full_scan(scan_interval)))
    }
}

impl InMemStore for Store {
    fn get(&self, id: &str) -> Option<ApiResponse> {
        if let Some(res) = self.0.read().unwrap().get(&String::from(id)) {
            return Some(ApiResponse::new(res.data().to_owned()));
        }
        None
    }

    fn contains_key(&self, id: &str) -> bool {
        self.0.read().unwrap().contains_key(&String::from(id))
    }

    fn insert(
        &self,
        id: String,
        val: ApiResponse,
        lifetime: Option<time::Duration>,
    ) -> Option<ApiResponse> {
        self.0.write().unwrap().insert(id, val, lifetime)
    }
}
