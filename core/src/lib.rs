#![doc(
    html_logo_url = "https://avatars.githubusercontent.com/u/187310527",
    html_favicon_url = "https://avatars.githubusercontent.com/u/187310527?s=256"
)]
#![warn(
    clippy::match_wildcard_for_single_variants,
    clippy::wildcard_enum_match_arm
)]

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

type CompletionCallbackRawFn =
    unsafe extern "C" fn(usize, *mut sys::tb_packet_t, u64, *const u8, u32);

pub struct Client<F>
where
    F: CallbacksPtr,
{
    raw: sys::tb_client_t,
    cb: *const F::Target,
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
        completion_callback: F,
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
        unsafe { Client::with_callback_unchecked(cluster_id, address, completion_callback) }
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
        completion_callback: F,
    ) -> Result<Self, NewClientError>
    where
        A: AsRef<[u8]>,
    {
        let completion_fn = completion_callback_raw_fn::<F::Target>;
        let completion_cb = F::into_raw_const_ptr(completion_callback);
        let completion_ctx = sptr::Strict::expose_addr(completion_cb);

        unsafe fn raw_with_callback(
            cluster_id: u128,
            address: &[u8],
            completion_ctx: usize,
            completion_callback: CompletionCallbackRawFn,
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
                completion_ctx,
                Some(completion_callback),
            );

            // SAFETY: Unwrapping is OK here, because the returned `TB_INIT_STATUS` is actually an
            //         enum with positive discriminant undoubtedly fitting into `u32`.
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
                    completion_ctx,
                    completion_fn,
                ) {
                    Ok(x) => x,
                    Err(e) => {
                        F::from_raw_const_ptr(completion_cb);
                        return Err(e);
                    }
                }
            },
            cb: completion_cb,
            marker: PhantomData,
        })
    }

    pub fn handle(&self) -> ClientHandle<'_, F::UserDataPtr> {
        ClientHandle {
            raw: self.raw,
            cb: unsafe { &*self.cb },
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
                _ = tokio::task::block_in_place(|| sys::tb_client_deinit(&mut self.raw));
            } else {
                _ = sys::tb_client_deinit(&mut self.raw);
            }
            #[cfg(not(feature = "tokio-rt-multi-thread"))]
            {
                _ = sys::tb_client_deinit(&mut self.raw);
            }
            F::from_raw_const_ptr(self.cb);
        }
    }
}
