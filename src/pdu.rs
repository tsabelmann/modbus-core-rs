use crate::FunctionKind;

pub trait PduData {
    fn pdu_data(&self) -> & [u8];
    fn function_code(&self) -> FunctionKind;
    fn length(&self) -> Option<u16>;
}

pub trait PduDataMut : PduData {
    fn pdu_data_mut(&mut self) -> &mut [u8];
    fn set_function_code(&mut self, code: FunctionKind);
    fn set_length(&mut self, length: u16);
}