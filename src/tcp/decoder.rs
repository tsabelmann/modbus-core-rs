
use crate::tcp::frame::ValidModbusTcpFrame;

use super::ModbusTcpFrame;

pub enum ModbusTcpFrameDecoderError {
    InvalidProtocolIdentifier,
    InvalidLength,
    Pending
}

enum ModbusTcpFrameDecoderState {
    DecodeFrameHeader { index: usize },
    DecodeFramePayload {
        expected: usize,
        index: usize,
    },
    DecoderError { reason: ModbusTcpFrameDecoderError }
}


pub struct ModbusTcpFrameDecoder {
    data: [u8; 260],
    state: ModbusTcpFrameDecoderState
}

impl ModbusTcpFrameDecoder {
    pub const fn new() -> ModbusTcpFrameDecoder {
        ModbusTcpFrameDecoder {
            data: [0u8; 260],
            state: ModbusTcpFrameDecoderState::DecodeFrameHeader { index: 0 }
        }
    }

    pub fn push_data(&mut self, byte: u8) -> Result<ModbusTcpFrame<'_>, ModbusTcpFrameDecoderError> {
        match &mut self.state {
            ModbusTcpFrameDecoderState::DecodeFrameHeader { index } => {
                // push data
                self.data[*index] = byte;
                
                // increment counter
                *index += 1;

                if *index == 7 {
                    // read protocol_identifier
                    let data: [u8; 2] = [self.data[2], self.data[3]];
                    let protocol_identifier = u16::from_be_bytes(data);
                    
                    // check for correct protocol identifier
                    if protocol_identifier != 0 {
                        self.state = ModbusTcpFrameDecoderState::DecoderError { reason: ModbusTcpFrameDecoderError::InvalidProtocolIdentifier };
                        return Err(ModbusTcpFrameDecoderError::InvalidProtocolIdentifier);
                    }

                    // read length
                    let data = [self.data[4], self.data[5]];
                    let length = u16::from_be_bytes(data) as usize;
                    
                    // check for correct length
                    if length == 0 || length > 253 + 1 {
                        self.state = ModbusTcpFrameDecoderState::DecoderError { reason: ModbusTcpFrameDecoderError::InvalidLength };
                        return Err(ModbusTcpFrameDecoderError::InvalidLength);
                    }

                    self.state = ModbusTcpFrameDecoderState::DecodeFramePayload {
                        expected: length.saturating_sub(1),
                        index: 0,
                    };
                }
                Err(ModbusTcpFrameDecoderError::Pending)
            },  
            ModbusTcpFrameDecoderState::DecodeFramePayload { expected, index } => {
                // compute index 
                let idx = 7 + *index;
                
                // push data
                self.data[idx] = byte;
                
                // increment counter
                *index += 1;

                if *index == *expected {
                    // compute total length
                    let total_length = 7 + *expected;

                    let result = {
                        let valid_frame = ValidModbusTcpFrame { data: &mut self.data, data_length: total_length};
                        ModbusTcpFrame::new(valid_frame)
                    };

                    // set new state
                    self.state = ModbusTcpFrameDecoderState::DecodeFrameHeader { index: 0 };

                    // return 
                    return Ok(result);
                }
                Err(ModbusTcpFrameDecoderError::Pending)
            },
            _ => Err(ModbusTcpFrameDecoderError::Pending)
        }
    }

    pub const fn reset(&mut self) {
        self.state = ModbusTcpFrameDecoderState::DecodeFrameHeader { index: 0 };
    }
}



#[cfg(test)]
mod decode_tests {    
    use super::*;

    #[test]
    fn decode_frame_001() {
        let mut decoder = ModbusTcpFrameDecoder::new();

        // example frame data
        let received = [
            0x00, 0x01,             // Transaction ID
            0x00, 0x00,             // Protocol ID
            0x00, 0x06,             // Length
            0x11,                   // Unit ID
            0x03,                   // Function code
            0x012, 0x6B, 0x00, 0x03, // Data
        ];

        // parsing without copy, mutable access to data
        for byte in received {
            match decoder.push_data(byte) {
                Ok(frame) => {
                    assert_eq!(frame.transaction_identifier(), 0x0001);
                    assert_eq!(frame.protocol_identifier(), 0x0000);
                    assert_eq!(frame.length(), 0x0006);
                    assert_eq!(frame.unit_identifier(), 0x11);
                }, 
                Err(ModbusTcpFrameDecoderError::Pending) => assert!(true),
                _ => assert!(false)
            } 
        }
    }

    #[test]
    fn decode_frame_002() {
        let mut decoder = ModbusTcpFrameDecoder::new();

        // example frame data
        let received = [
            0x00, 0x01, // Transaction ID
            0x00, 0x00, // Protocol ID
            0x00, 0x03, // Length
            0x11,       // Unit ID
            0x83,       // Function code (exception)
            0x01,       // Exception code
        ];

        // parsing without copy, mutable access to data
        for byte in received {
            match decoder.push_data(byte) {
                Ok(frame) => {
                    assert_eq!(frame.transaction_identifier(), 0x0001);
                    assert_eq!(frame.protocol_identifier(), 0x0000);
                    assert_eq!(frame.length(), 0x0003);
                    assert_eq!(frame.unit_identifier(), 0x11);
                }, 
                Err(ModbusTcpFrameDecoderError::Pending) => assert!(true),
                _ => assert!(false)
            } 
        }
    }
}