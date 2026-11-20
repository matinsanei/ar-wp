//! AR-WP Native Shared In-Memory Object Cache (DashMap).
//!
//! Provides ultra-low latency, lock-free cross-request caching in Rust RAM.
//! Replaces external Redis / Memcached roundtrips with sub-microsecond in-process access.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use dashmap::DashMap;

#[derive(Clone, Debug)]
pub enum CacheVal {
    Null,
    Bool(bool),
    Long(i64),
    Double(f64),
    Str(Vec<u8>),
    Serialized(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct CacheItem {
    pub val: CacheVal,
    pub expires_at: Option<Instant>,
}

pub struct ArwpCache {
    store: DashMap<Vec<u8>, CacheItem>,
    hits: AtomicUsize,
    misses: AtomicUsize,
    sets: AtomicUsize,
}

impl ArwpCache {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            sets: AtomicUsize::new(0),
        }
    }

    pub fn global() -> &'static Self {
        static CACHE: OnceLock<ArwpCache> = OnceLock::new();
        CACHE.get_or_init(Self::new)
    }

    pub fn make_key(key: &[u8], group: &[u8]) -> Vec<u8> {
        let g = if group.is_empty() { b"default" } else { group };
        let mut k = Vec::with_capacity(g.len() + 1 + key.len());
        k.extend_from_slice(g);
        k.push(b':');
        k.extend_from_slice(key);
        k
    }

    pub fn get(&self, key: &[u8], group: &[u8]) -> Option<CacheVal> {
        let full_key = Self::make_key(key, group);
        let entry = self.store.get(&full_key)?;
        if let Some(exp) = entry.expires_at {
            if Instant::now() > exp {
                drop(entry);
                self.store.remove(&full_key);
                self.misses.fetch_add(1, Ordering::Relaxed);
                return None;
            }
        }
        self.hits.fetch_add(1, Ordering::Relaxed);
        Some(entry.val.clone())
    }

    pub fn set(&self, key: &[u8], val: CacheVal, group: &[u8], expire_secs: u64) -> bool {
        let full_key = Self::make_key(key, group);
        let expires_at = if expire_secs > 0 {
            Some(Instant::now() + Duration::from_secs(expire_secs))
        } else {
            None
        };
        self.store.insert(full_key, CacheItem { val, expires_at });
        self.sets.fetch_add(1, Ordering::Relaxed);
        true
    }

    pub fn add(&self, key: &[u8], val: CacheVal, group: &[u8], expire_secs: u64) -> bool {
        let full_key = Self::make_key(key, group);
        if let Some(entry) = self.store.get(&full_key) {
            if let Some(exp) = entry.expires_at {
                if Instant::now() <= exp {
                    return false; // key exists and valid
                }
            } else {
                return false; // key exists without expiration
            }
        }
        self.set(key, val, group, expire_secs)
    }

    pub fn delete(&self, key: &[u8], group: &[u8]) -> bool {
        let full_key = Self::make_key(key, group);
        self.store.remove(&full_key).is_some()
    }

    pub fn flush(&self) -> bool {
        self.store.clear();
        true
    }

    pub fn incr(&self, key: &[u8], offset: i64, group: &[u8]) -> Option<i64> {
        let full_key = Self::make_key(key, group);
        let mut entry = self.store.entry(full_key).or_insert_with(|| CacheItem {
            val: CacheVal::Long(0),
            expires_at: None,
        });
        match &entry.val {
            CacheVal::Long(curr) => {
                let next = curr.saturating_add(offset);
                entry.val = CacheVal::Long(next);
                Some(next)
            }
            _ => None,
        }
    }

    pub fn decr(&self, key: &[u8], offset: i64, group: &[u8]) -> Option<i64> {
        self.incr(key, -offset, group)
    }

    pub fn stats(&self) -> (usize, usize, usize, usize) {
        (
            self.store.len(),
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
            self.sets.load(Ordering::Relaxed),
        )
    }
}
