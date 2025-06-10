use crate::{FunctionKind, PduData, PduDataMut};

pub const MODBUS_FRAME_DATA_LENTGH: usize = 260;

pub(crate) struct ValidModbusTcpFrame<'a> {
    pub data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH],
    pub data_length: usize
}

pub struct ModbusTcpFrame<'a> {
    pub(crate) data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH],
    pub(crate) data_length: usize
}

impl<'a> ModbusTcpFrame<'a> {
    pub(crate) const fn new(frame: ValidModbusTcpFrame<'a>) -> ModbusTcpFrame<'a> {
        ModbusTcpFrame { data: frame.data, data_length: frame.data_length }
    }

    pub const unsafe fn new_unchecked(data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH]) -> ModbusTcpFrame<'a> {
        ModbusTcpFrame { data, data_length: MODBUS_FRAME_DATA_LENTGH  }
    }

    pub const fn transaction_identifier(&self) -> u16 {
        let data = [self.data[0], self.data[1]];
        u16::from_be_bytes(data)
    }

    pub const fn set_transaction_identifier(&mut self, transaction_id: u16) {
        let data = transaction_id.to_be_bytes();
        self.data[0] = data[0];
        self.data[1] = data[1]
    }

    pub const fn protocol_identifier(&self) -> u16 {
        let data = [self.data[2], self.data[3]];
        u16::from_be_bytes(data)
    }

    pub const fn length(&self) -> u16 {
        let data = [self.data[4], self.data[5]];
        u16::from_be_bytes(data)
    }

    pub const fn set_length(&mut self, length: u16) {
        let data = length.to_be_bytes();
        self.data[4] = data[0];
        self.data[5] = data[1];
    }

    pub const fn unit_identifier(&self) -> u8 {
        self.data[6]
    }

    pub fn copy_to<'b>(&self, frame: &mut ModbusTcpFrame<'b>) {
        let slice = &self.data[..self.data_length];
        let slice_iter = slice.iter();
        let frame_slice = &mut frame.data[..self.data_length];
        let frame_slice_iter = frame_slice.iter_mut();

        for (dest, src) in frame_slice_iter.zip(slice_iter) {
            *dest = *src;
        }
        frame.data_length = self.data_length;
    }
}

impl<'a> PduData for ModbusTcpFrame<'a> {
    fn pdu_data(&self) -> & [u8] {
        &self.data[7..self.data_length]
    }

    fn function_code(&self) -> FunctionKind {
        FunctionKind::from(self.data[7])
    }
}

impl<'a> PduDataMut for ModbusTcpFrame<'a> {
    fn pdu_data_mut(&mut self) -> &mut [u8] {
        &mut self.data[7..]
    }

    fn set_function_code(&mut self, code: FunctionKind) {
        self.data[7] = u8::from(code);
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

}