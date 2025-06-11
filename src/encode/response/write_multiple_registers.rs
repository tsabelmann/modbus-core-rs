use crate::{ExceptionCode, FunctionCode, FunctionKind, PduDataMut};

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteMultipleRegistersReponseEncodeError {
    NotEnoughData
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteMultipleRegistersReponseEncodeExceptionError {
    NotEnoughData,
    InvalidExceptionCode
}

pub trait WriteMultipleRegistersReponseEncoder {
    fn encode(&mut self, starting_address: u16, quantity_of_registers: u16) -> Result<(), WriteMultipleRegistersReponseEncodeError>; 
    fn encode_exception(&mut self, code: ExceptionCode) -> Result<(), WriteMultipleRegistersReponseEncodeExceptionError>;
}

impl<T: PduDataMut> WriteMultipleRegistersReponseEncoder for T {
    fn encode(&mut self, starting_address: u16, quantity_of_registers: u16) -> Result<(), WriteMultipleRegistersReponseEncodeError> {
        // check for size  
        let required_length = 5;
        if self.pdu_data_mut().len() < required_length {
            return Err(WriteMultipleRegistersReponseEncodeError::NotEnoughData);   
        }

        // write function code
        self.set_function_code(FunctionKind::Normal(FunctionCode::WriteSingleRegister));

        // write starting address
        let data = starting_address.to_be_bytes();
        self.pdu_data_mut()[1] = data[0];
        self.pdu_data_mut()[2] = data[1];
        
        // write quantity of registers
        let data = quantity_of_registers.to_be_bytes();
        self.pdu_data_mut()[3] = data[0];
        self.pdu_data_mut()[4] = data[1];

        // set length (necessary for TCP/IP based modbus frames)
        let length = (1 + 5) as u16;
        self.set_length(length);

        // return Ok
        Ok(())
    }

    fn encode_exception(&mut self, code: ExceptionCode) -> Result<(), WriteMultipleRegistersReponseEncodeExceptionError> {
        // check the required data
        let required_length = 2;
        if self.pdu_data_mut().len() < required_length {
            return Err(WriteMultipleRegistersReponseEncodeExceptionError::NotEnoughData);
        }

        // set function code
        self.set_function_code(FunctionKind::new_exception(FunctionCode::ReadHoldingRegisters));

        // compute exception code
        let exception_code = match code {
            ExceptionCode::IllegalFunction => u8::from(code),
            ExceptionCode::IllegalDataAddress => u8::from(code),
            ExceptionCode::IllegalDataValue => u8::from(code),
            ExceptionCode::ServerDeviceFailure => u8::from(code),
            _ => return Err(WriteMultipleRegistersReponseEncodeExceptionError::InvalidExceptionCode)
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