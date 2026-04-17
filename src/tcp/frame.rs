use crate::{FunctionKind, AduData, PduData, PduDataMut};

// ── Traits ──

pub trait MbapHeader {
    fn transaction_id(&self) -> u16;
    fn protocol_id(&self) -> u16;
    fn length(&self) -> u16;
    fn unit_id(&self) -> u8;
}

pub trait MbapHeaderMut {
    fn set_transaction_id(&mut self, id: u16);
    fn set_unit_id(&mut self, id: u8);
}

// ── Mbap ──

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mbap<'a> {
    data: &'a [u8]
}

impl<'a> MbapHeader for Mbap<'a> {
    fn transaction_id(&self) -> u16 {
        let data = [self.data[0], self.data[1]];
        u16::from_be_bytes(data)
    }

    fn protocol_id(&self) -> u16 {
        let data = [self.data[2], self.data[3]];
        u16::from_be_bytes(data)
    }

    fn length(&self) -> u16 {
        let data = [self.data[4], self.data[5]];
        u16::from_be_bytes(data)
    }

    fn unit_id(&self) -> u8 {
        self.data[6]
    }
}

// ── MbapMut ──

pub struct MbapMut<'a> {
    data: &'a mut [u8]
}

impl<'a> MbapHeader for MbapMut<'a> {
    fn transaction_id(&self) -> u16 {
        Mbap { data: self.data }.transaction_id()
    }

    fn protocol_id(&self) -> u16 {
        Mbap { data: self.data }.protocol_id()
    }

    fn length(&self) -> u16 {
        Mbap { data: self.data }.length()
    }

    fn unit_id(&self) -> u8 {
        Mbap { data: self.data }.unit_id()
    }
}

impl<'a> MbapHeaderMut for MbapMut<'a> {
    fn set_transaction_id(&mut self, id: u16) {
        let bytes = id.to_be_bytes();
        self.data[0] = bytes[0];
        self.data[1] = bytes[1];
    }

    fn set_unit_id(&mut self, id: u8) {
        self.data[6] = id;
    }
}

// ── Frame ──

#[derive(Debug, PartialEq, Clone)]
pub struct Frame {
    data: [u8; Frame::FRAME_SIZE],
    data_length: usize
}

impl Frame {
    pub const FRAME_SIZE: usize = 260;
    pub const MBAP_HEADER_SIZE: usize = 7;

    pub const unsafe fn new_unchecked() -> Frame {
        let mut data = [0u8; Frame::FRAME_SIZE];
        let bytes = 254u16.to_be_bytes();
        data[4] = bytes[0];
        data[5] = bytes[1]; 
        Frame { data, data_length: Frame::FRAME_SIZE }
    }

    pub fn mbap(&self) -> Mbap<'_> {
        Mbap { data: &self.data[..Frame::MBAP_HEADER_SIZE] }
    }

    pub fn mbap_mut(&mut self) -> MbapMut<'_> {
        MbapMut { data: &mut self.data[..Frame::MBAP_HEADER_SIZE] }
    }

    pub(crate) fn raw_mut(&mut self) -> &mut [u8; Frame::FRAME_SIZE] {
        &mut self.data
    }

    pub(crate) fn set_len(&mut self, len: usize) {
        debug_assert!(len <= Frame::FRAME_SIZE);
        self.data_length = len.min(Frame::FRAME_SIZE);
    }
}

impl AduData for Frame {
    fn adu_data(&self) -> & [u8] {
        &self.data[..self.data_length]
    }

    fn adu_length(&self) -> usize {
        self.data_length
    }
}

impl PduData for Frame {
    fn pdu_data(&self) -> & [u8] {
        &self.data[Frame::MBAP_HEADER_SIZE..self.data_length]
    }

    fn function_code(&self) -> FunctionKind {
        FunctionKind::from(self.data[7])
    }

    fn length(&self) -> Option<u16> {
        Some(self.mbap().length())
    }
}

// /* PDU DATA MUT */

// impl<'a> PduDataMut for Frame<'a> {
//     fn pdu_data_mut(&mut self) -> &mut [u8] {
//         &mut self.data[7..]
//     }

//     fn set_function_code(&mut self, code: FunctionKind) {
//         self.data[7] = u8::from(code);
//     }

//     fn set_length(&mut self, length: u16) {
//         if length <= 254 {
//             // write length field
//             let data = length.to_be_bytes();
//             self.data[4] = data[0];
//             self.data[5] = data[1];
            
//             // set internal array length
//             self.data_length = (6 + length) as usize;
//         }
//     }
// }

#[cfg(test)]
mod frame_tests {}