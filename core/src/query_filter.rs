use std::{
    fmt,
    time::{Duration, SystemTime},
};

use bytemuck::{Pod, TransparentWrapper, Zeroable};
use derive_more::{From, Into};

pub use sys::{generated_safe::QueryFilterFlags as Flags, tb_query_filter_t as Raw};

#[derive(Clone, Copy, From, Into, Pod, TransparentWrapper, Zeroable)]
#[repr(transparent)]
pub struct QueryFilter(Raw);

impl QueryFilter {
    #[track_caller]
    pub fn new(limit: u32) -> Self {
        Self(Raw::zeroed()).with_limit(limit)
    }

    pub const fn from_raw(raw: Raw) -> Self {
        Self(raw)
    }
    pub const fn into_raw(self) -> Raw {
        self.0
    }
    pub const fn as_raw(&self) -> &Raw {
        &self.0
    }
    pub fn as_raw_mut(&mut self) -> &mut Raw {
        &mut self.0
    }

    pub const fn user_data_128(&self) -> u128 {
        self.0.user_data_128
    }
    pub fn set_user_data_128(&mut self, user_data: u128) {
        self.0.user_data_128 = user_data;
    }
    pub fn with_user_data_128(mut self, user_data: u128) -> Self {
        self.set_user_data_128(user_data);
        self
    }

    pub const fn user_data_64(&self) -> u64 {
        self.0.user_data_64
    }
    pub fn set_user_data_64(&mut self, user_data: u64) {
        self.0.user_data_64 = user_data;
    }
    pub fn with_user_data_64(mut self, user_data: u64) -> Self {
        self.set_user_data_64(user_data);
        self
    }

    pub const fn user_data_32(&self) -> u32 {
        self.0.user_data_32
    }
    pub fn set_user_data_32(&mut self, user_data: u32) {
        self.0.user_data_32 = user_data;
    }
    pub fn with_user_data_32(mut self, user_data: u32) -> Self {
        self.set_user_data_32(user_data);
        self
    }

    pub const fn ledger(&self) -> u32 {
        self.0.ledger
    }
    pub fn set_ledger(&mut self, ledger: u32) {
        self.0.ledger = ledger;
    }
    pub fn with_ledger(mut self, ledger: u32) -> Self {
        self.set_ledger(ledger);
        self
    }

    pub const fn code(&self) -> u16 {
        self.0.code
    }
    pub fn set_code(&mut self, code: u16) {
        self.0.code = code;
    }
    pub fn with_code(mut self, code: u16) -> Self {
        self.set_code(code);
        self
    }

    pub fn timestamp_min(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_nanos(self.0.timestamp_min)
    }
    pub fn set_timestamp_min(&mut self, timestamp_min: SystemTime) {
        let t = timestamp_min
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .and_then(|t| t.as_nanos().try_into().ok())
            .expect("failed to get nanoseconds since unix epoch from the argument");
        assert_ne!(t, u64::MAX, "timestamp_min must not be `2^64 - 1`");
        self.0.timestamp_min = t;
    }
    pub fn with_timestamp_min(mut self, timestamp_min: SystemTime) -> Self {
        self.set_timestamp_min(timestamp_min);
        self
    }

    pub fn timestamp_max(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_nanos(self.0.timestamp_max)
    }
    pub fn set_timestamp_max(&mut self, timestamp_max: SystemTime) {
        let t = timestamp_max
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .and_then(|t| t.as_nanos().try_into().ok())
            .expect("failed to get nanoseconds since unix epoch from the argument");
        assert_ne!(t, u64::MAX, "timestamp_max must not be `2^64 - 1`");
        self.0.timestamp_max = t;
    }
    pub fn with_timestamp_max(mut self, timestamp_max: SystemTime) -> Self {
        self.set_timestamp_max(timestamp_max);
        self
    }

    pub const fn limit(&self) -> u32 {
        self.0.limit
    }
    pub fn set_limit(&mut self, limit: u32) {
        assert_ne!(limit, 0, "limit must not be zero");
        self.0.limit = limit;
    }
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.set_limit(limit);
        self
    }

    pub const fn flags(&self) -> Flags {
        Flags::from_bits_retain(self.0.flags)
    }
    pub fn set_flags(&mut self, flags: Flags) {
        self.0.flags = flags.bits();
    }
    pub const fn with_flags(mut self, flags: Flags) -> Self {
        self.0.flags = flags.bits();
        self
    }
}

impl fmt::Debug for QueryFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccountFilter")
            .field("user_data_128", &self.0.user_data_128)
            .field("user_data_64", &self.0.user_data_64)
            .field("user_data_32", &self.0.user_data_32)
            .field("ledger", &self.0.ledger)
            .field("code", &self.0.code)
            .field("timestamp_min", &self.timestamp_min())
            .field("timestamp_max", &self.timestamp_max())
            .field("limit", &self.0.limit)
            .field("flags", &self.flags())
            .finish_non_exhaustive()
    }
}
