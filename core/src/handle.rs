use std::{ffi::c_void, ptr};

use super::{
    callback::{Callbacks, UserDataPtr},
    packet, Packet,
};

pub struct ClientHandle<'a, U>
where
    U: UserDataPtr,
{
    pub(crate) raw: sys::tb_client_t,
    pub(crate) on_completion: &'a dyn Callbacks<UserDataPtr = U>,
}

unsafe impl<U> Send for ClientHandle<'_, U> where U: UserDataPtr {}
unsafe impl<U> Sync for ClientHandle<'_, U> where U: UserDataPtr {}

impl<U> Copy for ClientHandle<'_, U> where U: UserDataPtr {}

impl<U> Clone for ClientHandle<'_, U>
where
    U: UserDataPtr,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, U> ClientHandle<'a, U>
where
    U: UserDataPtr,
{
    pub fn acquire(self, user_data: U, operation: packet::Operation) -> Packet<'a, U> {
        Packet {
            raw: Box::into_raw(Box::new(sys::tb_packet_t {
                next: ptr::null_mut(),
                user_data: U::into_raw_const_ptr(user_data).cast::<c_void>().cast_mut(),
                operation: operation.0,
                status: 0,
                data_size: 0,
                data: ptr::null_mut(),
                batch_next: ptr::null_mut(),
                batch_tail: ptr::null_mut(),
                batch_size: 0,
                batch_allowed: 0,
                reserved: [0; 7],
            })),
            handle: self,
        }
    }
}
