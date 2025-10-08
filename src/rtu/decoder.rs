use crate::{constants::MODBUS_RTU_FRAME_DATA_LENGTH, rtu::{crc::check_rtu_crc, ModbusRtuFrame}, FunctionCode, FunctionKind};

fn compute_pdu_data_length_for_request_frame(function_code: FunctionCode) -> usize {
    match function_code {
        FunctionCode::ReadHoldingRegisters => 4,
        FunctionCode::ReadCoils => 4,
        _ => 0
    }
}

fn compute_pdu_data_length_for_response_frame(function_kind: FunctionKind) -> usize {
    match function_kind {
        FunctionKind::Normal(function_code) => match function_code {
            FunctionCode::ReadCoils => todo!(),
            FunctionCode::ReadDiscreteInputs => todo!(),
            FunctionCode::ReadHoldingRegisters => todo!(),
            FunctionCode::ReadInputRegisters => todo!(),
            FunctionCode::WriteSingleCoil => todo!(),
            FunctionCode::WriteSingleRegister => todo!(),
            FunctionCode::ReadExceptionStatus => todo!(),
            FunctionCode::Diagnostic => todo!(),
            FunctionCode::GetComEventCounter => todo!(),
            FunctionCode::GetComEventLog => todo!(),
            FunctionCode::WriteMultipleCoils => todo!(),
            FunctionCode::WriteMultipleRegisters => todo!(),
            FunctionCode::ReportServerId => todo!(),
            FunctionCode::ReadFileRecord => todo!(),
            FunctionCode::WriteFileRecord => todo!(),
            FunctionCode::MaskWriteRegister => todo!(),
            FunctionCode::ReadWriteMultipleRegisters => todo!(),
            FunctionCode::ReadFifoQueue => todo!(),
            FunctionCode::Unknown(_) => todo!(),
        },
        FunctionKind::Exception(_) => 2
    }
}


#[derive(Debug, PartialEq, Clone)]
enum ModbusRtuFrameDecoderState {
    WaitForSlaveAddress,
    WaitForFunctionCode,
    CollectPduData {
        index: usize,
        length: usize
    },
    CollectCrcData {
        index: usize
    }
}

pub enum ModbusRtuFrameDecoderResult<'a> {
    Success(ModbusRtuFrame<'a>),
    Pending,
    CrcError,
    NotSupported,
    InvalidFunctionCode,
    NotEnoughData
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum ModbusRtuFrameDecoderDirection {
    #[default]
    Request,
    Response
}

pub struct ModbusRtuFrameDecoder<'a> {
    data: &'a mut [u8; MODBUS_RTU_FRAME_DATA_LENGTH],
    state: ModbusRtuFrameDecoderState,
    index: usize,
    decoder_direction: ModbusRtuFrameDecoderDirection
}

impl<'a> ModbusRtuFrameDecoder<'a> {
    pub const fn new(data: &'a mut [u8; MODBUS_RTU_FRAME_DATA_LENGTH], decoder_direction: ModbusRtuFrameDecoderDirection) -> ModbusRtuFrameDecoder<'a> {
        let decoder = ModbusRtuFrameDecoder {
            data,
            state: ModbusRtuFrameDecoderState::WaitForSlaveAddress,
            index: 0,
            decoder_direction
        };
        decoder
    }

    pub const fn expect_request(&mut self) {
        // Reset state
        self.reset();

        // Switch to request mode
        self.decoder_direction = ModbusRtuFrameDecoderDirection::Request;
    }

    pub const fn expect_response(&mut self) {
        // Reset state
        self.reset();

        // Switch to response mode
        self.decoder_direction = ModbusRtuFrameDecoderDirection::Response;
    }

    pub fn push_data(&mut self, data: u8) -> ModbusRtuFrameDecoderResult<'_> {
        match &mut self.state {
            ModbusRtuFrameDecoderState::WaitForSlaveAddress => {
                // Push data into internal storage
                if let Some(ptr) = self.data.get_mut(self.index) {
                    *ptr = data;
                } else {
                    self.reset();
                    return ModbusRtuFrameDecoderResult::NotEnoughData;
                }

                // Increment index
                self.index += 1;

                // Next state
                self.state = ModbusRtuFrameDecoderState::WaitForFunctionCode;
            },
            ModbusRtuFrameDecoderState::WaitForFunctionCode => {
                // Push data into internal storage
                if let Some(ptr) = self.data.get_mut(self.index) {
                    *ptr = data;
                } else {
                    self.reset();
                    return ModbusRtuFrameDecoderResult::NotEnoughData;
                }

                // Increment index
                self.index += 1;

                // Function Code
                let function_kind = FunctionKind::from(data);

                // Check function kind
                match function_kind {
                    FunctionKind::Normal(function_code) => {
                        // Next State
                        self.state = ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: compute_pdu_data_length_for_request_frame(function_code) };
                    },
                    FunctionKind::Exception(_) => {
                        // Reset state 
                        self.reset();

                        // Return
                        return ModbusRtuFrameDecoderResult::InvalidFunctionCode;
                    },
                }
            },
            ModbusRtuFrameDecoderState::CollectPduData { index, length } => {
                // Push data into internal storage
                if let Some(ptr) = self.data.get_mut(self.index) {
                    *ptr = data;
                } else {
                    self.reset();
                    return ModbusRtuFrameDecoderResult::NotEnoughData;
                }

                // Increment index
                self.index += 1;

                // Increment index
                *index += 1;
                    
                if *index >= *length  {
                    self.state = ModbusRtuFrameDecoderState::CollectCrcData { index: 0 };
                }
            },
            ModbusRtuFrameDecoderState::CollectCrcData { index } => {   
                // Push data into internal storage
                if let Some(ptr) = self.data.get_mut(self.index) {
                    *ptr = data;
                } else {
                    self.reset();
                    return ModbusRtuFrameDecoderResult::NotEnoughData;
                }

                // Increment index
                self.index += 1;

                // Increment index
                *index += 1;

                if *index >= 2 {
                    // Check CRC
                    if check_rtu_crc(&self.data[..self.index-2], self.data[self.index-2], self.data[self.index-1]) {
                        let frame = unsafe {
                            ModbusRtuFrame::new_unchecked(self.data, self.index)
                        };
                        
                        // Reset state
                        self.state = ModbusRtuFrameDecoderState::WaitForSlaveAddress;
                        self.index = 0;

                        // Success
                        return ModbusRtuFrameDecoderResult::Success(frame);
                    } else {
                        // Reset state
                        self.state = ModbusRtuFrameDecoderState::WaitForSlaveAddress;
                        self.index = 0;

                        // CrcError
                        return ModbusRtuFrameDecoderResult::CrcError;
                    }
                }
            }
        }
        ModbusRtuFrameDecoderResult::Pending
    }

    pub const fn reset(&mut self) {
        self.state = ModbusRtuFrameDecoderState::WaitForSlaveAddress;
        self.index = 0;
    }


}

#[cfg(test)]
mod rtu_frames_tests {
    use super::*;

     #[test]
    fn rtu_frame_decoder_read_coils_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameDecoder::new(&mut buffer, ModbusRtuFrameDecoderDirection::Request);

        let _ = decoder.push_data(0x04); 
        let _ = decoder.push_data(0x01);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x0A);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x0D);
        let _ = decoder.push_data(0xDD);
        match decoder.push_data(0x98) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn rtu_frame_decoder_read_holding_registers_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameDecoder::new(&mut buffer, ModbusRtuFrameDecoderDirection::Request);

        let _ = decoder.push_data(0x01); 
        let _ = decoder.push_data(0x03);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0xC4);
        match decoder.push_data(0x0B) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }

        let _ = decoder.push_data(0x01); 
        let _ = decoder.push_data(0x03);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0xC4);
        match decoder.push_data(0x0B) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

}











