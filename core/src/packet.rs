use std::{fmt, mem, num::NonZeroU8};

use crate::{
    callback::{UserData, UserDataPtr},
    error::{SendError, SendErrorKind},
    ClientHandle,
};

pub use sys::generated_safe::OperationKind;

pub struct Packet<'a, U>
where
    U: UserDataPtr,
{
    pub(super) raw: *mut sys::tb_packet_t,
    pub(super) handle: ClientHandle<'a, U>,
}

impl<'a, U> Packet<'a, U>
where
    U: UserDataPtr,
{
    pub fn submit(mut self) {
        let data = self.user_data().data();
        let Ok(data_size) = data.len().try_into() else {
            self.set_status(Err(SendErrorKind::TooMuchData.into()));
            self.handle.on_completion.on_completion(self, &[]);
            return;
        };
        let data = data.as_ptr();

        let raw = self.raw_mut();
        raw.data_size = data_size;
        raw.data = data.cast_mut().cast();

        unsafe { sys::tb_client_submit(self.handle.raw, self.raw) };
        mem::forget(self);
    }

    fn raw(&self) -> &sys::tb_packet_t {
        unsafe { &*self.raw }
    }

    fn raw_mut(&mut self) -> &mut sys::tb_packet_t {
        unsafe { &mut *self.raw }
    }

    pub fn into_user_data(self) -> U {
        let this = mem::ManuallyDrop::new(self);
        let user_data;
        unsafe {
            user_data = U::from_raw_const_ptr(this.raw().user_data.cast_const().cast());
            sys::tb_client_release_packet(this.handle.raw, this.raw);
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

    pub fn client_handle(&self) -> ClientHandle<'a, U> {
        self.handle
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

impl<U> Drop for Packet<'_, U>
where
    U: UserDataPtr,
{
    fn drop(&mut self) {
        unsafe {
            U::from_raw_const_ptr(self.raw().user_data.cast_const().cast());
            sys::tb_client_release_packet(self.handle.raw, self.raw);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Operation(pub(crate) u8);

impl Operation {
    const CODE_RANGE: std::ops::RangeInclusive<u8> =
        sys::generated_safe::MIN_OPERATION_CODE..=sys::generated_safe::MAX_OPERATION_CODE;

    pub fn kind(self) -> OperationKind {
        if Self::CODE_RANGE.contains(&self.0) {
            // SAFETY: We checked if it's in range right above
            unsafe { mem::transmute(self.0) }
        } else {
            OperationKind::UnstableUncategorized
        }
    }

    pub fn code(self) -> u8 {
        self.0
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("Operation");
        let kind = self.kind();
        if matches!(kind, OperationKind::UnstableUncategorized) {
            d.field(&self.0);
        } else {
            d.field(&kind);
        }
        d.finish()
    }
}

impl From<OperationKind> for Operation {
    /// Constructs a new [`Operation`] from [`OperationKind`].
    ///
    /// # Panics
    ///
    /// Panics on hidden `OperationKind::UnstableUncategorized` variant.
    fn from(value: OperationKind) -> Self {
        let this = Self(value as _);
        if matches!(this.kind(), OperationKind::UnstableUncategorized) {
            panic!("OperationKind::{value:?}")
        }
        this
    }
}

unsafe impl<U> Sync for Packet<'_, U>
where
    U: UserDataPtr,
    U::Pointee: Sync,
{
}
unsafe impl<U> Send for Packet<'_, U> where U: UserDataPtr {}
