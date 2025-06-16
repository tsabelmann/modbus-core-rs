use crate::{FunctionCode, FunctionKind, PduData};

pub struct WriteSingleRegisterRequestDecoder<'a, T> 
where 
    T: PduData
{
    pdu: &'a T
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteSingleRegisterRequestDecoderError {
    NotEnoughData,
    InvalidFunctionCode
}

impl<'a, T: PduData> WriteSingleRegisterRequestDecoder<'a, T> {
    pub fn new(pdu: &'a T) -> Result<WriteSingleRegisterRequestDecoder<'a, T>, WriteSingleRegisterRequestDecoderError> {
        let reqired_length = 5;
        if pdu.pdu_data().len() < reqired_length {
            Err(WriteSingleRegisterRequestDecoderError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::WriteSingleRegister) => {
                    Ok(WriteSingleRegisterRequestDecoder { pdu })
                }
                _ => Err(WriteSingleRegisterRequestDecoderError::InvalidFunctionCode)
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

pub struct WriteSingleRegisterRequestDecoderIter<'a, T: PduData> {
    pdu: WriteSingleRegisterRequestDecoder<'a, T>,
    index: u8
}

impl<'a, T: PduData> Iterator for WriteSingleRegisterRequestDecoderIter<'a, T> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => Some(self.pdu.register_value()),
            _ => None
        }
    }
}

impl<'a, T: PduData> IntoIterator for WriteSingleRegisterRequestDecoder<'a, T> {
    type Item = u16;
    type IntoIter = WriteSingleRegisterRequestDecoderIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        WriteSingleRegisterRequestDecoderIter {
            pdu: self, 
            index: 0
        }
    }
}

