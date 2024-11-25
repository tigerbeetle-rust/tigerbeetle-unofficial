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
    pub fn packet(self, user_data: U, operation: packet::Operation) -> Packet<'a, U> {
        Packet::new(self, user_data, operation)
    }
}
