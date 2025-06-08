pub struct ModbusApplicationProtocol {
    transaction_identifier: u16, 
    protocol_identifier: u16,
    length: u16,
    unit_identifier: u8
}


impl ModbusApplicationProtocol {
    pub const fn new(transaction_identifier: u16, length: u16, unit_identifier: u8) -> ModbusApplicationProtocol {
        ModbusApplicationProtocol { transaction_identifier, protocol_identifier: 0, length, unit_identifier }
    }
}

impl From<ModbusApplicationProtocol> for [u8; 7] {
    fn from(value: ModbusApplicationProtocol) -> Self {
        let mut data = [0u8; 7];
        let transaction_identifier_data = value.transaction_identifier.to_be_bytes();
        data[0] = transaction_identifier_data[0];
        data[1] = transaction_identifier_data[1];
        let protocol_identifier_data = value.protocol_identifier.to_be_bytes();
        data[2] = protocol_identifier_data[0];
        data[3] = protocol_identifier_data[1];
        let length_data = value.length.to_be_bytes();
        data[4] = length_data[0];
        data[5] = length_data[1];
        data[6] = value.unit_identifier;
        data
    }
}