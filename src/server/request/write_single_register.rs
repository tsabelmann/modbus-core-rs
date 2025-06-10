use crate::{FunctionCode, FunctionKind, PduData};

pub struct WriteSingleRegisterRequest<'a, T> 
where 
    T: PduData
{
    pdu: &'a T
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteSingleRegisterRequestError {
    NotEnoughData,
    InvalidFunctionCode
}

impl<'a, T: PduData> WriteSingleRegisterRequest<'a, T> {
    pub fn new(pdu: &'a T) -> Result<WriteSingleRegisterRequest<'a, T>, WriteSingleRegisterRequestError> {
        if pdu.pdu_data().len() < 5 {
            Err(WriteSingleRegisterRequestError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::WriteSingleRegister) => {
                    Ok(WriteSingleRegisterRequest { pdu })
                }
                _ => Err(WriteSingleRegisterRequestError::InvalidFunctionCode)
            }
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        self.pdu.function_code()
    }

    pub fn register_address(&self) -> u16 {
        let data = [self.pdu.pdu_data()[1], self.pdu.pdu_data()[2]];
        u16::from_be_bytes(data)
    }

    pub fn register_value(&self) -> u16 {
        let data = [self.pdu.pdu_data()[3], self.pdu.pdu_data()[4]];
        u16::from_be_bytes(data)
    }
}
