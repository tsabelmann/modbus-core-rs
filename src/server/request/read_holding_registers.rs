use crate::{FunctionCode, FunctionKind, PduData};

pub struct ReadHoldingRegistersRequest<'a, T> 
where 
    T: PduData
{
    pdu: &'a T
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadHoldingRegistersRequestError {
    NotEnoughData,
    InvalidQuanitityOfRegisters,
    InvalidFunctionCode
}

impl<'a, T: PduData> ReadHoldingRegistersRequest<'a, T> {
    pub fn new(pdu: &'a T) -> Result<ReadHoldingRegistersRequest<'a, T>, ReadHoldingRegistersRequestError> {
        if pdu.pdu_data().len() < 5 {
            Err(ReadHoldingRegistersRequestError::NotEnoughData)
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
                        return Err(ReadHoldingRegistersRequestError::InvalidQuanitityOfRegisters);
                    }

                    // to many registers to read
                    if 0xFFFF - quantity_of_registers < starting_address {
                        return Err(ReadHoldingRegistersRequestError::InvalidQuanitityOfRegisters);
                    }

                    Ok(ReadHoldingRegistersRequest { pdu })
                }
                _ => Err(ReadHoldingRegistersRequestError::InvalidFunctionCode)
            }
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        self.pdu.function_code()
    }

    pub fn starting_address(&self) -> u16 {
        let data = [self.pdu.pdu_data()[1], self.pdu.pdu_data()[2]];
        u16::from_be_bytes(data)
    }

    pub fn quantity_of_registers(&self) -> u16 {
        let data = [self.pdu.pdu_data()[3], self.pdu.pdu_data()[4]];
        u16::from_be_bytes(data)
    }
}
