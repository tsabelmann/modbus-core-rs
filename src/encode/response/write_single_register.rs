use crate::{ExceptionCode, FunctionCode, FunctionKind};
use crate::encode::FrameEncoder;

// ── EncodeError ──

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EncodeError {
    NotEnoughData
}

// ── Response ──

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Response {
    Normal {
        register_address: u16,
        register_value: u16
    },
    Exception(ExceptionCode)
}


impl Response {
    pub const MAX_PAYLOAD_LENGTH: usize = 4;

    pub const fn new(register_address: u16, register_value: u16) -> Response {
        Response::Normal { register_address, register_value }
    }

    pub const fn new_exception(code: ExceptionCode) -> Response {
        Response::Exception(code)
    }

    pub fn encode(&self, frame: &mut dyn FrameEncoder) -> Result<(), EncodeError> {
        match self {
            Response::Normal { register_address, register_value } => {
                let payload = frame.payload_mut();
                if payload.len() < Self::MAX_PAYLOAD_LENGTH {
                    return Err(EncodeError::NotEnoughData);
                }

                // register address
                let bytes = register_address.to_be_bytes();
                payload[0] = bytes[0];
                payload[1] = bytes[1];

                // register value
                let bytes = register_value.to_be_bytes();
                payload[2] = bytes[0];
                payload[3] = bytes[1];

                frame.finalize(
                    FunctionKind::Normal(FunctionCode::WriteSingleRegister),
                    Self::MAX_PAYLOAD_LENGTH,
                );
                Ok(())
            },
            Response::Exception(code) => {
                let payload = frame.payload_mut();
                if payload.is_empty() {
                    return Err(EncodeError::NotEnoughData);
                }

                let raw = u8::from(*code);
                payload[0] = raw;

                frame.finalize(
                    FunctionKind::Exception(FunctionCode::WriteSingleRegister),
                    1,
                );
                Ok(())
            },
        }
    }
}
