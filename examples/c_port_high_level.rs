use std::time::{Duration, Instant};

use tigerbeetle_unofficial as tb;

// config.message_size_max - @sizeOf(vsr.Header):
const MAX_MESSAGE_BYTE_SIZE: usize = (1024 * 1024) - 256;

// Crate is runtime agnostic, so you can use tokio or any other async runtime
#[pollster::main]
async fn main() {
    println!("TigerBeetle C Sample");
    println!("Connecting...");
    let address = std::env::var("TB_ADDRESS");
    let address = address.as_deref().unwrap_or("3000");
    let client = tb::Client::new(0, address).expect("creating a tigerbeetle client");

    ////////////////////////////////////////////////////////////
    // Submitting a batch of accounts:                        //
    ////////////////////////////////////////////////////////////

    println!("Creating accounts...");
    let accounts = [
        tb::Account::new(1, 777, 2).with_user_data_32(1),
        tb::Account::new(2, 777, 2),
    ];
    client
        .create_accounts(accounts.to_vec())
        .await
        .expect("creating accounts");
    println!("Accounts created successfully");

    ////////////////////////////////////////////////////////////
    // Submitting multiple batches of transfers:              //
    ////////////////////////////////////////////////////////////

    println!("Creating transfers...");
    const MAX_BATCHES: usize = 100;
    const TRANSFERS_PER_BATCH: usize =
        MAX_MESSAGE_BYTE_SIZE / std::mem::size_of::<tb::Transfer>() - 1;
    let max_batches = std::env::var("TIGERBEETLE_RS_MAX_BATCHES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(MAX_BATCHES);
    let mut max_latency = Duration::ZERO;
    let mut total_time = Duration::ZERO;

    for i in 0..max_batches {
        let transfers: Vec<_> = (0..TRANSFERS_PER_BATCH)
            .map(|j| {
                tb::Transfer::new((j + 1 + i * TRANSFERS_PER_BATCH).try_into().unwrap())
                    .with_debit_account_id(accounts[0].id())
                    .with_credit_account_id(accounts[1].id())
                    .with_code(2)
                    .with_ledger(777)
                    .with_amount(1)
                    .with_user_data_32(1)
            })
            .collect();

        let start = Instant::now();
        client
            .create_transfers(transfers)
            .await
            .expect("creating transfers");

        let elapsed = start.elapsed();
        max_latency = max_latency.max(elapsed);
        total_time += elapsed;
    }

    println!("Transfers created successfully");
    println!("============================================");
    println!(
        "{:.0} transfers per second",
        (max_batches * TRANSFERS_PER_BATCH) as f64 / total_time.as_secs_f64()
    );
    println!(
        "create_transfers max p100 latency per {} transfers = {}ms",
        TRANSFERS_PER_BATCH,
        max_latency.as_millis()
    );
    println!(
        "total {} transfers in {}ms",
        max_batches * TRANSFERS_PER_BATCH,
        total_time.as_millis()
    );
    println!();

    ////////////////////////////////////////////////////////////
    // Looking up accounts:                                   //
    ////////////////////////////////////////////////////////////

    println!("Looking up accounts ...");
    let ids = accounts.map(|a| a.id());
    let accounts = client
        .lookup_accounts(ids.to_vec())
        .await
        .expect("looking up accounts");
    assert!(!accounts.is_empty());
    println!("{} Account(s) found", accounts.len());
    println!("============================================");
    for account in accounts {
        println!(
            "Account {{ id: {}, debits_posted: {}, credits_posted: {}, .. }}",
            account.id(),
            account.debits_posted(),
            account.credits_posted()
        );
    }

    ////////////////////////////////////////////////////////////
    // Querying accounts:                                     //
    ////////////////////////////////////////////////////////////

    println!("Querying accounts ...");
    let accounts = client
        .query_accounts(Box::new(
            tb::QueryFilter::new(u32::MAX).with_user_data_32(1),
        ))
        .await
        .expect("querying accounts");
    assert!(!accounts.is_empty());
    println!("{} Account(s) found", accounts.len());
    println!("============================================");
    for account in accounts {
        println!(
            "Account {{ id: {}, debits_posted: {}, credits_posted: {}, .. }}",
            account.id(),
            account.debits_posted(),
            account.credits_posted()
        );
    }

    ////////////////////////////////////////////////////////////
    // Querying transfers:                                    //
    ////////////////////////////////////////////////////////////

    println!("Querying transfers ...");
    let transfers = client
        .query_transfers(Box::new(
            tb::QueryFilter::new(u32::MAX).with_user_data_32(1),
        ))
        .await
        .expect("querying transfers");
    assert!(!transfers.is_empty());
    println!("{} Transfer(s) found", transfers.len());
    println!("============================================");
    for transfer in transfers {
        println!(
            "Transfer {{ id: {}, debit_account_id: {}, credit_account_id: {}, amount: {}, .. }}",
            transfer.id(),
            transfer.debit_account_id(),
            transfer.credit_account_id(),
            transfer.amount(),
        );
    }
}
