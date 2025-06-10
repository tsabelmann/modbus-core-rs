use crate::{tcp::{encoder::{EncodeError, EncodeExceptionError, ReadHoldingRegistersEnconder}, ModbusTcpFrame}, FunctionCode, FunctionKind, PduDataMut};

pub struct ReadHoldingRegistersResponseBuilder<'a, T, I> 
where 
    T: PduDataMut,
    I: Iterator<Item=u16>
{
    pdu: &'a mut T,
    iterator: I,
    quantity_of_registers: u16
}


impl<'b> ReadHoldingRegistersEnconder for ModbusTcpFrame<'b> 
{
    fn encode<'a, I>(mut self, quantity_of_registers: u16, iterator: I) -> Result<Self, EncodeError> 
            where Self: Sized, I: Iterator<Item=&'a u16> 
    {   
        // check number of registers
        if quantity_of_registers > 125 || quantity_of_registers == 0 {
            return Err(EncodeError::InvalidQuantityOfRegisters);
        }

        let mut idx = 2usize;
        let mut reg_idx = 0;
        let slice = &mut self.data[7..];

        for (reg_index, reg_value) in iterator.enumerate() {
            // check quantity of registers
            if reg_index >= quantity_of_registers as usize {
                break;
            }

            // update register index
            reg_idx = reg_index;
            
            // deserialize u16 reg data
            let data = (*reg_value).to_be_bytes();
            
            let high_reff = slice.get_mut(idx);
            match high_reff {
                Some(reff) => {
                    *reff = data[0];
                },
                _ => {}
            }

            let low_reff = slice.get_mut(idx+1);
            match low_reff {
                Some(reff) => {
                    *reff = data[1];
                },
                _ => {}
            };

            // increase by 2 because two bytes were written   
            idx += 2;
        };

        // check register index
        if reg_idx < quantity_of_registers as usize {
            return Err(EncodeError::InvalidQuantityOfProvidedRegisters);
        }

        // encode function code
        slice[0] = u8::from(FunctionKind::new_normal(FunctionCode::ReadHoldingRegisters));

        // encode byte count
        slice[1] = (idx - 2) as u8;

        // encode length
        let length = (1 + idx)  as u16;
        let data = length.to_be_bytes();
        self.data[4] = data[0];
        self.data[5] = data[1];

        // save encoded length in frame
        self.data_length = (6 + length) as usize;

        Ok(self)
    }   

    fn encode_exception(self, code: crate::ExceptionCode) -> Result<Self, EncodeExceptionError> where Self: Sized {
        Ok(self)
    }
}


impl<'a, 'b, T, I> ReadHoldingRegistersResponseBuilder<'a, T, I> 
where 
    T: PduDataMut,
    I: Iterator<Item=u16>
{
    pub fn new(pdu: &'a mut T, iterator: I, quantity_of_registers: u16) -> ReadHoldingRegistersResponseBuilder<'a, T, I>  {
        ReadHoldingRegistersResponseBuilder { pdu, iterator, quantity_of_registers }
    }

    pub fn encode(&mut self) -> Result<u16, ()> {
        let mut idx = 2usize;
        let mut byte_counter = 0u16;

        let iterator = &mut self.iterator;
        for (reg_counter, reg_value) in iterator.enumerate() {
            if reg_counter >= self.quantity_of_registers as usize {
                break;
            }
            
            let data = u16::to_be_bytes(reg_value);
            
            let high_reff = self.pdu.pdu_data_mut().get_mut(idx);
            match high_reff {
                Some(reff) => {
                    *reff = data[0];
                },
                _ => return Err(())
            }

            let low_reff = self.pdu.pdu_data_mut().get_mut(idx+1);
            match low_reff {
                Some(reff) => {
                    *reff = data[1];
                },
                _ => return Err(())
            }
            
            idx += 2;
            byte_counter += 2;
        }

        self.pdu.pdu_data_mut()[1] = byte_counter as u8;
        Ok(byte_counter + 2)
    }
}
