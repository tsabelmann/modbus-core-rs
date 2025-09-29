pub fn rtu_crc(data: &[u8]) -> u16 {
    let mut crc = 0xFFFF;
    for byte in data {
        crc = crc ^ (*byte as u16);
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}