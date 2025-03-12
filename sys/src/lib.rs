#![doc(
    html_logo_url = "https://avatars.githubusercontent.com/u/187310527",
    html_favicon_url = "https://avatars.githubusercontent.com/u/187310527?s=256"
)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
                1_u128.to_ne_bytes().as_ptr(),
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
                1_u128.to_ne_bytes().as_ptr(),
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
            let mut client = mem::zeroed();
            _ = crate::tb_client_completion_context(&mut client, ptr::null_mut());
        }
    }

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_submit() {
        unsafe {
            let mut client = mem::zeroed();
            _ = crate::tb_client_submit(&mut client, ptr::null_mut());
        }
    }

    #[test]
    #[ignore = "only checks linkage"]
    fn tb_client_deinit() {
        unsafe {
            let mut client = mem::zeroed();
            _ = crate::tb_client_deinit(&mut client);
        }
    }
}
