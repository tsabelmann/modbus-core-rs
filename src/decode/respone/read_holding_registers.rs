use crate::{FunctionCode, FunctionKind, PduData};

pub struct ReadHoldingRegistersResponseDecoder<'a, T> 
where 
    T: PduData
{
    pdu: &'a T
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadHoldingRegistersResponseDecoderError {
    NotEnoughData,
    InvalidQuanitityOfRegisters,
    InvalidFunctionCode,
    CrcError
}

impl<'a, T: PduData> ReadHoldingRegistersResponseDecoder<'a, T> {
    pub fn new(pdu: &'a T) -> Result<ReadHoldingRegistersResponseDecoder<'a, T>, ReadHoldingRegistersResponseDecoderError> {
        let required_length = 4;
        if pdu.pdu_data().len() < required_length {
            Err(ReadHoldingRegistersResponseDecoderError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::ReadHoldingRegisters) => {
                    let quantity_of_registers = pdu.pdu_data()[1];
                    let byte_count = (quantity_of_registers as usize) * 2;
                    let required_length = 2 + byte_count;
                    
                    // Not enough data to provide
                    if pdu.pdu_data().len() < required_length {
                        return Err(ReadHoldingRegistersResponseDecoderError::NotEnoughData);
                    }
                    
                    // invalid number of registers
                    if (quantity_of_registers > 125) || (quantity_of_registers == 0) {
                        return Err(ReadHoldingRegistersResponseDecoderError::InvalidQuanitityOfRegisters);
                    }

                    Ok(ReadHoldingRegistersResponseDecoder { pdu })
                }
                _ => Err(ReadHoldingRegistersResponseDecoderError::InvalidFunctionCode)
            }
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        self.pdu.function_code()
    }

    pub fn byte_count(&self) -> u16 {
        (self.pdu.pdu_data()[1] as u16) * 2 
    }
}
