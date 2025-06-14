#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

/// Modbus related constants for frame size, number of registers, etc.
pub mod constants;

// pub mod decode;

pub mod encode;

/// Register related types and helper functions.
pub mod register;

/// Function code related types.
mod codes;
pub use codes::{FunctionCode, FunctionKind};

/// Exception code related types and helper functions.
mod exception;
pub use exception::ExceptionCode;

/// PDU related types.
mod pdu;
pub use pdu::{PduData, PduDataMut};

/// TCP/IP related types and helper functions.
pub mod tcp;
