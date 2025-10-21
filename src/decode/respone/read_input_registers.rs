use crate::{FunctionCode, FunctionKind, PduData};

pub struct ReadInputRegistersResponseDecoder<'a, T> 
where 
    T: PduData
{
    pdu: &'a T 
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadInputRegistersResponseDecoderError {
    NotEnoughData,
    InvalidQuanitityOfRegisters,
    InvalidFunctionCode,
    CrcError
}

impl<'a, T: PduData> ReadInputRegistersResponseDecoder<'a, T> {
    pub fn new(pdu: &'a T) -> Result<ReadInputRegistersResponseDecoder<'a, T>, ReadInputRegistersResponseDecoderError> {
        let required_length = 4;
        if pdu.pdu_data().len() < required_length {
            Err(ReadInputRegistersResponseDecoderError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::ReadInputRegisters) => {
                    let byte_count = pdu.pdu_data()[1] as usize;
                    let quantity_of_registers = byte_count / 2;
                    let required_length = 2 + byte_count;
                    
                    // Not enough data to provide
                    if pdu.pdu_data().len() < required_length {
                        return Err(ReadInputRegistersResponseDecoderError::NotEnoughData);
                    }
                    
                    // invalid number of registers
                    if (quantity_of_registers > 125) || (quantity_of_registers == 0) {
                        return Err(ReadInputRegistersResponseDecoderError::InvalidQuanitityOfRegisters);
                    }

                    Ok(ReadInputRegistersResponseDecoder { pdu })
                }
                _ => Err(ReadInputRegistersResponseDecoderError::InvalidFunctionCode)
            }
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        self.pdu.function_code()
    }

    pub fn byte_count(&self) -> u8 {
        self.pdu.pdu_data()[1]
    }

    pub fn quantity_of_registers(&self) -> u8 {
        self.byte_count() / 2
    }

}

/* INTO ITERATOR */

impl<'a, T: PduData> IntoIterator for ReadInputRegistersResponseDecoder<'a, T> {
    type Item = u16;
    type IntoIter = ReadInputRegistersResponseRegisterIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ReadInputRegistersResponseRegisterIter {
            data: self,
            register_index: 0
        }
    }
}

/* ITERATOR */

pub struct ReadInputRegistersResponseRegisterIter<'a, T>
where 
    T: PduData
{
    data: ReadInputRegistersResponseDecoder<'a, T>,
    register_index: u8
}

impl<'a, T: PduData> Iterator for ReadInputRegistersResponseRegisterIter<'a, T> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.register_index < self.data.quantity_of_registers() {
            let base_index = 2 + 2 * (self.register_index as usize);
            let range = base_index..(base_index+2);

            match self.data.pdu.pdu_data().get(range) {
                Some(slice) => {
                    // Compute iterator return value
                    let mut value = (slice[0] as u16) << 8;
                    value += slice[1] as u16;

                    // Increment register index
                    self.register_index += 1;

                    // Return iterator value
                    Some(value)
                },
                None => None,
            }
        } else {
            None
        }
    }
}
