///
pub const MODBUS_TCP_FRAME_DATA_SIZE: usize = 253;

pub struct ModbusTcpFrame {
    transaction_identifier: u16,
    protocol_identifier: u16,
    unit_identifier: u8,
    function_code: u8,
    data: [u8; MODBUS_TCP_FRAME_DATA_SIZE],
    data_length: usize,
}

impl ModbusTcpFrame {
    pub const fn new(transaction_identifier: u16, unit_identifier: u8, function_code: u8) -> ModbusTcpFrame {
        ModbusTcpFrame {
            transaction_identifier,
            protocol_identifier: 0,
            unit_identifier,
            function_code,
            data: [0u8; MODBUS_TCP_FRAME_DATA_SIZE],
            data_length: 0
        }
    }
}

#[cfg(test)]
mod frame_tests {
    use super::*;

}