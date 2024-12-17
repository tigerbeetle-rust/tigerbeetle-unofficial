pub mod account;
mod callback;
pub mod error;
mod handle;
mod packet;
pub mod query_filter;
pub mod transfer;
pub mod util;

use std::{marker::PhantomData, mem, num::NonZeroU32};

use error::{NewClientError, NewClientErrorKind};

pub use account::Account;
pub use callback::*;
pub use handle::ClientHandle;
pub use packet::*;
pub use query_filter::QueryFilter;
pub use transfer::Transfer;

type OnCompletionRawFn =
    unsafe extern "C" fn(usize, sys::tb_client_t, *mut sys::tb_packet_t, u64, *const u8, u32);

pub struct Client<F>
where
    F: CallbacksPtr,
{
    raw: sys::tb_client_t,
    on_completion: *const F::Target,
    marker: PhantomData<F>,
}

unsafe impl<F> Send for Client<F> where F: CallbacksPtr + Send {}
unsafe impl<F> Sync for Client<F> where F: CallbacksPtr {}

impl<F> Client<F>
where
    F: CallbacksPtr,
{
    pub fn with_callback<A>(
        cluster_id: u128,
        address: A,
        on_completion: F,
    ) -> Result<Self, NewClientError>
    where
        A: AsRef<[u8]>,
        // `F` and `UserDataPtr` are `'static`, because we can `mem::forget(self)`
        // and drop anything that is being referred from `F` or `UserDataPtr`,
        // thus invalidating callback or user data.
        F: 'static,
        F::UserDataPtr: 'static,
    {
        // SAFETY: `F` and `UserDataPtr` are `'static`.
        unsafe { Client::with_callback_unchecked(cluster_id, address, on_completion) }
    }

    /// Highly unsafe method. Please use [`Self::with_callback`]
    /// unless you are *really sure* you are doing it right.
    ///
    /// # Safety
    ///
    /// `F` and `U` are unresticted by any lifetime. It's user's responsibility
    /// to ensure validity of `on_completion` callback or packet's `user_data`
    /// for client's use. If client is dropped, you can safely invalidate these
    /// things.
    pub unsafe fn with_callback_unchecked<A>(
        cluster_id: u128,
        address: A,
        on_completion: F,
    ) -> Result<Self, NewClientError>
    where
        A: AsRef<[u8]>,
    {
        let on_completion_fn = callback::on_completion_raw_fn::<F::Target>;
        let on_completion = F::into_raw_const_ptr(on_completion);
        let on_completion_ctx = sptr::Strict::expose_addr(on_completion);

        unsafe fn raw_with_callback(
            cluster_id: u128,
            address: &[u8],
            on_completion_ctx: usize,
            on_completion_fn: OnCompletionRawFn,
        ) -> Result<sys::tb_client_t, NewClientError> {
            let mut raw = mem::zeroed();
            let status = sys::tb_client_init(
                &mut raw,
                cluster_id.to_ne_bytes().as_ptr(),
                address.as_ptr().cast(),
                address
                    .len()
                    .try_into()
                    .map_err(|_| NewClientErrorKind::AddressInvalid)?,
                on_completion_ctx,
                Some(on_completion_fn),
            );

            // SAFETY: Unwrapping is OK here, because the returned `TB_STATUS` is actually an enum
            //         with positive discriminant undoubtedly fitting into `u32`.
            #[allow(clippy::useless_conversion)] // not true for Windows
            if let Some(c) = NonZeroU32::new(status.try_into().unwrap_unchecked()) {
                Err(NewClientError(c))
            } else {
                Ok(raw)
            }
        }

        Ok(Client {
            raw: unsafe {
                match raw_with_callback(
                    cluster_id,
                    address.as_ref(),
                    on_completion_ctx,
                    on_completion_fn,
                ) {
                    Ok(x) => x,
                    Err(err) => {
                        F::from_raw_const_ptr(on_completion);
                        return Err(err);
                    }
                }
            },
            on_completion,
            marker: PhantomData,
        })
    }

    pub fn handle(&self) -> ClientHandle<'_, F::UserDataPtr> {
        ClientHandle {
            raw: self.raw,
            on_completion: unsafe { &*self.on_completion },
        }
    }

    pub fn packet(
        &self,
        user_data: F::UserDataPtr,
        operation: impl Into<packet::Operation>,
    ) -> Packet<'_, F::UserDataPtr> {
        self.handle().packet(user_data, operation)
    }
}

/// Blocks until all pending requests finish
impl<F> Drop for Client<F>
where
    F: CallbacksPtr,
{
    fn drop(&mut self) {
        unsafe {
            #[cfg(feature = "tokio-rt-multi-thread")]
            if tokio::runtime::Handle::try_current().is_ok_and(|h| {
                matches!(
                    h.runtime_flavor(),
                    tokio::runtime::RuntimeFlavor::MultiThread
                )
            }) {
                tokio::task::block_in_place(|| sys::tb_client_deinit(self.raw));
            } else {
                sys::tb_client_deinit(self.raw)
            }
            #[cfg(not(feature = "tokio-rt-multi-thread"))]
            sys::tb_client_deinit(self.raw);
            F::from_raw_const_ptr(self.on_completion);
        }
    }
}
