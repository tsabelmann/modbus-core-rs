pub const MODBUS_FRAME_DATA_LENTGH: usize = 260;



pub struct ModbusTcpFrame<'a> {
    data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH],
    data_length: usize
}

impl<'a> ModbusTcpFrame<'a> {
    pub unsafe fn new_unchecked(data: &'a mut [u8; MODBUS_FRAME_DATA_LENTGH], data_length: usize) -> ModbusTcpFrame<'a> {
        ModbusTcpFrame { data, data_length:  data_length.min(MODBUS_FRAME_DATA_LENTGH) }
    }

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

    pub const fn is_valid(&self) -> bool {
        let length = 6 + self.length() as usize;
        length == self.data.len()
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

}