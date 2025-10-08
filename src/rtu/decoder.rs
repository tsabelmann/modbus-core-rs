use crate::{constants::MODBUS_RTU_FRAME_DATA_LENGTH, rtu::{crc::check_rtu_crc, ModbusRtuFrame}, FunctionCode, FunctionKind};

fn compute_pdu_data_length_for_request_frame(function_code: FunctionCode) -> usize {
    match function_code {
        FunctionCode::ReadHoldingRegisters => 4,
        FunctionCode::ReadCoils => 4,
        _ => 0
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ModbusRtuFrameDecoderState {
    WaitForSlaveAddress,
    WaitForFunctionCode,
    ReadByteCount {
        bytes_before_byte_count: usize
    },
    CollectPduData {
        index: usize,
        length: usize
    },
    CollectCrcData {
        index: usize
    }
}

#[derive(Debug, PartialEq)]
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
                
                // Decode based on the direction
                match self.decoder_direction {
                    ModbusRtuFrameDecoderDirection::Request => {
                        // Check function kind
                        match function_kind {
                            FunctionKind::Normal(function_code) => {
                                // Next State
                                self.state = match function_code {
                                    FunctionCode::ReadCoils => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::ReadDiscreteInputs => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::ReadHoldingRegisters => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::ReadInputRegisters => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::WriteSingleCoil => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::WriteSingleRegister => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::ReadExceptionStatus => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 1 },
                                    FunctionCode::Diagnostic => panic!("Not supported"),
                                    FunctionCode::GetComEventCounter => ModbusRtuFrameDecoderState::CollectCrcData { index: 0 },
                                    FunctionCode::GetComEventLog => ModbusRtuFrameDecoderState::CollectCrcData { index: 0 },
                                    FunctionCode::WriteMultipleCoils => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 4 },
                                    FunctionCode::WriteMultipleRegisters => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 4 },
                                    FunctionCode::ReportServerId => ModbusRtuFrameDecoderState::CollectCrcData { index: 0 },
                                    FunctionCode::ReadFileRecord => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::WriteFileRecord => panic!("Not supported"),
                                    FunctionCode::MaskWriteRegister => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 6 },
                                    FunctionCode::ReadWriteMultipleRegisters => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 8 },
                                    FunctionCode::ReadFifoQueue => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 2 },
                                    FunctionCode::Unknown(_) => {
                                        // Reset
                                        self.index = 0;
                                        ModbusRtuFrameDecoderState::WaitForSlaveAddress
                                    },
                                }
                            },
                            FunctionKind::Exception(_) => {
                                // Reset state 
                                self.reset();

                                // Return
                                return ModbusRtuFrameDecoderResult::InvalidFunctionCode;
                            },
                        }
                    },
                    ModbusRtuFrameDecoderDirection::Response => {
                        match function_kind {
                            FunctionKind::Normal(function_code) => {
                                // Next state
                                self.state = match function_code {
                                    FunctionCode::ReadCoils => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::ReadDiscreteInputs => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::ReadHoldingRegisters => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::ReadInputRegisters => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::WriteSingleCoil => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::WriteSingleRegister => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::ReadExceptionStatus => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 1 },
                                    FunctionCode::Diagnostic => panic!("Not supported!"),
                                    FunctionCode::GetComEventCounter => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::GetComEventLog => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::WriteMultipleCoils => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::WriteMultipleRegisters => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 4 },
                                    FunctionCode::ReportServerId => panic!("Not supported!"),
                                    FunctionCode::ReadFileRecord => panic!("Not supported!"),
                                    FunctionCode::WriteFileRecord => panic!("Not supported!"),
                                    FunctionCode::MaskWriteRegister => ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 6 },
                                    FunctionCode::ReadWriteMultipleRegisters => ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count: 0 },
                                    FunctionCode::ReadFifoQueue => panic!("Not supported!"),
                                    FunctionCode::Unknown(_) => {
                                        // Reset
                                        self.index = 0;
                                        ModbusRtuFrameDecoderState::WaitForSlaveAddress
                                    },
                                };
                            },
                            FunctionKind::Exception(_) => {
                                // Next State
                                self.state = ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: 1 };
                            },
                        }
                    },
                }
            },
            ModbusRtuFrameDecoderState::ReadByteCount { bytes_before_byte_count } => {
                // Push data into internal storage
                if let Some(ptr) = self.data.get_mut(self.index) {
                    *ptr = data;
                } else {
                    self.reset();
                    return ModbusRtuFrameDecoderResult::NotEnoughData;
                }

                // Increment index
                self.index += 1;

                // Check if the next byte is the byte_count
                if *bytes_before_byte_count == 0 {
                    // byte count 
                    let byte_count = data as usize;

                    // Next state
                    self.state = ModbusRtuFrameDecoderState::CollectPduData { index: 0, length: byte_count };
                } else {
                    // Decrement bytes before byte count
                    *bytes_before_byte_count -= 1;
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
    fn rtu_frame_decoder_read_coils_request_001() {
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
    fn rtu_frame_decoder_read_coils_response_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameDecoder::new(&mut buffer, ModbusRtuFrameDecoderDirection::Response);

        let _ = decoder.push_data(0x04); 
        let _ = decoder.push_data(0x01);
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0x0A);
        let _ = decoder.push_data(0x11);
        let _ = decoder.push_data(0xB3);
        match decoder.push_data(0x50) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn rtu_frame_decoder_read_discrete_inputs_request_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameDecoder::new(&mut buffer, ModbusRtuFrameDecoderDirection::Request);

        let _ = decoder.push_data(0x04); 
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x0A);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x0D);
        let _ = decoder.push_data(0x99);
        match decoder.push_data(0x98) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn rtu_frame_decoder_read_discrete_inputs_response_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameDecoder::new(&mut buffer, ModbusRtuFrameDecoderDirection::Response);

        let _ = decoder.push_data(0x04); 
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0x0A);
        let _ = decoder.push_data(0x11);
        let _ = decoder.push_data(0xB3);
        match decoder.push_data(0x14) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn rtu_frame_decoder_read_holding_registers_request_001() {
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
    }

    #[test]
    fn rtu_frame_decoder_read_holding_registers_response_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];

        let mut decoder = ModbusRtuFrameDecoder::new(&mut buffer, ModbusRtuFrameDecoderDirection::Response);
        let _ = decoder.push_data(0x01); 
        let _ = decoder.push_data(0x03);
        let _ = decoder.push_data(0x04);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x06);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x05);
        let _ = decoder.push_data(0xDA);
        match decoder.push_data(0x31) {
            ModbusRtuFrameDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

}











