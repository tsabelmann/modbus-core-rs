use crate::{FunctionCode, FunctionKind, PduDataMut};

pub struct ReadHoldingRegistersResponseBuilder<'a, T, I> 
where 
    T: PduDataMut,
    I: Iterator<Item=u16>
{
    pdu: &'a mut T,
    iterator: I,
    quantity_of_registers: u16
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
