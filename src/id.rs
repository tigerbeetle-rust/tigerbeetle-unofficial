//! [TigerBeetle Time-Based Identifier][0] implementation.
//!
//! [0]: https://docs.tigerbeetle.com/coding/data-modeling#tigerbeetle-time-based-identifiers-recommended

use std::{sync::Mutex, time::UNIX_EPOCH};

/// Returns the current timestamp in milliseconds since [`UNIX_EPOCH`].
///
/// # Panics
///
/// - If the [`SystemTime`] clock went backwards beyond [`UNIX_EPOCH`].
/// - If milliseconds since [`UNIX_EPOCH`] overflow [`u64`].
fn get_current_timestamp() -> u64 {
    UNIX_EPOCH
        .elapsed()
        .unwrap_or_else(|e| panic!("`SystemTime` went backwards beyond `UNIX_EPOCH`: {e}"))
        .as_millis()
        .try_into()
        .unwrap_or_else(|e| panic!("milliseconds since `UNIX_EPOCH` overflow `u64`: {e}"))
}

/// Generates and returns 10 random bytes.
fn generate_random_bytes() -> [u8; 10] {
    let mut bytes = [0u8; 10];
    fastrand::fill(&mut bytes);
    bytes
}

/// Generates a new [TigerBeetle Time-Based Identifier][0].
///
/// [TigerBeetle Time-Based Identifier][0] consists of:
/// - 48 bits of (millisecond) timestamp (high-order bits)
/// - 80 bits of randomness (low-order bits)
///
/// [0]: https://docs.tigerbeetle.com/coding/data-modeling#tigerbeetle-time-based-identifiers-recommended
#[must_use]
pub fn id() -> u128 {
    static LAST: Mutex<(u64, [u8; 10])> = Mutex::new((0, [0; 10]));

    let (timestamp, random) = {
        let timestamp = get_current_timestamp();

        // Lock the `Mutex` to ensure that `last_timestamp` is monotonically increasing and
        // `last_random` changes each millisecond.
        let (last_timestamp, last_random) = &mut *LAST.lock().unwrap();
        if timestamp > *last_timestamp {
            *last_timestamp = timestamp;
            *last_random = generate_random_bytes();
        }

        // Read out a `u80` from the `last_random` as a `u64` and `u16`.
        // PANIC: Unwrapping is OK here, since `mem::size_of<u64>() == 8` and
        //        `mem::size_of<u16>() == 2`.
        let mut random_lo = u64::from_le_bytes(last_random[..8].try_into().unwrap());
        let mut random_hi = u16::from_le_bytes(last_random[8..].try_into().unwrap());

        // Increment the random bits as a `u80` together, checking for overflow.
        random_lo = random_lo.wrapping_add(1);
        if random_lo == 0 {
            random_hi = random_hi.wrapping_add(1);
            if random_hi == 0 {
                *last_timestamp = last_timestamp.wrapping_add(1);
            }
        }

        // Write incremented `u80` back to the `last_random`.
        last_random[..8].copy_from_slice(&random_lo.to_le_bytes());
        last_random[8..].copy_from_slice(&random_hi.to_le_bytes());

        (*last_timestamp, *last_random)
    };

    // Create `u128` from new `timestamp` and `random`.
    let mut id = [0u8; 16];
    id[0..10].copy_from_slice(&random);
    id[10..16].copy_from_slice(&timestamp.to_le_bytes()[..6]);
    u128::from_le_bytes(id)
}

#[cfg(test)]
mod id_spec {
    use std::{sync::Barrier, thread, time::Duration};

    use super::id;

    #[test]
    fn unique() {
        let id1 = id();
        let id2 = id();
        assert_ne!(id1, id2, "expected: {id1} != {id2}");
    }

    #[test]
    fn monotonic_between_millis() {
        let id1 = id();
        thread::sleep(Duration::from_millis(2));
        let id2 = id();
        assert!(id1 < id2, "expected: {id1} < {id2}");
    }

    #[test]
    fn monotonic_within_millis() {
        let id1 = id();
        thread::sleep(Duration::from_micros(1));
        let id2 = id();
        assert!(id1 < id2, "expected: {id1} < {id2}");
    }

    #[test]
    fn monotonic_immediately() {
        let id1 = id();
        let id2 = id();
        assert!(id1 < id2, "expected: {id1} < {id2}");
    }

    // Port of upstream test:
    // https://github.com/tigerbeetle/tigerbeetle/blob/0.16.64/src/clients/go/pkg/types/main_test.go#L92-L132
    #[test]
    fn monotonic_fuzz() {
        fn verifier() {
            let mut id1 = id();
            for i in 0..1_000_000 {
                if i % 1_000 == 0 {
                    thread::sleep(Duration::from_millis(1));
                }
                let id2 = id();

                assert!(id1 < id2, "expected: {id1} < {id2}");

                id1 = id2;
            }
        }

        // Verify monotonic IDs locally.
        verifier();

        // Verify monotonic IDs across multiple threads.
        let n = 10;
        let barrier = Barrier::new(n);
        thread::scope(|s| {
            let threads = (0..n)
                .map(|i| {
                    thread::Builder::new()
                        .name(i.to_string())
                        .spawn_scoped(s, || {
                            // Sync up all threads before `verifier()` to maximize contention.
                            barrier.wait();
                            verifier();
                        })
                        .unwrap()
                })
                .collect::<Vec<_>>();
            for t in threads {
                t.join().unwrap();
            }
        });
    }
}
