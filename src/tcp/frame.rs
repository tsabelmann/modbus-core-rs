use crate::{FunctionKind, PduData, PduDataMut};

pub const MODBUS_FRAME_DATA_LENTGH: usize = 260;

pub(crate) struct ValidModbusTcpFrame<'a> {
    pub data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH],
    pub data_length: usize
}

pub struct ModbusTcpFrame<'a> {
    data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH],
    data_length: usize
}

impl<'a> ModbusTcpFrame<'a> {
    pub(crate) const fn new(frame: ValidModbusTcpFrame<'a>) -> ModbusTcpFrame<'a> {
        ModbusTcpFrame { data: frame.data, data_length: frame.data_length }
    }

    // pub const fn new_unchecked()

    pub const fn transaction_identifier(&self) -> u16 {
        let data = [self.data[0], self.data[1]];
        u16::from_be_bytes(data)
    }

    pub const fn protocol_identifier(&self) -> u16 {
        let data = [self.data[2], self.data[3]];
        u16::from_be_bytes(data)
    }

    pub const fn length(&self) -> u16 {
        let data = [self.data[4], self.data[5]];
        u16::from_be_bytes(data)
    }

    pub const fn unit_identifier(&self) -> u8 {
        self.data[6]
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