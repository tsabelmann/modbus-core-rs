pub mod read_holding_registers;
pub use read_holding_registers::{RequestDecoder, DecoderError};

mod write_single_register;
pub use write_single_register::{WriteSingleRegisterRequestDecoder, WriteSingleRegisterRequestDecoderError};

mod write_multiple_registers;
pub use write_multiple_registers::{WriteMultipleRegistersRequestDecoder, WriteMultipleRegistersRequestDecoderError, WriteMultipleRegistersRequestDecoderIter};