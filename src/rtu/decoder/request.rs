use crate::{constants::MODBUS_RTU_FRAME_DATA_LENGTH, rtu::{crc::check_rtu_crc, ModbusRtuFrame}, FunctionCode, FunctionKind};

fn compute_pdu_data_length_for_request_frame(function_code: FunctionCode) -> usize {
    match function_code {
        FunctionCode::ReadHoldingRegisters => 4,
        FunctionCode::ReadCoils => 4,
        _ => 0
    }
}

enum ModbusRtuFrameRequestDecoderState {
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


pub enum ModbusRtuFrameRequestDecoderResult<'a> {
    Success(ModbusRtuFrame<'a>),
    Pending,
    CrcError,
    NotSupported,
    InvalidFunctionCode,
    NotEnoughData
}

pub struct ModbusRtuFrameRequestDecoder<'a> {
    data: &'a mut [u8; MODBUS_RTU_FRAME_DATA_LENGTH],
    state: ModbusRtuFrameRequestDecoderState,
    index: usize
}

impl<'a> ModbusRtuFrameRequestDecoder<'a> {
    pub const fn new(data: &'a mut [u8; MODBUS_RTU_FRAME_DATA_LENGTH]) -> ModbusRtuFrameRequestDecoder<'a> {
        let decoder = ModbusRtuFrameRequestDecoder {
            data,
            state: ModbusRtuFrameRequestDecoderState::WaitForSlaveAddress,
            index: 0
        };
        decoder
    }

    pub fn push_data(&mut self, data: u8) -> ModbusRtuFrameRequestDecoderResult<'_> {
        match &mut self.state {
            ModbusRtuFrameRequestDecoderState::WaitForSlaveAddress => {
                // Push data into internal storage
                self.data[self.index] = data;

                // Increment index
                self.index += 1;

                // Next state
                self.state = ModbusRtuFrameRequestDecoderState::WaitForFunctionCode;
            },
            ModbusRtuFrameRequestDecoderState::WaitForFunctionCode => {
                // Push data into internal storage
                self.data[self.index] = data;

                // Increment index
                self.index += 1;

                // Function Code
                let function_kind = FunctionKind::from(data);

                // Check function kind
                match function_kind {
                    FunctionKind::Normal(function_code) => {
                        // Next State
                        self.state = ModbusRtuFrameRequestDecoderState::CollectPduData { index: 0, length: compute_pdu_data_length_for_request_frame(function_code) };
                    },
                    FunctionKind::Exception(_) => {
                        // Reset state 
                        self.state = ModbusRtuFrameRequestDecoderState::WaitForSlaveAddress;

                        // Reset index
                        self.index = 0;

                        // Return
                        return ModbusRtuFrameRequestDecoderResult::InvalidFunctionCode;
                    },
                }
            },
            ModbusRtuFrameRequestDecoderState::CollectPduData { index, length } => {
                // Push data into internal storage
                self.data[self.index] = data;

                // Increment index
                self.index += 1;

                // Increment index
                *index += 1;
                    
                if *index >= *length  {
                    self.state = ModbusRtuFrameRequestDecoderState::CollectCrcData { index: 0 };
                }
            },
            ModbusRtuFrameRequestDecoderState::CollectCrcData { index } => {   
                // Push data into internal storage
                self.data[self.index] = data;

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
                        self.state = ModbusRtuFrameRequestDecoderState::WaitForSlaveAddress;
                        self.index = 0;

                        // Success
                        return ModbusRtuFrameRequestDecoderResult::Success(frame);
                    } else {
                        // Reset state
                        self.state = ModbusRtuFrameRequestDecoderState::WaitForSlaveAddress;
                        self.index = 0;

                        // CrcError
                        return ModbusRtuFrameRequestDecoderResult::CrcError;
                    }
                }
            }
        }
        ModbusRtuFrameRequestDecoderResult::Pending
    }

    pub const fn reset(&mut self) {
        self.state = ModbusRtuFrameRequestDecoderState::WaitForSlaveAddress;
        self.index = 0;
    }


}

#[cfg(test)]
mod rtu_frames_tests {
    use super::*;

     #[test]
    fn rtu_frame_decoder_read_coils_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameRequestDecoder::new(&mut buffer);

        let _ = decoder.push_data(0x04); 
        let _ = decoder.push_data(0x01);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x0A);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x0D);
        let _ = decoder.push_data(0xDD);
        match decoder.push_data(0x98) {
            ModbusRtuFrameRequestDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn rtu_frame_decoder_read_holding_registers_001() {
        let mut buffer = [0u8; MODBUS_RTU_FRAME_DATA_LENGTH];
        let mut decoder = ModbusRtuFrameRequestDecoder::new(&mut buffer);

        let _ = decoder.push_data(0x01); 
        let _ = decoder.push_data(0x03);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x00);
        let _ = decoder.push_data(0x02);
        let _ = decoder.push_data(0xC4);
        match decoder.push_data(0x0B) {
            ModbusRtuFrameRequestDecoderResult::Success(_frame) => {
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
            ModbusRtuFrameRequestDecoderResult::Success(_frame) => {
                assert!(true);
            },
            _ => assert!(false)
        }
    }

}











