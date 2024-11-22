#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // u128

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Available only with `generated-safe` feature
#[cfg(feature = "generated-safe")]
#[allow(clippy::unnecessary_cast, clippy::assign_op_pattern)]
#[doc(hidden)]
pub mod generated_safe {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

#[cfg(test)]
mod linked {
    //! Bunch of dummies to ensure eagerly that everything is linked properly.
    //!
    //! Executing them will definitely cause undefined behaviour, so they're ignored, which is
    //! enough to check the linker does its job.

    use std::{mem, ptr};

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_init() {
        unsafe {
            let mut raw = mem::zeroed();
            let address = "3000".as_bytes();
            _ = crate::tb_client_init(
                &mut raw,
                1,
                address.as_ptr().cast(),
                address.len().try_into().unwrap(),
                ptr::null::<()>() as usize,
                None,
            );
        }
    }

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_init_echo() {
        unsafe {
            let mut raw = mem::zeroed();
            let address = "3000".as_bytes();
            _ = crate::tb_client_init_echo(
                &mut raw,
                1,
                address.as_ptr().cast(),
                address.len().try_into().unwrap(),
                ptr::null::<()>() as usize,
                None,
            );
        }
    }

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_completion_context() {
        unsafe {
            let client = mem::zeroed();
            _ = crate::tb_client_completion_context(client);
        }
    }

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_submit() {
        unsafe {
            let client = mem::zeroed();
            _ = crate::tb_client_submit(client, ptr::null_mut());
        }
    }

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_deinit() {
        unsafe {
            let client = mem::zeroed();
            _ = crate::tb_client_deinit(client);
        }
    }
}
