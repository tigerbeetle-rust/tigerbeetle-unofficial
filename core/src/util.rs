//! Helpful abstractions to generalize over various types

mod owned_slice;
mod raw_const_ptr;
pub mod send_marker;

pub use self::{
    owned_slice::{AsBytesOwnedSlice, Erased, OwnedSlice, SendAsBytesOwnedSlice, SendOwnedSlice},
    raw_const_ptr::RawConstPtr,
    send_marker::SendMarker,
};
