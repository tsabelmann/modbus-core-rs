use crate::{ExceptionCode, FunctionKind, FunctionCode, PduDataMut};

pub(crate) trait IntoU16 {
    fn into_u16(self) -> u16;
}

impl IntoU16 for u16 {
    fn into_u16(self) -> u16 {
        self   
    }
}

impl<'a> IntoU16 for &'a u16 {
    fn into_u16(self) -> u16 {
        *self
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadHoldingRegistersReponseEncodeError {
    NotEnoughData,
    InvalidQuantityOfRegisters,
    InvalidQuantityOfProvidedRegisters
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadHoldingRegistersReponseEncodeExceptionError {
    NotEnoughData,
    InvalidExceptionCode
}

pub trait ReadHoldingRegistersReponseEncoder {
    fn encode<U: IntoU16, I: Iterator<Item=U>>(&mut self, quantity_of_registers: u16, iterator: I) -> Result<(), ReadHoldingRegistersReponseEncodeError>; 
    fn encode_exception(&mut self, code: ExceptionCode) -> Result<(), ReadHoldingRegistersReponseEncodeExceptionError>;
}

impl<T: PduDataMut> ReadHoldingRegistersReponseEncoder for T {
    fn encode<U: IntoU16, I: Iterator<Item=U>>(&mut self, quantity_of_registers: u16, iterator: I) -> Result<(), ReadHoldingRegistersReponseEncodeError> {
        // check for quantity of registers
        if quantity_of_registers > 125 || quantity_of_registers == 0 {
            return Err(ReadHoldingRegistersReponseEncodeError::InvalidQuantityOfRegisters);
        }

        // check for size  
        let required_length = 2 + (2 * quantity_of_registers) as usize;
        if self.pdu_data_mut().len() < required_length {
            return Err(ReadHoldingRegistersReponseEncodeError::NotEnoughData);   
        }

        // book keeping data for iteration
        let mut idx = 2;
        let mut reg_index = 0;
        let slice = self.pdu_data_mut();

        // iterate over registers
        for (index, reg_value) in iterator.enumerate() {
            // check quantity of registers
            if index >= quantity_of_registers as usize {
                break;
            }

            // update register index
            reg_index = index;

            // deserialize u16 register data 
            let reg_value = reg_value.into_u16();
            let data = reg_value.to_be_bytes();

            // write MSB
            let high_reff = slice.get_mut(idx);
            match high_reff {
                Some(reff) => {
                    *reff = data[0];
                },
                _ => {}
            }

            // write LSB
            let low_reff = slice.get_mut(idx+1);
            match low_reff {
                Some(reff) => {
                    *reff = data[1];
                },
                _ => {}
            };

            // increase by 2 because two bytes were written   
            idx += 2;
        }

        // check that the required amount of register were written
        if reg_index < (quantity_of_registers.saturating_sub(1)) as usize {
            return Err(ReadHoldingRegistersReponseEncodeError::InvalidQuantityOfProvidedRegisters);
        }

        // write function code
        self.set_function_code(FunctionKind::Normal(FunctionCode::ReadHoldingRegisters));

        // write byte count
        self.pdu_data_mut()[1] = (idx - 2) as u8;

        // set length (necessary for TCP/IP based modbus frames)
        let length = (1 + idx) as u16;
        self.set_length(length);

        // return Ok
        Ok(())
    }

    fn encode_exception(&mut self, code: ExceptionCode) -> Result<(), ReadHoldingRegistersReponseEncodeExceptionError> {
        // check the required data
        let required_length = 2;
        if self.pdu_data_mut().len() < required_length {
            return Err(ReadHoldingRegistersReponseEncodeExceptionError::NotEnoughData);
        }

        // set function code
        self.set_function_code(FunctionKind::new_exception(FunctionCode::ReadHoldingRegisters));

        // compute exception code
        let exception_code = match code {
            ExceptionCode::IllegalFunction => u8::from(code),
            ExceptionCode::IllegalDataAddress => u8::from(code),
            ExceptionCode::IllegalDataValue => u8::from(code),
            ExceptionCode::ServerDeviceFailure => u8::from(code),
            _ => return Err(ReadHoldingRegistersReponseEncodeExceptionError::InvalidExceptionCode)
        };

        // set exception code
        self.pdu_data_mut()[1] = exception_code;

        // set length (necessary for TCP/IP based modbus frames)
        let length = (1 + required_length)  as u16;
        self.set_length(length);

        // return Ok
        Ok(())
    }
}