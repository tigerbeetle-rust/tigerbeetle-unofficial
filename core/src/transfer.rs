use std::{
    fmt,
    time::{Duration, SystemTime},
};

use bytemuck::{Pod, TransparentWrapper, Zeroable};
use derive_more::{From, Into};

pub use sys::{generated_safe::TransferFlags as Flags, tb_transfer_t as Raw};

#[derive(Clone, Copy, From, Into, Pod, TransparentWrapper, Zeroable)]
#[repr(transparent)]
pub struct Transfer(Raw);

impl Transfer {
    #[track_caller]
    pub fn new(id: u128) -> Self {
        Transfer(Raw::zeroed()).with_id(id)
    }

    pub const fn from_raw(raw: Raw) -> Self {
        Transfer(raw)
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

    pub const fn id(&self) -> u128 {
        self.0.id
    }
    #[track_caller]
    pub fn set_id(&mut self, id: u128) {
        assert_ne!(id, 0, "transfer id must not be zero");
        assert_ne!(
            id,
            u128::MAX,
            "transfer id must not be `2^128 - 1` (the highest 128-bit unsigned integer)"
        );
        self.0.id = id;
    }
    #[track_caller]
    pub fn with_id(mut self, id: u128) -> Self {
        self.set_id(id);
        self
    }

    pub const fn debit_account_id(&self) -> u128 {
        self.0.debit_account_id
    }
    pub fn set_debit_account_id(&mut self, debit_account_id: u128) {
        self.0.debit_account_id = debit_account_id;
    }
    pub const fn with_debit_account_id(mut self, debit_account_id: u128) -> Self {
        self.0.debit_account_id = debit_account_id;
        self
    }

    pub const fn credit_account_id(&self) -> u128 {
        self.0.credit_account_id
    }
    pub fn set_credit_account_id(&mut self, credit_account_id: u128) {
        self.0.credit_account_id = credit_account_id;
    }
    pub const fn with_credit_account_id(mut self, credit_account_id: u128) -> Self {
        self.0.credit_account_id = credit_account_id;
        self
    }

    pub const fn user_data_128(&self) -> u128 {
        self.0.user_data_128
    }
    pub fn set_user_data_128(&mut self, user_data_128: u128) {
        self.0.user_data_128 = user_data_128;
    }
    pub const fn with_user_data_128(mut self, user_data_128: u128) -> Self {
        self.0.user_data_128 = user_data_128;
        self
    }

    pub const fn user_data_64(&self) -> u64 {
        self.0.user_data_64
    }
    pub fn set_user_data_64(&mut self, user_data_64: u64) {
        self.0.user_data_64 = user_data_64;
    }
    pub const fn with_user_data_64(mut self, user_data_64: u64) -> Self {
        self.0.user_data_64 = user_data_64;
        self
    }

    pub const fn user_data_32(&self) -> u32 {
        self.0.user_data_32
    }
    pub fn set_user_data_32(&mut self, user_data_32: u32) {
        self.0.user_data_32 = user_data_32;
    }
    pub const fn with_user_data_32(mut self, user_data_32: u32) -> Self {
        self.0.user_data_32 = user_data_32;
        self
    }

    pub const fn ledger(&self) -> u32 {
        self.0.ledger
    }
    pub fn set_ledger(&mut self, ledger: u32) {
        self.0.ledger = ledger;
    }
    pub const fn with_ledger(mut self, ledger: u32) -> Self {
        self.0.ledger = ledger;
        self
    }

    pub const fn code(&self) -> u16 {
        self.0.code
    }
    pub fn set_code(&mut self, code: u16) {
        self.0.code = code;
    }
    pub const fn with_code(mut self, code: u16) -> Self {
        self.0.code = code;
        self
    }

    pub const fn pending_id(&self) -> u128 {
        self.0.pending_id
    }
    pub fn set_pending_id(&mut self, pending_id: u128) {
        self.0.pending_id = pending_id;
    }
    pub const fn with_pending_id(mut self, pending_id: u128) -> Self {
        self.0.pending_id = pending_id;
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

    pub const fn timeout(&self) -> u32 {
        self.0.timeout
    }
    #[track_caller]
    pub fn set_timeout(&mut self, timeout: u32) {
        self.0.timeout = timeout;
    }
    #[track_caller]
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.set_timeout(timeout);
        self
    }

    pub const fn amount(&self) -> u128 {
        self.0.amount
    }
    pub fn set_amount(&mut self, amount: u128) {
        self.0.amount = amount;
    }
    pub const fn with_amount(mut self, amount: u128) -> Self {
        self.0.amount = amount;
        self
    }

    pub fn timestamp(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_nanos(self.0.timestamp)
    }
}

impl fmt::Debug for Transfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transfer")
            .field("id", &self.id())
            .field("debit_account_id", &self.debit_account_id())
            .field("credit_account_id", &self.credit_account_id())
            .field("amount", &self.amount())
            .field("pending_id", &self.pending_id())
            .field("user_data_128", &self.user_data_128())
            .field("user_data_64", &self.user_data_64())
            .field("user_data_32", &self.user_data_32())
            .field("timeout", &self.timeout())
            .field("ledger", &self.ledger())
            .field("code", &self.code())
            .field("flags", &self.flags())
            .field("timestamp", &self.timestamp())
            .finish_non_exhaustive()
    }
}
