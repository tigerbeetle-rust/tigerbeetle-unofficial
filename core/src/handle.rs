use super::callback::{Callbacks, UserDataPtr};

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
