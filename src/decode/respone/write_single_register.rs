use crate::{FunctionCode, FunctionKind, PduData};

pub struct WriteSingleRegisterReponseDecoder<'a, T>
where 
    T: PduData
{
    pdu: &'a T
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteSingleRegisterReponseDecoderError {
    NotEnoughData,
    InvalidFunctionCode,
}

impl<'a, T: PduData> WriteSingleRegisterReponseDecoder<'a, T> {
    pub fn new(pdu: &'a T) -> Result<WriteSingleRegisterReponseDecoder<'a, T>, WriteSingleRegisterReponseDecoderError> {
        let required_length = 5;
        if pdu.pdu_data().len() < required_length {
            Err(WriteSingleRegisterReponseDecoderError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::WriteSingleRegister) => {
                    Ok(WriteSingleRegisterReponseDecoder { pdu })
                },
                _ => Err(WriteSingleRegisterReponseDecoderError::InvalidFunctionCode)
            }

        }
    }

    pub fn register_address(&self) -> u16 {
        let bytes = [
            self.pdu.pdu_data()[1],
            self.pdu.pdu_data()[2]
        ];
        u16::from_be_bytes(bytes)
    }

    pub fn register_value(&self) -> u16 {
        let bytes = [
            self.pdu.pdu_data()[3],
            self.pdu.pdu_data()[4]
        ];
        u16::from_be_bytes(bytes)
    }
}
