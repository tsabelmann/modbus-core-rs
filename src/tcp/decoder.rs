


enum ModbusTcpFrameDecoderState {
    DecodeFrameHeader { index: usize },
    DecodeFramePayload,
}


pub struct ModbusTcpFrameDecoder {
    data: [u8; 7],
    state: ModbusTcpFrameDecoderState
}

impl ModbusTcpFrameDecoder {
    pub const fn new() -> ModbusTcpFrameDecoder {
        ModbusTcpFrameDecoder {
            data: [0u8; 7],
            state: ModbusTcpFrameDecoderState::DecodeFrameHeader { index: 0 }
        }
    }
}



#[cfg(test)]
mod decode_tests {
    use super::*;
}