use crate::{FunctionCode, FunctionKind, PduData};

// ── Request ──

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RequestError {
    NotEnoughData,
    InvalidFunctionCode,
    InvalidQuantityOfRegisters,
    InvalidByteCount
}

// ── Request ──

#[derive(Clone, Copy)]
pub struct Request<'a> {
    pdu: &'a dyn PduData
}

impl<'a> Request<'a> {
    pub fn new(pdu: &'a dyn PduData) -> Result<Request<'a>, RequestError> {
        if pdu.pdu_data().len() < 6 {
            return Err(RequestError::NotEnoughData);
        }

        match pdu.function_code() {
            FunctionKind::Normal(FunctionCode::WriteMultipleRegisters) => {
                let data = [pdu.pdu_data()[1], pdu.pdu_data()[2]];
                let starting_address = u16::from_be_bytes(data);

                let data = [pdu.pdu_data()[3], pdu.pdu_data()[4]];
                let quantity_of_registers = u16::from_be_bytes(data);

                let byte_count = pdu.pdu_data()[5];

                // invalid number of registers
                if (quantity_of_registers > 123) || (quantity_of_registers == 0) {
                    return Err(RequestError::InvalidQuantityOfRegisters);
                }

                // to many registers to read
                if 0xFFFF - quantity_of_registers < starting_address {
                    return Err(RequestError::InvalidQuantityOfRegisters);
                }

                if pdu.pdu_data().len() < (6 + byte_count) as usize {
                    return Err(RequestError::NotEnoughData);
                }

                // invalid byte count
                if ((2 * quantity_of_registers) as u8) != byte_count {
                    return Err(RequestError::InvalidByteCount);
                }

                Ok(Request { pdu })
            }
            _ => Err(RequestError::InvalidFunctionCode)
        }
    }

    pub fn function_code(&self) -> FunctionKind {
        self.pdu.function_code()
    }

    pub fn starting_address(&self) -> u16 {
        let data = [self.pdu.pdu_data()[1], self.pdu.pdu_data()[2]];
        u16::from_be_bytes(data)
    }

    pub fn quantity_of_registers(&self) -> u16 {
        let data = [self.pdu.pdu_data()[3], self.pdu.pdu_data()[4]];
        u16::from_be_bytes(data)
    }

    pub fn byte_count(&self) -> u8 {
        self.pdu.pdu_data()[5]
    }

    pub fn register_values(&self) -> RegisterValueIter<'a> {
        let data = &self.pdu.pdu_data()[6..6 + self.byte_count() as usize];
        RegisterValueIter { data, index: 0 }
    }
}

impl<'a> IntoIterator for Request<'a> {
    type Item = u16;
    type IntoIter = RegisterValueIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.register_values()
    }
}

// ── RegisterValueIter ──

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterValueIter<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> Iterator for RegisterValueIter<'a> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 < self.data.len() {
            let value = u16::from_be_bytes(
                [self.data[self.index], self.data[self.index + 1]]
            );
            self.index += 2;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.data.len() - self.index) / 2;
        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for RegisterValueIter<'a> {}
