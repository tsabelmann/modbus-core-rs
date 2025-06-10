use crate::ExceptionCode;

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EncodeError {
    InvalidQuantityOfRegisters,
    InvalidQuantityOfProvidedRegisters
}

pub enum EncodeExceptionError {
    WrongExceptionCode
}


pub trait ReadHoldingRegistersEnconder {
    fn encode<'a, I>(self, quantity_of_registers: u16, iterator: I) -> Result<Self, EncodeError> 
        where Self: Sized, I: Iterator<Item=&'a u16>;
    fn encode_exception(self, code: ExceptionCode) -> Result<Self, EncodeExceptionError> where Self: Sized;
}