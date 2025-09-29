mod crc;
pub use crc::rtu_crc;

mod decoder;

mod frame;
pub use frame::ModbusRtuFrame;