use crate::{FunctionKind, PduData, PduDataMut};
use crate::constants::MODBUS_RTU_FRAME_DATA_LENGTH;


pub struct ModbusRtuFrame<'a> {
    data: &'a mut [u8; MODBUS_RTU_FRAME_DATA_LENGTH],
    data_length: usize
}

impl<'a> ModbusRtuFrame<'a> {
    pub fn slave_address(&self) -> u8 {
        self.data[0]
    }

    pub fn checksum(&self) -> u16 {
        let data = [
            self.data[self.data_length-2],
            self.data[self.data_length-1]
        ];
        let crc = u16::from_le_bytes(data);
        crc
    }
}

impl<'a> PduData for ModbusRtuFrame<'a> {
    fn pdu_data(&self) -> & [u8] {
        let end_index: usize = self.data_length - 2;
        &self.data[1..end_index]
    }

    fn function_code(&self) -> FunctionKind {
        FunctionKind::from(self.data[1])
    }

    fn length(&self) -> Option<u16> {
        None
    }
}

impl<'a> PduDataMut for ModbusRtuFrame<'a> {
    fn pdu_data_mut(&mut self) -> &mut [u8] {
        let end_index: usize = self.data_length - 2;
        &mut self.data[1..end_index]
    }

    fn set_function_code(&mut self, code: FunctionKind) {
        self.data[1] = u8::from(code);
    }

    fn set_length(&mut self, _length: u16) {}
}
