use std::{ffi::c_void, mem, num::NonZeroU8, ptr, marker::PhantomData};

pub use sys::generated_safe::OperationKind;

use crate::error::SendError;
use super::callback::{UserData, UserDataPtr};

pub struct Packet<U>
where
    U: UserDataPtr,
{
    pub(super) raw: *mut sys::tb_packet_t,
    pub(super) _ptr: PhantomData<U>,
}

#[derive(Clone, Copy)]
pub struct Operation(pub(crate) u8);

unsafe impl<U> Sync for Packet<U>
where
    U: UserDataPtr,
    U::Pointee: Sync,
{
}
unsafe impl<U> Send for Packet<U> where U: UserDataPtr {}

impl<U> Packet<U>
where
    U: UserDataPtr,
{
    /// Creates a new [`Packet`].
    #[must_use]
    pub fn new(user_data: U, operation: impl Into<Operation>) -> Self {
        Self {
            raw: Box::into_raw(Box::new(sys::tb_packet_t {
                user_data: U::into_raw_const_ptr(user_data).cast::<c_void>().cast_mut(),
                data: ptr::null_mut(),
                data_size: 0,
                user_tag: 0,
                operation: operation.into().0,
                status: 0,
                opaque: [0; 32],
            })),
            _ptr: PhantomData,
        }
    }

    pub(crate) fn raw(&self) -> &sys::tb_packet_t {
        unsafe { &*self.raw }
    }

    pub(crate) fn raw_mut(&mut self) -> &mut sys::tb_packet_t {
        unsafe { &mut *self.raw }
    }

    pub fn into_user_data(self) -> U {
        let this = mem::ManuallyDrop::new(self);
        let user_data;
        unsafe {
            user_data = U::from_raw_const_ptr(this.raw().user_data.cast_const().cast());
            drop(Box::from_raw(this.raw));
        }
        user_data
    }

    pub fn replace_user_data(&mut self, user_data: U) -> U {
        let new = U::into_raw_const_ptr(user_data).cast_mut().cast();
        let ptr = mem::replace(&mut self.raw_mut().user_data, new)
            .cast_const()
            .cast();
        unsafe { U::from_raw_const_ptr(ptr) }
    }

    pub fn user_data(&self) -> &U::Target {
        unsafe { self.raw().user_data.cast::<U::Target>().as_ref().unwrap() }
    }

    pub fn user_data_mut(&mut self) -> &mut U::Target
    where
        U: std::ops::DerefMut,
    {
        unsafe {
            self.raw_mut()
                .user_data
                .cast::<U::Target>()
                .as_mut()
                .unwrap()
        }
    }

    pub fn data(&self) -> &[u8] {
        self.user_data().data()
    }

    pub fn operation(&self) -> Operation {
        Operation(self.raw().operation)
    }

    pub fn set_operation(&mut self, operation: Operation) {
        self.raw_mut().operation = operation.0;
    }

    pub fn status(&self) -> Result<(), SendError> {
        if let Some(c) = NonZeroU8::new(self.raw().status) {
            Err(SendError(c))
        } else {
            Ok(())
        }
    }

    pub fn set_status(&mut self, status: Result<(), SendError>) {
        self.raw_mut().status = match status {
            Ok(()) => 0,
            Err(e) => e.0.get(),
        }
    }
}

impl<U> Drop for Packet<U>
where
    U: UserDataPtr,
{
    fn drop(&mut self) {
        unsafe {
            drop(U::from_raw_const_ptr(
                self.raw().user_data.cast_const().cast(),
            ));
            drop(Box::from_raw(self.raw));
        }
    }
}

impl Operation {
    const CODE_RANGE: std::ops::RangeInclusive<u8> =
        sys::generated_safe::MIN_OPERATION_CODE..=sys::generated_safe::MAX_OPERATION_CODE;

    pub fn kind(self) -> OperationKind {
        if Self::CODE_RANGE.contains(&self.0) {
            // SAFETY: We checked if it's in range right above.
            unsafe { mem::transmute::<u8, OperationKind>(self.0) }
        } else {
            OperationKind::UnstableUncategorized
        }
    }

    pub fn code(self) -> u8 {
        self.0
    }
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_tuple("Operation");
        if Self::CODE_RANGE.contains(&self.0) {
            d.field(&self.kind());
        } else {
            d.field(&self.0);
        }
        d.finish()
    }
}

impl From<OperationKind> for Operation {
    /// Panics on hidden `OperationKind::UnstableUncategorized` variant.
    fn from(value: OperationKind) -> Self {
        let code = value as _;
        if !Self::CODE_RANGE.contains(&code) {
            panic!("OperationKind::{value:?}")
        }
        Operation(code)
    }
}
