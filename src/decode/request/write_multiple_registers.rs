use crate::{FunctionCode, FunctionKind, PduData};

pub struct WriteMultipleRegistersRequestDecoder<'a, T> 
where 
    T: PduData
{
    pdu: &'a T
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteMultipleRegistersRequestDecoderError {
    NotEnoughData,
    InvalidFunctionCode,
    InvalidQuanitityOfRegisters,
    InvalidByteCount
}

impl<'a, T: PduData> WriteMultipleRegistersRequestDecoder<'a, T> {
    pub fn new(pdu: &'a T) -> Result<WriteMultipleRegistersRequestDecoder<'a, T>, WriteMultipleRegistersRequestDecoderError> {
        let required_length = 6;
        if pdu.pdu_data().len() < required_length {
            Err(WriteMultipleRegistersRequestDecoderError::NotEnoughData)
        } else {
            let code = pdu.function_code();
            match code {
                FunctionKind::Normal(FunctionCode::WriteMultipleRegisters) => {
                    let data = [pdu.pdu_data()[1], pdu.pdu_data()[2]];
                    let starting_address = u16::from_be_bytes(data);

                    let data = [pdu.pdu_data()[3], pdu.pdu_data()[4]];
                    let quantity_of_registers = u16::from_be_bytes(data);

                    let byte_count = pdu.pdu_data()[5];

                    // invalid number of registers
                    if (quantity_of_registers > 123) || (quantity_of_registers == 0) {
                        return Err(WriteMultipleRegistersRequestDecoderError::InvalidQuanitityOfRegisters);
                    }

                    // to many registers to read
                    if 0xFFFF - quantity_of_registers < starting_address {
                        return Err(WriteMultipleRegistersRequestDecoderError::InvalidQuanitityOfRegisters);
                    }

                    if pdu.pdu_data().len() < (6 + byte_count) as usize {
                        return Err(WriteMultipleRegistersRequestDecoderError::NotEnoughData);
                    }

                    // invalid byte count
                    if ((2 * quantity_of_registers) as u8) != byte_count {
                        return Err(WriteMultipleRegistersRequestDecoderError::InvalidByteCount);
                    }

                    Ok(WriteMultipleRegistersRequestDecoder { pdu })
                }
                _ => Err(WriteMultipleRegistersRequestDecoderError::InvalidFunctionCode)
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

    pub fn byte_count(&self) -> u8 {
        self.pdu.pdu_data()[5]
    }
}

pub struct WriteMultipleRegistersRequestDecoderIter<'a, T: PduData> {
    pdu: WriteMultipleRegistersRequestDecoder<'a, T>,
    index: u8
}

impl<'a, T: PduData> Iterator for WriteMultipleRegistersRequestDecoderIter<'a, T> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = 6 + (2 * self.index) as usize;
        let quantity_of_registers = self.pdu.quantity_of_registers() as u8;
        if self.index < quantity_of_registers {
            let mut data = [0u8; 2];
            let slice = &self.pdu.pdu.pdu_data();
            
            let high_reff = slice.get(idx);
            match high_reff {
                Some(reff) => {
                    data[0] = *reff;
                },
                _ => return None
            };

            let low_reff = slice.get(idx+1);
            match low_reff {
                Some(reff) => {
                    data[1] = *reff;
                },
                _ => return None
            };

            // construct value
            let value = u16::from_be_bytes(data);

            // increase index
            self.index += 1;

            // return value
            Some(value)
        } else {
            None
        }
    }
}

impl<'a, T: PduData> IntoIterator for WriteMultipleRegistersRequestDecoder<'a, T> {
    type Item = u16;
    type IntoIter = WriteMultipleRegistersRequestDecoderIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        WriteMultipleRegistersRequestDecoderIter {
            pdu: self, 
            index: 0
        }
    }
}
