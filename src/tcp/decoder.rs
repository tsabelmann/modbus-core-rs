
use super::ModbusTcpFrame;

pub enum ModbusTcpFrameDecoderError {
    InvalidProtocolIdentifier,
    InvalidLength
}

enum ModbusTcpFrameDecoderState {
    DecodeFrameHeader { index: usize },
    DecodeFramePayload {
        expected: usize,
        index: usize,
    },
    Done,
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

    pub fn push_data(&mut self, byte: u8) -> Option<ModbusTcpFrame<'_>>{
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
                        return None;
                    }

                    // read length
                    let data = [self.data[4], self.data[5]];
                    let length = u16::from_be_bytes(data) as usize;
                    
                    // check for correct length
                    if length == 0 || length > 253 + 1 {
                        self.state = ModbusTcpFrameDecoderState::DecoderError { reason: ModbusTcpFrameDecoderError::InvalidLength };
                        return None;
                    }

                    self.state = ModbusTcpFrameDecoderState::DecodeFramePayload {
                        expected: length.saturating_sub(1),
                        index: 0,
                    };
                }
                None
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

                    let result = unsafe {
                        ModbusTcpFrame::new_unchecked(&mut self.data, total_length)
                    };

                    // set new state
                    self.state = ModbusTcpFrameDecoderState::Done;

                    // return 
                    return Some(result);
                }
                None
            },
            _ => None
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
    fn test_001() {
        let mut decoder = ModbusTcpFrameDecoder::new();

        // Beispiel: eingelesene Modbus-Daten
        let received = [
            0x00, 0x01,  // Transaction ID
            0x00, 0x00,  // Protocol ID
            0x00, 0x06,  // Length
            0x11,        // Unit ID
            0x03,        // Function code
            0x00, 0x6B, 0x00, 0x03u8, // Data
        ];

        {
            // Parsing ohne Kopie, mutable Zugriff auf Daten
            for byte in received {
                if let Some(frame) = decoder.push_data(byte) {
                    println!("Transaction-Identifier={:04X}", frame.transaction_identifier());
                    println!("Protocol-Identifier={:04X}", frame.protocol_identifier());
                    println!("Length={}", frame.length());
                    println!("Unit-Identifier={}", frame.unit_identifier());
                } 
            }
        }
    }
}