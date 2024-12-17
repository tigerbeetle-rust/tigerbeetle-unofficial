use rand::RngCore;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as u64
}

fn generate_random_bytes() -> [u8; 10] {
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 10];
    rng.fill_bytes(&mut random_bytes);
    random_bytes
}

/// Generate a new unique ID suitable for use as an ID in TigerBeetle.
///
/// https://docs.tigerbeetle.com/coding/data-modeling/#id
pub fn id() -> u128 {
    let timestamp = get_current_timestamp();

    let millis = &timestamp.to_le_bytes()[..6];
    let random = generate_random_bytes();

    let mut id = [0u8; 16];
    id[0..10].copy_from_slice(&random);
    id[10..16].copy_from_slice(millis);
    // SAFETY: `id` is 16 bytes long
    u128::from_le_bytes(id[..16].try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn smoke() {
        let id1 = id();
        let id2 = id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn smoke_order() {
        let id1 = id();
        std::thread::sleep(Duration::from_millis(2));
        let id2 = id();
        assert!(dbg!(id1) < dbg!(id2));
    }
}
