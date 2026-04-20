use crate::{ExceptionCode, FunctionKind, FunctionCode};
use crate::encode::FrameEncoder;

// ── EncodeError ──

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EncodeError {
    NotEnoughData,
    InvalidQuantityOfRegisters
}

// ── Response ──

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Response<'a> {
    Normal(&'a [u16]),
    Exception(ExceptionCode),
}

impl<'a> Response<'a> {
    pub const MAX_QUANTITY_OF_REGISTERS: usize = 125;

    pub const fn new(registers: &'a [u16]) -> Response<'a> {
        Response::Normal(registers)
    }

    pub const fn new_exception(code: ExceptionCode) -> Response<'a> {
        Response::Exception(code)
    }

    pub fn encode(&self, frame: &mut dyn FrameEncoder) -> Result<(), EncodeError> {
        match self {
            Response::Normal(registers) => {
                let byte_count = registers.len() * 2;

                if registers.is_empty() || registers.len() > Self::MAX_QUANTITY_OF_REGISTERS {
                    return Err(EncodeError::InvalidQuantityOfRegisters);
                }

                let payload = frame.payload_mut();
                if payload.len() < 1 + byte_count {
                    return Err(EncodeError::NotEnoughData);
                }

                // Byte count
                payload[0] = byte_count as u8;

                // Register values
                for (i, &value) in registers.iter().enumerate() {
                    let bytes = value.to_be_bytes();
                    payload[1 + i * 2] = bytes[0];
                    payload[1 + i * 2 + 1] = bytes[1];
                }

                frame.finalize(
                    FunctionKind::Normal(FunctionCode::ReadHoldingRegisters),
                    1 + byte_count,
                );
                Ok(())
            }
            Response::Exception(code) => {
                let payload = frame.payload_mut();
                if payload.is_empty() {
                    return Err(EncodeError::NotEnoughData);
                }

                let raw = u8::from(*code);
                payload[0] = raw;

                frame.finalize(
                    FunctionKind::Exception(FunctionCode::ReadHoldingRegisters),
                    1,
                );
                Ok(())
            }
        }
    }
}
