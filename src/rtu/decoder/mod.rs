use crate::{constants::MODBUS_RTU_FRAME_DATA_LENGTH, rtu::ModbusRtuFrame};

enum ModbusRtuFrameDecoderState {
    DecodeFrameHeader { index: u16 },

}

pub enum ModbusRtuFrameDecoderResult<'a> {
    Success(ModbusRtuFrame<'a>),
    Pending,
    CrcError
}

pub struct ModbusRtuFrameDecoder<'a> {
    data: &'a mut [u8],
    state: ModbusRtuFrameDecoderState
}

impl<'a> ModbusRtuFrameDecoder<'a> {
    pub const fn new(data: &'a mut [u8]) -> Option<ModbusRtuFrameDecoder<'a>> {
        if data.len() >= MODBUS_RTU_FRAME_DATA_LENGTH {
            let decoder = ModbusRtuFrameDecoder {
                data,
                state: ModbusRtuFrameDecoderState::DecodeFrameHeader { index: 0 }
            };
            Some(decoder)
        } else {
            None
        }
    }

    pub fn push_data(&mut self, byte: u8) -> ModbusRtuFrameDecoderResult<'a> {
        match &mut self.state {
            ModbusRtuFrameDecoderState::DecodeFrameHeader { index } => {
                // Push data into internal storage
                self.data[*index as usize] = byte;

                // Increment index
                *index += 1;

                if *index == 2 {
                    
                }
            },
        }


        ModbusRtuFrameDecoderResult::Pending
    }
}












