use crate::tcp::frame::{Frame, MbapHeader};

// ── DecodeError ──

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DecodeError {
    InvalidProtocolIdentifier,
    InvalidLength
}

// ── DecodeResult ──

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DecodeResult {
    Done,
    Pending,
    Error(DecodeError)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum DecoderState {
    DecodeFrameHeader { index: usize },
    DecodeFramePayload {
        expected: usize,
        index: usize,
    },
    DecoderError { reason: DecodeError }
}

// ── FrameDecoder ──

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Decoder {
    frame_ptr: *const Frame,
    state: DecoderState
}

impl Decoder {
    pub const fn new() -> Decoder {
        Decoder {
            frame_ptr: core::ptr::null(),
            state: DecoderState::DecodeFrameHeader { index: 0 }
        }
    }

    pub fn push_data<'a>(&mut self, frame: &'a mut Frame, byte: u8) -> DecodeResult {
        let ptr = frame as *const Frame;
        if !self.frame_ptr.is_null() && self.frame_ptr != ptr {
            self.reset();
        }
        self.frame_ptr = ptr;
        
        match &mut self.state {
            DecoderState::DecodeFrameHeader { index } => {
                // push data
                frame.raw_mut()[*index] = byte;
                
                // increment counter
                *index += 1;

                if *index == Frame::MBAP_HEADER_SIZE {
                    // read protocol_identifier
                    let protocol_identifier = frame.mbap().protocol_id();
                    
                    // check for correct protocol identifier
                    if protocol_identifier != 0 {
                        self.state = DecoderState::DecoderError { reason: DecodeError::InvalidProtocolIdentifier };
                        return DecodeResult::Error(DecodeError::InvalidProtocolIdentifier);
                    }

                    // read length
                    let length = frame.mbap().length() as usize;
                    
                    // check for correct length
                    if length == 0 || length > Frame::FRAME_SIZE - Frame::MBAP_HEADER_SIZE {
                        self.state = DecoderState::DecoderError { reason: DecodeError::InvalidLength };
                        return DecodeResult::Error(DecodeError::InvalidLength);
                    }

                    self.state = DecoderState::DecodeFramePayload {
                        expected: length - 1,
                        index: 0,
                    };
                }
                DecodeResult::Pending
            },  
            DecoderState::DecodeFramePayload { expected, index } => {
                // compute index 
                let idx = Frame::MBAP_HEADER_SIZE + *index;
                
                // push data
                frame.raw_mut()[idx] = byte;
                
                // increment counter
                *index += 1;

                if *index == *expected {
                    // compute total length
                    let total_length = Frame::MBAP_HEADER_SIZE + *expected;

                    // set new state
                    self.frame_ptr = core::ptr::null();
                    self.state = DecoderState::DecodeFrameHeader { index: 0 };

                    // set length
                    frame.set_len(total_length);

                    // return 
                    return DecodeResult::Done;
                }
                DecodeResult::Pending
            },
            DecoderState::DecoderError { reason } => {
                let reason = *reason;
                DecodeResult::Error(reason)
            }
        }
    }

    pub const fn reset(&mut self) {
        self.state = DecoderState::DecodeFrameHeader { index: 0 };
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Decoder::new()
    }
}

#[cfg(test)]
mod decode_tests {    
    use super::*;
    use crate::tcp::frame::{MbapHeader};

    // Helper to feed bytes and return final result
    fn feed_bytes(decoder: &mut Decoder, frame: &mut Frame, bytes: &[u8]) -> DecodeResult {
        let mut result = DecodeResult::Pending;
        for &byte in bytes {
            result = decoder.push_data(frame, byte);
        }
        result
    }

    #[test]
    fn decode_frame_001() {
        let mut decoder = Decoder::new();

        // example frame data
        let received = [
            0x00, 0x01,                 // Transaction ID
            0x00, 0x00,                 // Protocol ID
            0x00, 0x06,                 // Length
            0x11,                       // Unit ID
            0x03,                       // Function code
            0x012, 0x6B, 0x00, 0x03,    // Data
        ];

        // parsing without copy, mutable access to data
        let mut frame = unsafe { Frame::new_unchecked() };
        for byte in received {
            match decoder.push_data(&mut frame, byte) {
                DecodeResult::Done => {
                    assert_eq!(frame.mbap().transaction_id(), 0x0001);
                    assert_eq!(frame.mbap().protocol_id(), 0x0000);
                    assert_eq!(frame.mbap().length(), 0x0006);
                    assert_eq!(frame.mbap().unit_id(), 0x11);
                }, 
                DecodeResult::Pending => assert!(true),
                _ => assert!(false)
            } 
        }
    }

    #[test]
    fn decode_frame_002() {
        let mut decoder = Decoder::new();

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
        let mut frame = unsafe { Frame::new_unchecked() };
        for byte in received {
            match decoder.push_data(&mut frame, byte) {
                DecodeResult::Done => {
                    assert_eq!(frame.mbap().transaction_id(), 0x0001);
                    assert_eq!(frame.mbap().protocol_id(), 0x0000);
                    assert_eq!(frame.mbap().length(), 0x0003);
                    assert_eq!(frame.mbap().unit_id(), 0x11);
                }, 
                DecodeResult::Pending => assert!(true),
                _ => assert!(false)
            } 
        }
    }
  
    // ── Happy path ──

    #[test]
    fn decode_read_holding_registers_request() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01,                 // Transaction ID
            0x00, 0x00,                 // Protocol ID
            0x00, 0x06,                 // Length
            0x11,                       // Unit ID
            0x03,                       // Function code
            0x01, 0x2B, 0x00, 0x03,     // Data
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().transaction_id(), 0x0001);
        assert_eq!(frame.mbap().protocol_id(), 0x0000);
        assert_eq!(frame.mbap().length(), 0x0006);
        assert_eq!(frame.mbap().unit_id(), 0x11);
    }

    #[test]
    fn decode_exception_response() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x03,
            0x11, 0x83, 0x01,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().length(), 0x0003);
    }

    #[test]
    fn decode_minimum_length_frame() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Length = 2: unit_id + function_code, no payload
        let received = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x02,
            0x11, 0x03,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().length(), 0x0002);
    }

    #[test]
    fn decode_two_frames_sequentially() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let frame1 = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x03,
            0x11, 0x83, 0x01,
        ];
        let frame2 = [
            0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x22, 0x83, 0x02,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &frame1);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().transaction_id(), 0x0001);
        assert_eq!(frame.mbap().unit_id(), 0x11);

        let result = feed_bytes(&mut decoder, &mut frame, &frame2);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().transaction_id(), 0x0002);
        assert_eq!(frame.mbap().unit_id(), 0x22);
    }

    // ── Pending ──

    #[test]
    fn partial_header_returns_pending() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Only 4 of 7 header bytes
        let partial = [0x00, 0x01, 0x00, 0x00];
        let result = feed_bytes(&mut decoder, &mut frame, &partial);
        assert_eq!(result, DecodeResult::Pending);
    }

    #[test]
    fn partial_payload_returns_pending() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Full header + partial payload (expects 5 payload bytes, only 2 sent)
        let partial = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06,
            0x11, 0x03, 0x01,
        ];
        let result = feed_bytes(&mut decoder, &mut frame, &partial);
        assert_eq!(result, DecodeResult::Pending);
    }

    #[test]
    fn single_byte_returns_pending() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let result = decoder.push_data(&mut frame, 0x00);
        assert_eq!(result, DecodeResult::Pending);
    }

    // ── Error: invalid protocol ID ──

    #[test]
    fn invalid_protocol_id_returns_error() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01,
            0x00, 0x01, // wrong protocol ID
            0x00, 0x06,
            0x11,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Error(DecodeError::InvalidProtocolIdentifier));
    }

    #[test]
    fn invalid_protocol_id_high_byte() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01,
            0x01, 0x00, // protocol ID = 0x0100
            0x00, 0x06,
            0x11,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Error(DecodeError::InvalidProtocolIdentifier));
    }

    // ── Error: invalid length ──

    #[test]
    fn length_zero_returns_error() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, // length = 0
            0x11,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Error(DecodeError::InvalidLength));
    }

    #[test]
    fn length_too_large_returns_error() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01, 0x00, 0x00,
            0x00, 0xFF, // length = 255, exceeds max
            0x11,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &received);
        assert_eq!(result, DecodeResult::Error(DecodeError::InvalidLength));
    }

    // ── Error state behavior ──

    #[test]
    fn error_state_persists_until_reset() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Trigger error
        let bad = [0x00, 0x01, 0x00, 0x01, 0x00, 0x06, 0x11];
        feed_bytes(&mut decoder, &mut frame, &bad);

        // Subsequent bytes should still return error
        let result = decoder.push_data(&mut frame, 0x00);
        assert_eq!(result, DecodeResult::Error(DecodeError::InvalidProtocolIdentifier));

        let result = decoder.push_data(&mut frame, 0x00);
        assert_eq!(result, DecodeResult::Error(DecodeError::InvalidProtocolIdentifier));
    }

    #[test]
    fn reset_recovers_from_error() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Trigger error
        let bad = [0x00, 0x01, 0x00, 0x01, 0x00, 0x06, 0x11];
        feed_bytes(&mut decoder, &mut frame, &bad);

        // Reset and decode valid frame
        decoder.reset();
        let good = [
            0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x22, 0x83, 0x01,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &good);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().transaction_id(), 0x0002);
        assert_eq!(frame.mbap().unit_id(), 0x22);
    }

    // ── Reset ──

    #[test]
    fn reset_during_header() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Start decoding
        let partial = [0x00, 0x01, 0x00];
        feed_bytes(&mut decoder, &mut frame, &partial);

        // Reset mid-header
        decoder.reset();

        // New valid frame should decode correctly
        let good = [
            0x00, 0x05, 0x00, 0x00, 0x00, 0x03,
            0x11, 0x83, 0x01,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &good);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().transaction_id(), 0x0005);
    }

    #[test]
    fn reset_during_payload() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        // Partial valid frame (header + 1 byte payload)
        let partial = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x06,
            0x11, 0x03,
        ];
        feed_bytes(&mut decoder, &mut frame, &partial);

        // Reset mid-payload
        decoder.reset();

        // New frame
        let good = [
            0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x22, 0x83, 0x01,
        ];

        let result = feed_bytes(&mut decoder, &mut frame, &good);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame.mbap().transaction_id(), 0x0002);
    }

    // ── Frame pointer change ──

    #[test]
    fn different_frame_resets_decoder() {
        let mut decoder = Decoder::new();
        let mut frame1 = unsafe { Frame::new_unchecked() };
        let mut frame2 = unsafe { Frame::new_unchecked() };

        // Start decoding into frame1
        let partial = [0x00, 0x01, 0x00, 0x00];
        feed_bytes(&mut decoder, &mut frame1, &partial);

        // Switch to frame2 — decoder should reset
        let good = [
            0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x22, 0x83, 0x01,
        ];

        let result = feed_bytes(&mut decoder, &mut frame2, &good);
        assert_eq!(result, DecodeResult::Done);
        assert_eq!(frame2.mbap().transaction_id(), 0x0002);
    }

    // ── Byte-by-byte ──

    #[test]
    fn byte_by_byte_only_last_returns_done() {
        let mut decoder = Decoder::new();
        let mut frame = unsafe { Frame::new_unchecked() };

        let received = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x03,
            0x11, 0x83, 0x01,
        ];

        let mut done_count = 0;
        let mut pending_count = 0;

        for &byte in &received {
            match decoder.push_data(&mut frame, byte) {
                DecodeResult::Done => done_count += 1,
                DecodeResult::Pending => pending_count += 1,
                _ => panic!("unexpected error"),
            }
        }

        assert_eq!(done_count, 1);
        assert_eq!(pending_count, received.len() - 1);
    }
}