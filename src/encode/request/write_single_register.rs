use crate::{ExceptionCode, FunctionKind, FunctionCode};
use crate::rtu::{ModbusRtuFrame, rtu_crc};

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteSingleRegisterRequestEncodeError {
    Succes,
    NotEnoughData
}

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WriteSingleRegisterRequestEncodeExceptionError {
    Success,
    NotEnoughData,
    InvalidExceptionCode
}

/* MODBUS RTU */

pub trait WriteSingleRegisterRequestEncoder {
    fn encode(&mut self, register_address: u16, register_value: u16) -> WriteSingleRegisterRequestEncodeError;
    fn encode_exception(&mut self, code: ExceptionCode) -> WriteSingleRegisterRequestEncodeExceptionError;
}

impl<'a> WriteSingleRegisterRequestEncoder for ModbusRtuFrame<'a> {
    fn encode(&mut self, register_address: u16, register_value: u16) -> WriteSingleRegisterRequestEncodeError {
        if self.data.len() >= 8 {
            // Encode function code
            let function_kind = FunctionKind::Normal(FunctionCode::WriteSingleRegister);
            let function_kind = u8::from(function_kind);
            self.data[1] = function_kind;

            // Encode register address
            let bytes = register_address.to_be_bytes();
            self.data[2] = bytes[0];
            self.data[3] = bytes[1];

            // Encode register value
            let bytes = register_value.to_be_bytes();
            self.data[4] = bytes[0];
            self.data[5] = bytes[1];

            // Encode CRC
            let crc = rtu_crc(&self.data[..6]);
            let bytes = crc.to_le_bytes();
            self.data[6] = bytes[0];
            self.data[7] = bytes[1];

            // Update data length
            self.data_length = 8;
        } else {
            return WriteSingleRegisterRequestEncodeError::NotEnoughData;
        }

        WriteSingleRegisterRequestEncodeError::Succes
    }

    fn encode_exception(&mut self, code: ExceptionCode) -> WriteSingleRegisterRequestEncodeExceptionError {
        if self.data.len() >= 5 {
            // Update function code (error code)
            let function_code = FunctionKind::Exception(FunctionCode::WriteSingleRegister);
            let function_code = u8::from(function_code);
            self.data[1] = function_code;

            // Set exception code
            self.data[2] = match code {
                ExceptionCode::IllegalFunction => ExceptionCode::IllegalFunction.into(),
                ExceptionCode::IllegalDataAddress => ExceptionCode::IllegalDataAddress.into(),
                ExceptionCode::IllegalDataValue => ExceptionCode::IllegalDataValue.into(),
                ExceptionCode::ServerDeviceFailure => ExceptionCode::ServerDeviceFailure.into(),
                _ => return WriteSingleRegisterRequestEncodeExceptionError::InvalidExceptionCode
            };

            // Set data length
            self.data_length = 5;
        } else {
            return WriteSingleRegisterRequestEncodeExceptionError::NotEnoughData;
        }

        WriteSingleRegisterRequestEncodeExceptionError::Success
    }
}