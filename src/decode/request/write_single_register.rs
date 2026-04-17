use crate::{FunctionCode, FunctionKind, PduData};

// ── DecoderError ──

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RequestError {
    NotEnoughData,
    InvalidFunctionCode
}

// ── RequestDecoder ──

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    register_address: u16,
    register_value: u16,
}

impl Request {
    pub fn new(pdu: &dyn PduData) -> Result<Request, RequestError> {
        if pdu.pdu_data().len() < 5 {
            return Err(RequestError::NotEnoughData);
        }

        match pdu.function_code() {
            FunctionKind::Normal(FunctionCode::WriteSingleRegister) => {
                let register_address = u16::from_be_bytes(
                    [pdu.pdu_data()[1], pdu.pdu_data()[2]]
                );
                let register_value = u16::from_be_bytes(
                    [pdu.pdu_data()[3], pdu.pdu_data()[4]]
                );
                Ok(Request { register_address, register_value })
            }
            _ => Err(RequestError::InvalidFunctionCode),
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        FunctionKind::Normal(FunctionCode::WriteSingleRegister)
    }

    pub fn register_address(&self) -> u16 {
        self.register_address
    }

    pub fn register_value(&self) -> u16 {
        self.register_value
    }
}
