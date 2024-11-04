use std::{
    fmt, mem,
    num::{NonZeroU32, NonZeroU8},
};

use derive_more::{AsRef, Display, Error as StdError, From};

pub use sys::{
    generated_safe::{
        self as sys_safe, CreateAccountErrorKind, CreateTransferErrorKind,
        PacketStatusErrorKind as SendErrorKind, StatusErrorKind as NewClientErrorKind,
    },
    tb_create_accounts_result_t as RawCreateAccountsIndividualApiResult,
    tb_create_transfers_result_t as RawCreateTransfersIndividualApiResult,
};

#[derive(Clone, Copy, Debug, Display, StdError)]
#[display("{_0:?}")]
pub struct ClientClosedError(#[error(not(source))] ());

#[derive(Clone, Copy, Display, StdError)]
#[display("{_0:?}")]
pub struct NewClientError(#[error(not(source))] pub(crate) NonZeroU32);

impl NewClientError {
    const CODE_RANGE: std::ops::RangeInclusive<u32> =
        sys_safe::MIN_STATUS_ERROR_CODE..=sys_safe::MAX_STATUS_ERROR_CODE;

    pub fn kind(self) -> NewClientErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above
            unsafe { mem::transmute(code) }
        } else {
            NewClientErrorKind::UnstableUncategorized
        }
    }

    pub fn code(self) -> NonZeroU32 {
        self.0
    }
}

impl fmt::Debug for NewClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("NewClientErrorError");
        let kind = self.kind();
        if matches!(kind, NewClientErrorKind::UnstableUncategorized) {
            let code = self.code().get();
            d.field(&code);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl From<NewClientErrorKind> for NewClientError {
    /// Constructs a [`NewClientError`] out of [`NewClientErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on hidden `NewClientErrorKind::UnstableUncategorized` variant.
    fn from(value: NewClientErrorKind) -> Self {
        let this = NewClientError(NonZeroU32::new(value as _).unwrap());
        if matches!(this.kind(), NewClientErrorKind::UnstableUncategorized) {
            panic!("NewClientErrorKind::{value:?}")
        }
        this
    }
}

#[derive(Clone, Copy, Display, StdError)]
#[display("error occured while sending packets for accounts creation")]
pub struct SendError(#[error(not(source))] pub(crate) NonZeroU8);

impl SendError {
    const CODE_RANGE: std::ops::RangeInclusive<u8> =
        sys_safe::MIN_PACKET_STATUS_ERROR_CODE..=sys_safe::MAX_PACKET_STATUS_ERROR_CODE;

    pub fn kind(self) -> SendErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above
            unsafe { mem::transmute(code) }
        } else {
            SendErrorKind::UnstableUncategorized
        }
    }

    pub fn code(self) -> NonZeroU8 {
        self.0
    }
}

impl fmt::Debug for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("SendError");
        let kind = self.kind();
        if matches!(kind, SendErrorKind::UnstableUncategorized) {
            let code = self.code().get();
            d.field(&code);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl From<SendErrorKind> for SendError {
    /// Constructs a [`SendError`] out of [`SendErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on hidden `SendErrorKind::UnstableUncategorized` variant.
    fn from(value: SendErrorKind) -> Self {
        let this = SendError(NonZeroU8::new(value as _).unwrap());
        if matches!(this.kind(), SendErrorKind::UnstableUncategorized) {
            panic!("SendErrorKind::{value:?}")
        }
        this
    }
}

#[derive(Clone, Copy, Display, StdError)]
#[display("{_0:?}")]
pub struct CreateAccountError(#[error(not(source))] pub(crate) NonZeroU32);

impl CreateAccountError {
    const CODE_RANGE: std::ops::RangeInclusive<u32> =
        sys_safe::MIN_CREATE_ACCOUNT_ERROR_CODE..=sys_safe::MAX_CREATE_ACCOUNT_ERROR_CODE;

    pub fn kind(self) -> CreateAccountErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above
            unsafe { mem::transmute(code) }
        } else {
            CreateAccountErrorKind::UnstableUncategorized
        }
    }

    pub fn code(self) -> NonZeroU32 {
        self.0
    }
}

impl fmt::Debug for CreateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("CreateAccountError");
        let kind = self.kind();
        if matches!(kind, CreateAccountErrorKind::UnstableUncategorized) {
            let code = self.0.get();
            d.field(&code);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl From<CreateAccountErrorKind> for CreateAccountError {
    /// Constructs a [`CreateAccountError`] out of [`CreateAccountErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on hidden `CreateAccountErrorKind::UnstableUncategorized` variant.
    fn from(value: CreateAccountErrorKind) -> Self {
        let this = CreateAccountError(NonZeroU32::new(value as _).unwrap());
        if matches!(this.kind(), CreateAccountErrorKind::UnstableUncategorized) {
            panic!("CreateAccountErrorKind::{value:?}")
        }
        this
    }
}

/// Type indicating individual api error for account creation.
///
/// Safe to `transpose` from [`RawCreateAccountsIndividualApiResult`]
/// if [`Self::from_raw_result_unchecked`] would also be safe.
//
// INVARIANT: self.0.result must not be zero
#[derive(Clone, Copy, Display, StdError)]
#[display(
    "`{}` error occured at account with index {}",
    self.inner(),
    self.index(),
)]
#[repr(transparent)]
pub struct CreateAccountsIndividualApiError(
    #[error(not(source))] RawCreateAccountsIndividualApiResult,
);

impl CreateAccountsIndividualApiError {
    /// Create error from raw result.
    ///
    /// # Errors
    ///
    /// Returns `None` if `raw.result` is zero.
    pub fn from_raw_result(raw: RawCreateAccountsIndividualApiResult) -> Option<Self> {
        (raw.result != 0).then_some(CreateAccountsIndividualApiError(raw))
    }

    /// Create error from raw result. Unchecked version of [`Self::from_raw_result`].
    ///
    /// # Safety
    ///
    /// This function is unsafe. `raw.result` must not be zero.
    pub unsafe fn from_raw_result_unchecked(raw: RawCreateAccountsIndividualApiResult) -> Self {
        CreateAccountsIndividualApiError(raw)
    }

    /// Create vec of errors from vec of raw results.
    ///
    /// Retains only elements `r` of vec `v` that satisfy `r.result != 0`.
    pub fn vec_from_raw_results(mut v: Vec<RawCreateAccountsIndividualApiResult>) -> Vec<Self> {
        v.retain(|r| r.result != 0);
        unsafe { Self::vec_from_raw_results_unchecked(v) }
    }

    /// Create vec of errors from vec of raw results. Unchecked version of
    /// [`Self::vec_from_raw_results`]
    ///
    /// # Safety
    ///
    /// This function is unsafe. Every element `r` of vec `v` must satisfy
    /// `r.result != 0`.
    pub unsafe fn vec_from_raw_results_unchecked(
        v: Vec<RawCreateAccountsIndividualApiResult>,
    ) -> Vec<Self> {
        let mut v = mem::ManuallyDrop::new(v);
        let len = v.len();
        let cap = v.capacity();
        let ptr = v.as_mut_ptr().cast::<CreateAccountsIndividualApiError>();
        // SAFETY: this is fine because `Vec::from_raw_parts` has pretty loose
        // safety requirements, and since `CreateAccountsIndividualApiError` is
        // just a transparent wrapper of `RawCreateAccountsIndividualApiResult`
        // this is safe.
        Vec::from_raw_parts(ptr, len, cap)
    }

    /// Get index of the failed account.
    pub fn index(&self) -> u32 {
        self.0.index
    }

    /// Get error stripped of context, like index.
    pub fn inner(&self) -> CreateAccountError {
        CreateAccountError(
            // SAFETY: type invariant
            unsafe { NonZeroU32::new_unchecked(self.0.result) },
        )
    }

    /// Get kind of error to match upon.
    pub fn kind(&self) -> CreateAccountErrorKind {
        self.inner().kind()
    }
}

impl fmt::Debug for CreateAccountsIndividualApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateAccountsIndividualApiError")
            .field("index", &self.index())
            .field("inner", &self.inner())
            .finish()
    }
}

// INVARIANT: self.0 must not be empty
#[derive(Debug, Display)]
#[display(
    "{} api errors occured at accounts creation",
     self.0.len(),
)]
pub struct CreateAccountsApiError(Vec<CreateAccountsIndividualApiError>);

impl CreateAccountsApiError {
    /// Get a slice of individual errors. Never empty.
    pub fn as_slice(&self) -> &[CreateAccountsIndividualApiError] {
        &self.0
    }

    /// Create error from vec of raw results.
    ///
    /// # Errors
    ///
    /// Returns `None` if `v.is_empty()`.
    pub fn from_errors(v: Vec<CreateAccountsIndividualApiError>) -> Option<Self> {
        (!v.is_empty()).then_some(CreateAccountsApiError(v))
    }

    /// Create error from vec of raw results.
    ///
    /// Retains only results with errors.
    pub fn from_raw_results(v: Vec<RawCreateAccountsIndividualApiResult>) -> Option<Self> {
        Self::from_errors(CreateAccountsIndividualApiError::vec_from_raw_results(v))
    }
}

impl AsRef<[CreateAccountsIndividualApiError]> for CreateAccountsApiError {
    fn as_ref(&self) -> &[CreateAccountsIndividualApiError] {
        &self.0
    }
}

impl StdError for CreateAccountsApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.first().map(|e| e as _)
    }
}

impl From<CreateAccountsIndividualApiError> for CreateAccountsApiError {
    fn from(value: CreateAccountsIndividualApiError) -> Self {
        CreateAccountsApiError(vec![value])
    }
}

#[non_exhaustive]
#[derive(Debug, Display, From, StdError)]
pub enum CreateAccountsError {
    Send(SendError),
    Api(CreateAccountsApiError),
}

#[derive(Clone, Copy, Display, StdError)]
pub struct CreateTransferError(#[error(not(source))] pub(crate) NonZeroU32);

impl CreateTransferError {
    const CODE_RANGE: std::ops::RangeInclusive<u32> =
        sys_safe::MIN_CREATE_TRANSFER_ERROR_CODE..=sys_safe::MAX_CREATE_TRANSFER_ERROR_CODE;

    pub fn kind(self) -> CreateTransferErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above
            unsafe { mem::transmute(code) }
        } else {
            CreateTransferErrorKind::UnstableUncategorized
        }
    }

    pub fn code(self) -> NonZeroU32 {
        self.0
    }
}

impl fmt::Debug for CreateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("CreateTransferError");
        let kind = self.kind();
        if matches!(kind, CreateTransferErrorKind::UnstableUncategorized) {
            let code = self.code().get();
            d.field(&code);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl From<CreateTransferErrorKind> for CreateTransferError {
    /// Constructs a [`CreateTransferError`] out of [`CreateTransferErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on hidden `CreateTransferErrorKind::UnstableUncategorized` variant.
    fn from(value: CreateTransferErrorKind) -> Self {
        let this = CreateTransferError(NonZeroU32::new(value as _).unwrap());
        if matches!(this.kind(), CreateTransferErrorKind::UnstableUncategorized) {
            panic!("CreateTransferErrorKind::{value:?}")
        }
        this
    }
}

/// Type indicating individual api error for account creation.
///
/// Safe to `transpose` from [`RawCreateTransfersIndividualApiResult`]
/// if [`Self::from_raw_result_unchecked`] would also be safe.
//
// INVARIANT: self.0.result must not be zero
#[derive(Clone, Copy, Display, StdError)]
#[display(
    "`{}` error occured at account with index {}",
    self.inner(),
    self.index(),
)]
#[repr(transparent)]
pub struct CreateTransfersIndividualApiError(
    #[error(not(source))] RawCreateTransfersIndividualApiResult,
);

impl CreateTransfersIndividualApiError {
    /// Create error from raw struct.
    ///
    /// # Errors
    ///
    /// Returns `None` if `raw.result` is zero.
    pub fn from_raw_result(raw: RawCreateTransfersIndividualApiResult) -> Option<Self> {
        (raw.result != 0).then_some(CreateTransfersIndividualApiError(raw))
    }

    /// Create error from raw struct. Unchecked version of [`Self::from_raw_result`].
    ///
    /// # Safety
    ///
    /// This function is unsafe. `raw.result` must not be zero.
    pub unsafe fn from_raw_result_unchecked(raw: RawCreateTransfersIndividualApiResult) -> Self {
        CreateTransfersIndividualApiError(raw)
    }

    /// Create vec of errors from vec of raw results.
    ///
    /// Retains only elements `r` of vec `v` that satisfy `r.result != 0`.
    pub fn vec_from_raw_results(mut v: Vec<RawCreateTransfersIndividualApiResult>) -> Vec<Self> {
        v.retain(|r| r.result != 0);
        unsafe { Self::vec_from_raw_results_unchecked(v) }
    }

    /// Create vec of errors from vec of raw results. Unchecked version of
    /// [`Self::vec_from_raw_results`]
    ///
    /// # Safety
    ///
    /// This function is unsafe. Every element `r` of vec `v` must satisfy
    /// `r.result != 0`.
    pub unsafe fn vec_from_raw_results_unchecked(
        v: Vec<RawCreateTransfersIndividualApiResult>,
    ) -> Vec<Self> {
        let mut v = mem::ManuallyDrop::new(v);
        let len = v.len();
        let cap = v.capacity();
        let ptr = v.as_mut_ptr().cast::<CreateTransfersIndividualApiError>();
        // SAFETY: this is fine because `Vec::from_raw_parts` has pretty loose
        // safety requirements, and since `CreateTransfersIndividualApiError` is
        // just a transparent wrapper of `RawCreateTransfersIndividualApiResult`
        // this is safe.
        Vec::from_raw_parts(ptr, len, cap)
    }

    /// Get index of the failed transfer.
    pub fn index(&self) -> u32 {
        self.0.index
    }

    /// Get error stripped of context, like index.
    pub fn inner(&self) -> CreateTransferError {
        CreateTransferError(
            // SAFETY: type invariant
            unsafe { NonZeroU32::new_unchecked(self.0.result) },
        )
    }

    /// Get kind of error to match upon.
    pub fn kind(&self) -> CreateTransferErrorKind {
        self.inner().kind()
    }
}

impl fmt::Debug for CreateTransfersIndividualApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateTransfersIndividualApiError")
            .field("index", &self.index())
            .field("inner", &self.inner())
            .finish()
    }
}

// INVARIANT: self.0 must not be empty
#[derive(AsRef, Debug, Display)]
#[as_ref(forward)]
#[display(
    "{} api errors occured at transfers' creation",
    self.0.len(),
)]
pub struct CreateTransfersApiError(Vec<CreateTransfersIndividualApiError>);

impl CreateTransfersApiError {
    /// Get a slice of individual errors. Never empty.
    pub fn as_slice(&self) -> &[CreateTransfersIndividualApiError] {
        &self.0
    }

    /// Create error from vec of raw results.
    ///
    /// # Errors
    ///
    /// Returns `None` if `v.is_empty()`.
    pub fn from_errors(v: Vec<CreateTransfersIndividualApiError>) -> Option<Self> {
        (!v.is_empty()).then_some(CreateTransfersApiError(v))
    }

    /// Create error from vec of raw results.
    ///
    /// Retains only results with errors.
    pub fn from_raw_results(v: Vec<RawCreateTransfersIndividualApiResult>) -> Option<Self> {
        Self::from_errors(CreateTransfersIndividualApiError::vec_from_raw_results(v))
    }
}

impl StdError for CreateTransfersApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.first().map(|e| e as _)
    }
}

impl From<CreateTransfersIndividualApiError> for CreateTransfersApiError {
    fn from(value: CreateTransfersIndividualApiError) -> Self {
        CreateTransfersApiError(vec![value])
    }
}

#[non_exhaustive]
#[derive(Debug, Display, From, StdError)]
pub enum CreateTransfersError {
    Send(SendError),
    Api(CreateTransfersApiError),
}
