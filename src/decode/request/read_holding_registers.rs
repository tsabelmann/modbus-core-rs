use crate::{FunctionCode, FunctionKind, PduData};

pub struct RequestDecoder {
    starting_address: u16,
    quantity_of_registers: u16
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DecoderError {
    NotEnoughData,
    InvalidQuantityOfRegisters,
    InvalidFunctionCode
}

impl RequestDecoder {
    pub fn new(pdu: &dyn PduData) -> Result<RequestDecoder, DecoderError> {
        let required_length = 5;
        if pdu.pdu_data().len() < required_length {
            Err(DecoderError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::ReadHoldingRegisters) => {
                    let data = [pdu.pdu_data()[1], pdu.pdu_data()[2]];
                    let starting_address = u16::from_be_bytes(data);

                    let data = [pdu.pdu_data()[3], pdu.pdu_data()[4]];
                    let quantity_of_registers = u16::from_be_bytes(data);
                    
                    // invalid number of registers
                    if (quantity_of_registers > 125) || (quantity_of_registers == 0) {
                        return Err(DecoderError::InvalidQuantityOfRegisters);
                    }

                    Ok(RequestDecoder { starting_address, quantity_of_registers })
                }
                _ => Err(DecoderError::InvalidFunctionCode)
            }
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        FunctionKind::Normal(FunctionCode::ReadHoldingRegisters)
    }

    pub fn starting_address(&self) -> u16 {
        self.starting_address
    }

    pub fn quantity_of_registers(&self) -> u16 {
        self.quantity_of_registers
    }
}
