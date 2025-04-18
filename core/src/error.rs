use std::{
    error::Error,
    fmt, mem,
    num::{NonZeroU32, NonZeroU8},
};

pub use sys::{
    generated_safe::{
        self as sys_safe, CreateAccountErrorKind, CreateTransferErrorKind,
        InitStatusErrorKind as NewClientErrorKind, PacketStatusErrorKind as SendErrorKind,
    },
    tb_create_accounts_result_t as RawCreateAccountsIndividualApiResult,
    tb_create_transfers_result_t as RawCreateTransfersIndividualApiResult,
};

#[derive(Clone, Copy)]
pub struct NewClientError(pub(crate) NonZeroU32);

impl NewClientError {
    const CODE_RANGE: std::ops::RangeInclusive<u32> =
        sys_safe::MIN_INIT_STATUS_ERROR_CODE..=sys_safe::MAX_INIT_STATUS_ERROR_CODE;

    pub fn kind(self) -> NewClientErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above.
            unsafe { mem::transmute::<u32, NewClientErrorKind>(code) }
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
        let mut d = f.debug_tuple("NewClientError");
        let kind = self.kind();
        if matches!(kind, NewClientErrorKind::UnstableUncategorized) {
            let code = self.code();
            d.field(&code);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl fmt::Display for NewClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NewClientErrorKind as K;

        match self.kind() {
            K::AddressInvalid => "Replica addresses format is invalid",
            K::AddressLimitExceeded => "Replica addresses limit exceeded",
            K::NetworkSubsystem => "Internal client had unexpected networking issues",
            K::OutOfMemory => "Internal client ran out of memory",
            K::SystemResources => "Internal client ran out of system resources",
            K::Unexpected => "Unexpected internal error",
            _ => return write!(f, "Unknown error status: {}", self.code()),
        }
        .fmt(f)
    }
}

impl Error for NewClientError {}

impl From<NewClientErrorKind> for NewClientError {
    /// Constructs a [`NewClientError`] out of the provided [`NewClientErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on the hidden [`NewClientErrorKind::UnstableUncategorized`] variant.
    fn from(value: NewClientErrorKind) -> Self {
        let this = Self(NonZeroU32::new(value as _).unwrap());
        if matches!(this.kind(), NewClientErrorKind::UnstableUncategorized) {
            panic!("NewClientErrorKind::{value:?}")
        }
        this
    }
}

#[derive(Clone, Copy)]
pub struct SendError(pub(crate) NonZeroU8);

impl SendError {
    const CODE_RANGE: std::ops::RangeInclusive<u8> =
        sys_safe::MIN_PACKET_STATUS_ERROR_CODE..=sys_safe::MAX_PACKET_STATUS_ERROR_CODE;

    pub fn kind(self) -> SendErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above.
            unsafe { mem::transmute::<u8, SendErrorKind>(code) }
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
            let code = self.code();
            d.field(&code);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SendErrorKind as K;

        match self.kind() {
            K::TooMuchData => "Too much data provided on this batch",
            K::InvalidOperation => "Invalid operation",
            K::InvalidDataSize => "Invalid data size",
            K::ClientEvicted => "Client was evicted",
            K::ClientReleaseTooLow => "Client was evicted: release too old",
            K::ClientReleaseTooHigh => "Client was evicted: release too new",
            K::ClientShutdown => "Client was closed",
            _ => return write!(f, "Unknown error status: {}", self.code()),
        }
        .fmt(f)
    }
}

impl Error for SendError {}

impl From<SendErrorKind> for SendError {
    /// Constructs a [`SendError`] out of the provided [`SendErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on the hidden [`SendErrorKind::UnstableUncategorized`] variant.
    fn from(value: SendErrorKind) -> Self {
        let this = Self(NonZeroU8::new(value as _).unwrap());
        if matches!(this.kind(), SendErrorKind::UnstableUncategorized) {
            panic!("SendErrorKind::{value:?}")
        }
        this
    }
}

#[derive(Clone, Copy)]
pub struct CreateAccountError(pub(crate) NonZeroU32);

impl CreateAccountError {
    const CODE_RANGE: std::ops::RangeInclusive<u32> =
        sys_safe::MIN_CREATE_ACCOUNT_ERROR_CODE..=sys_safe::MAX_CREATE_ACCOUNT_ERROR_CODE;

    pub fn kind(self) -> CreateAccountErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code) {
            // SAFETY: We checked if it's in range right above.
            unsafe { mem::transmute::<u32, CreateAccountErrorKind>(code) }
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
            d.field(&self.0);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl fmt::Display for CreateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind())
    }
}

impl Error for CreateAccountError {}

impl From<CreateAccountErrorKind> for CreateAccountError {
    /// Constructs a [`CreateAccountError`] out of the provided [`CreateAccountErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on the hidden [`CreateAccountErrorKind::UnstableUncategorized`] variant.
    fn from(value: CreateAccountErrorKind) -> Self {
        let this = Self(NonZeroU32::new(value as _).unwrap());
        if matches!(this.kind(), CreateAccountErrorKind::UnstableUncategorized) {
            panic!("CreateAccountErrorKind::{value:?}")
        }
        this
    }
}

/// Type indicating individual API error for account creation.
///
/// Safe to `transpose` from [`RawCreateAccountsIndividualApiResult`]
/// if [`Self::from_raw_result_unchecked`] would also be safe.
// INVARIANT: `self.0.result` must not be zero.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CreateAccountsIndividualApiError(RawCreateAccountsIndividualApiResult);

impl CreateAccountsIndividualApiError {
    /// Create error from raw result.
    ///
    /// # Errors
    ///
    /// Returns `None` if `raw.result` is zero.
    pub fn from_raw_result(raw: RawCreateAccountsIndividualApiResult) -> Option<Self> {
        (raw.result != 0).then_some(Self(raw))
    }

    /// Create error from raw result. Unchecked version of [`Self::from_raw_result`].
    ///
    /// # Safety
    ///
    /// This function is unsafe. `raw.result` must not be zero.
    pub unsafe fn from_raw_result_unchecked(raw: RawCreateAccountsIndividualApiResult) -> Self {
        Self(raw)
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

impl fmt::Display for CreateAccountsIndividualApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` error occurred at account with index {}",
            self.inner(),
            self.index(),
        )
    }
}

impl Error for CreateAccountsIndividualApiError {}

// INVARIANT: `self.0` must not be empty.
#[derive(Debug)]
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

impl fmt::Display for CreateAccountsApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} api errors occurred at accounts' creation",
            self.0.len(),
        )
    }
}

impl Error for CreateAccountsApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.first().map(|e| e as _)
    }
}

impl From<CreateAccountsIndividualApiError> for CreateAccountsApiError {
    fn from(value: CreateAccountsIndividualApiError) -> Self {
        CreateAccountsApiError(vec![value])
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CreateAccountsError {
    Send(SendError),
    Api(CreateAccountsApiError),
}

impl Error for CreateAccountsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Send(e) => e as _,
            Self::Api(e) => e as _,
        })
    }
}

impl fmt::Display for CreateAccountsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to create accounts: ")?;
        match self {
            Self::Send(e) => write!(f, "{e}"),
            Self::Api(e) => write!(f, "{e}"),
        }
    }
}

impl From<SendError> for CreateAccountsError {
    fn from(value: SendError) -> Self {
        Self::Send(value)
    }
}

impl From<CreateAccountsApiError> for CreateAccountsError {
    fn from(value: CreateAccountsApiError) -> Self {
        Self::Api(value)
    }
}

#[derive(Clone, Copy)]
pub struct CreateTransferError(pub(crate) NonZeroU32);

impl CreateTransferError {
    const CODE_RANGE: std::ops::RangeInclusive<u32> =
        sys_safe::MIN_CREATE_TRANSFER_ERROR_CODE..=sys_safe::MAX_CREATE_TRANSFER_ERROR_CODE;

    pub fn kind(self) -> CreateTransferErrorKind {
        let code = self.0.get();
        if Self::CODE_RANGE.contains(&code)
            && !sys_safe::EXCLUDED_CREATE_TRANSFER_ERROR_CODES.contains(&code)
        {
            // SAFETY: We checked if it's in range right above.
            unsafe { mem::transmute::<u32, CreateTransferErrorKind>(code) }
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

impl fmt::Display for CreateTransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind())
    }
}

impl Error for CreateTransferError {}

impl From<CreateTransferErrorKind> for CreateTransferError {
    /// Constructs a [`CreateTransferError`] out of the provided [`CreateTransferErrorKind`].
    ///
    /// # Panics
    ///
    /// Panics on the hidden [`CreateTransferErrorKind::UnstableUncategorized`] variant.
    fn from(value: CreateTransferErrorKind) -> Self {
        let this = Self(NonZeroU32::new(value as _).unwrap());
        if matches!(this.kind(), CreateTransferErrorKind::UnstableUncategorized) {
            panic!("CreateTransferErrorKind::{value:?}")
        }
        this
    }
}

/// Type indicating individual API error for transfer creation.
///
/// Safe to `transpose` from [`RawCreateTransfersIndividualApiResult`]
/// if [`Self::from_raw_result_unchecked`] would also be safe.
// INVARIANT: `self.0.result` must not be zero.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CreateTransfersIndividualApiError(RawCreateTransfersIndividualApiResult);

impl CreateTransfersIndividualApiError {
    /// Create error from raw struct.
    ///
    /// # Errors
    ///
    /// Returns `None` if `raw.result` is zero.
    pub fn from_raw_result(raw: RawCreateTransfersIndividualApiResult) -> Option<Self> {
        (raw.result != 0).then_some(Self(raw))
    }

    /// Create error from raw struct. Unchecked version of [`Self::from_raw_result`].
    ///
    /// # Safety
    ///
    /// This function is unsafe. `raw.result` must not be zero.
    pub unsafe fn from_raw_result_unchecked(raw: RawCreateTransfersIndividualApiResult) -> Self {
        Self(raw)
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

impl fmt::Display for CreateTransfersIndividualApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` error occurred at account with index {}",
            self.inner(),
            self.index(),
        )
    }
}

impl Error for CreateTransfersIndividualApiError {}

// INVARIANT: `self.0` must not be empty.
#[derive(Debug)]
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

impl AsRef<[CreateTransfersIndividualApiError]> for CreateTransfersApiError {
    fn as_ref(&self) -> &[CreateTransfersIndividualApiError] {
        &self.0
    }
}

impl fmt::Display for CreateTransfersApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} api errors occurred at transfers' creation",
            self.0.len(),
        )
    }
}

impl Error for CreateTransfersApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.first().map(|e| e as _)
    }
}

impl From<CreateTransfersIndividualApiError> for CreateTransfersApiError {
    fn from(value: CreateTransfersIndividualApiError) -> Self {
        Self(vec![value])
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum CreateTransfersError {
    Send(SendError),
    Api(CreateTransfersApiError),
}

impl Error for CreateTransfersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Send(e) => e as _,
            Self::Api(e) => e as _,
        })
    }
}

impl fmt::Display for CreateTransfersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to create transfers: ")?;
        match self {
            Self::Send(e) => write!(f, "{e}"),
            Self::Api(e) => write!(f, "{e}"),
        }
    }
}

impl From<SendError> for CreateTransfersError {
    fn from(value: SendError) -> Self {
        Self::Send(value)
    }
}

impl From<CreateTransfersApiError> for CreateTransfersError {
    fn from(value: CreateTransfersApiError) -> Self {
        Self::Api(value)
    }
}
