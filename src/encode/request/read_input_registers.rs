use crate::{ExceptionCode, FunctionKind, FunctionCode};
use crate::rtu::{ModbusRtuFrame, rtu_crc};

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadInputRegistersRequestEncodeError {
    Succes,
    NotEnoughData,
    InvalidQuantityOfRegisters,
    InvalidQuantityOfProvidedRegisters
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReadInputRegistersRequestEncodeExceptionError {
    Success,
    NotEnoughData,
    InvalidExceptionCode
}

pub trait ReadInputRegistersRequestEncoder 
{
    fn encode(&mut self, start_address: u16, quantity_of_registers: u16) -> ReadInputRegistersRequestEncodeError; 
    fn encode_exception(&mut self, code: ExceptionCode) -> ReadInputRegistersRequestEncodeExceptionError;
}

impl<'a> ReadInputRegistersRequestEncoder for ModbusRtuFrame<'a> {
    fn encode(&mut self, start_address: u16, quantity_of_registers: u16) -> ReadInputRegistersRequestEncodeError {
        if self.data.len() > 8 {
            if quantity_of_registers == 0 || quantity_of_registers > 0x007D {
                return ReadInputRegistersRequestEncodeError::InvalidQuantityOfRegisters;
            }

            // Encode Starting Address
            let bytes = start_address.to_be_bytes();
            self.data[2] = bytes[0];
            self.data[3] = bytes[1];

            // Encode Quantity of Registers
            let bytes = quantity_of_registers.to_be_bytes();
            self.data[4] = bytes[0];
            self.data[5] = bytes[1];

            // Encode CRC
            let crc = rtu_crc(&self.data[..6]);
            let bytes = crc.to_le_bytes();        
            self.data[6] = bytes[0];
            self.data[7] = bytes[1];

            // Update Data Length
            self.data_length = 8;
        } else {
            return ReadInputRegistersRequestEncodeError::NotEnoughData;
        }

        ReadInputRegistersRequestEncodeError::Succes
    }

    fn encode_exception(&mut self, code: ExceptionCode) -> ReadInputRegistersRequestEncodeExceptionError {    
        if self.data.len() > 5 {
            // Update function code (error code)
            let function_code = FunctionKind::Exception(FunctionCode::ReadInputRegisters);
            let function_code = u8::from(function_code);
            self.data[1] = function_code;

            // Set exeception code
            self.data[2] = match code {
                ExceptionCode::IllegalFunction => 0x01,
                ExceptionCode::IllegalDataAddress => 0x02,
                ExceptionCode::IllegalDataValue => 0x03,
                ExceptionCode::ServerDeviceFailure => 0x04,
                _ => return ReadInputRegistersRequestEncodeExceptionError::InvalidExceptionCode
            };
        } else {
            return ReadInputRegistersRequestEncodeExceptionError::NotEnoughData;
        }

        ReadInputRegistersRequestEncodeExceptionError::Success
    }
}
