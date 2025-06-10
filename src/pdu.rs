use crate::FunctionKind;

pub trait PduData {
    fn pdu_data(&self) -> & [u8];
    fn function_code(&self) -> FunctionKind;
}

pub trait PduDataMut : PduData {
    fn pdu_data_mut(&mut self) -> &mut [u8];
    fn set_function_code(&mut self, code: FunctionKind);
}