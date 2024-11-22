#![forbid(unsafe_code)]

mod reply;

use error::NewClientError;
use reply::Reply;
use tokio::sync::oneshot;

use core::{
    error::{CreateAccountsError, CreateTransfersError, SendError},
    util::{RawConstPtr, SendAsBytesOwnedSlice, SendOwnedSlice},
};

pub use core::{self, account, error, transfer, Account, Packet, Transfer};

pub struct Client {
    inner: core::Client<&'static Callbacks>,
}

struct Callbacks;

struct UserData {
    reply_sender: oneshot::Sender<Result<Reply, SendError>>,
    data: SendAsBytesOwnedSlice,
}

impl Client {
    pub fn new<A>(cluster_id: u128, address: A) -> Result<Self, NewClientError>
    where
        A: AsRef<[u8]>,
    {
        Ok(Client {
            inner: core::Client::with_callback(cluster_id, address, &Callbacks)?,
        })
    }

    pub async fn create_accounts<T>(&self, accounts: T) -> Result<(), CreateAccountsError>
    where
        T: Into<SendOwnedSlice<Account>>,
    {
        let accounts: SendOwnedSlice<Account> = accounts.into();
        if accounts.is_empty() {
            return Ok(());
        }
        Ok(self
            .submit(
                accounts.into_as_bytes(),
                core::OperationKind::CreateAccounts.into(),
            )
            .await?
            .into_create_accounts()?)
    }

    pub async fn create_transfers<T>(&self, transfers: T) -> Result<(), CreateTransfersError>
    where
        T: Into<SendOwnedSlice<Transfer>>,
    {
        let transfers: SendOwnedSlice<Transfer> = transfers.into();
        if transfers.is_empty() {
            return Ok(());
        }
        Ok(self
            .submit(
                transfers.into_as_bytes(),
                core::OperationKind::CreateTransfers.into(),
            )
            .await?
            .into_create_transfers()?)
    }

    pub async fn get_account_balances<T>(
        &self,
        filter: T,
    ) -> Result<Vec<account::Balance>, SendError>
    where
        T: RawConstPtr<Target = account::Filter> + Send + 'static,
    {
        let filter: SendOwnedSlice<account::Filter> = SendOwnedSlice::from_single(filter);
        self.submit(
            filter.into_as_bytes(),
            core::OperationKind::GetAccountBalances.into(),
        )
        .await
        .map(Reply::into_get_account_balances)
    }

    pub async fn get_account_transfers<T>(&self, filter: T) -> Result<Vec<Transfer>, SendError>
    where
        T: RawConstPtr<Target = account::Filter> + Send + 'static,
    {
        let filter: SendOwnedSlice<account::Filter> = SendOwnedSlice::from_single(filter);
        self.submit(
            filter.into_as_bytes(),
            core::OperationKind::GetAccountTransfers.into(),
        )
        .await
        .map(Reply::into_get_account_transfers)
    }

    pub async fn lookup_accounts<T>(&self, ids: T) -> Result<Vec<Account>, SendError>
    where
        T: Into<SendOwnedSlice<u128>>,
    {
        let ids: SendOwnedSlice<u128> = ids.into();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        self.submit(
            ids.into_as_bytes(),
            core::OperationKind::LookupAccounts.into(),
        )
        .await
        .map(Reply::into_lookup_accounts)
    }

    pub async fn lookup_transfers<T>(&self, ids: T) -> Result<Vec<Transfer>, SendError>
    where
        T: Into<SendOwnedSlice<u128>>,
    {
        let ids: SendOwnedSlice<u128> = ids.into();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        self.submit(
            ids.into_as_bytes(),
            core::OperationKind::LookupTransfers.into(),
        )
        .await
        .map(Reply::into_lookup_transfers)
    }

    async fn submit(
        &self,
        data: SendAsBytesOwnedSlice,
        operation: core::Operation,
    ) -> Result<Reply, SendError> {
        let (reply_sender, reply_receiver) = oneshot::channel();
        let user_data = Box::new(UserData { reply_sender, data });
        self.inner.packet(user_data, operation).submit();
        reply_receiver.await.unwrap()
    }
}

impl core::Callbacks for Callbacks {
    type UserDataPtr = Box<UserData>;

    fn on_completion(&self, packet: core::Packet<'_, Self::UserDataPtr>, payload: &[u8]) {
        let status = packet.status();
        let operation = packet.operation();
        let user_data = packet.into_user_data();
        user_data
            .reply_sender
            .send(status.map(|()| Reply::copy_from_reply(operation.kind(), payload)))
            .unwrap_or_else(|_| panic!("Unexpected: reply receiver is already dropped"));
    }
}

impl core::UserData for UserData {
    fn data(&self) -> &[u8] {
        self.data.as_ref()
    }
}

fn _test_thread_safe(
    client: Client,
    accounts: Vec<Account>,
    transfers: Vec<Transfer>,
    ids: Vec<u128>,
) {
    check_thread_safe(async move {
        client.create_accounts(accounts).await.unwrap();
        client.lookup_accounts(ids.clone()).await.unwrap();
        client.create_transfers(transfers).await.unwrap();
        client.lookup_transfers(ids).await.unwrap();
    });

    fn check_thread_safe<T>(_: T)
    where
        T: Send + Sync + 'static,
    {
    }
}
