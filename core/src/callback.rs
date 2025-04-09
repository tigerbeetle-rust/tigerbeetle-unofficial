use std::{
    marker::PhantomData,
    panic::catch_unwind,
    slice,
    time::{Duration, SystemTime},
};

use crate::util::RawConstPtr;

use super::Packet;

pub trait CallbacksPtr: RawConstPtr<Target = Self::Pointee> + callbacks_ptr::Sealed {
    type Pointee: Callbacks<UserDataPtr = Self::UserDataPtr> + Sized;
    type UserDataPtr: UserDataPtr<Pointee = Self::UserData>;
    type UserData: UserData;
}

impl<T> CallbacksPtr for T
where
    T: RawConstPtr,
    T::Target: Callbacks + Sized,
{
    type Pointee = T::Target;
    type UserDataPtr = <Self::Pointee as Callbacks>::UserDataPtr;
    type UserData = <Self::UserDataPtr as UserDataPtr>::Pointee;
}

mod callbacks_ptr {
    use super::{Callbacks, RawConstPtr};

    pub trait Sealed {}

    impl<T> Sealed for T
    where
        T: RawConstPtr,
        T::Target: Callbacks + Sized,
    {
    }
}

/// Reply returned by [`Callbacks`].
#[non_exhaustive]
pub struct Reply<'a> {
    /// Returned raw payload of this [`Reply`] as bytes.
    pub payload: &'a [u8],

    /// Cluster timestamp when the reply was generated.
    pub timestamp: SystemTime,
}

// `Self: Sync` because `F` is called from some Zig thread.
pub trait Callbacks: Sync {
    type UserDataPtr: UserDataPtr;

    /// Calls back once a [`Packet`] is submitted and processed.
    ///
    /// [`None`] `reply` means that submitting the [`Packet`] failed (check the [`Packet::status`]
    /// for the reason).
    fn completion(&self, packet: Packet<Self::UserDataPtr>, reply: Option<Reply<'_>>);
}

pub struct CallbacksFn<F, U>
where
    F: Fn(Packet<U>, Option<Reply<'_>>) + Sync,
    U: UserDataPtr,
{
    inner: F,
    _marker: PhantomData<fn(U)>,
}

impl<F, U> CallbacksFn<F, U>
where
    F: Fn(Packet<U>, Option<Reply<'_>>) + Sync,
    U: UserDataPtr,
{
    pub const fn new(inner: F) -> Self
    where
        F: Sync,
        U: UserDataPtr,
    {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<F, U> Callbacks for CallbacksFn<F, U>
where
    F: Fn(Packet<U>, Option<Reply<'_>>) + Sync,
    U: UserDataPtr,
{
    type UserDataPtr = U;

    fn completion(&self, packet: Packet<Self::UserDataPtr>, reply: Option<Reply<'_>>) {
        (self.inner)(packet, reply)
    }
}

pub const fn on_completion_fn<U, F>(f: F) -> CallbacksFn<F, U>
where
    F: Fn(Packet<U>, Option<Reply<'_>>) + Sync,
    U: UserDataPtr,
{
    CallbacksFn::new(f)
}

pub(crate) unsafe extern "C" fn completion_callback_raw_fn<F>(
    ctx: usize,
    packet: *mut sys::tb_packet_t,
    timestamp: u64,
    payload: *const u8,
    payload_size: u32,
) where
    F: Callbacks,
{
    let _ = catch_unwind(|| {
        let cb = &*sptr::from_exposed_addr::<F>(ctx);
        let payload_size = payload_size.try_into().expect(
            "at the start of calling `completion_callback`: \
             unable to convert `payload_size` from `u32` into `usize`",
        );
        let payload = if payload_size != 0 {
            slice::from_raw_parts(payload, payload_size)
        } else {
            &[]
        };
        dbg!((*packet).operation);
        let packet = Packet {
            raw: packet,
            _ptr: PhantomData,
        };
        let timestamp = SystemTime::UNIX_EPOCH + Duration::from_nanos(timestamp);
        cb.completion(packet, Some(Reply { payload, timestamp }))
    });
}

// `Self: Send` because we are sending user_data into the callback as an
// argument.
pub trait UserDataPtr: RawConstPtr<Target = Self::Pointee> + Send + user_data_ptr::Sealed {
    type Pointee: UserData;
}

impl<T> UserDataPtr for T
where
    T: RawConstPtr + Send,
    T::Target: UserData + Sized,
{
    type Pointee = T::Target;
}

mod user_data_ptr {
    use super::{RawConstPtr, UserData};

    pub trait Sealed {}

    impl<T> Sealed for T
    where
        T: RawConstPtr + Send,
        T::Target: UserData + Sized,
    {
    }
}

pub trait UserData {
    /// Borrow the data to send.
    fn data(&self) -> &[u8];
}
