use core::sync::atomic::{AtomicU8, Ordering};
use crate::register::{RegisterData, ReadRegisterData, WriteRegisterData, RegisterResult, RegisterError};

pub struct AtomicRegU8 {
    value: AtomicU8
}

impl<'a> AtomicRegU8 {
    pub const fn new(value: u8) -> AtomicRegU8 {
        AtomicRegU8 { value: AtomicU8::new(value) }
    }
}

pub struct AtomicRegU8Ref<'a> {
    value: &'a AtomicU8
}

impl<'a> AtomicRegU8Ref<'a> {
    pub const fn new(value: &'a AtomicU8) -> AtomicRegU8Ref<'a> {
        AtomicRegU8Ref { value }
    }
}

/* RegisterData */

impl RegisterData for AtomicRegU8 {
    fn register_span(&self) -> usize {
        1
    }
}

impl<'a> RegisterData for AtomicRegU8Ref<'a> {
    fn register_span(&self) -> usize {
        1
    }
}

/* ReadRegisterData */

impl ReadRegisterData for AtomicRegU8 {
    fn read(&self, offset: usize, buf: &mut [u16]) -> RegisterResult<usize> {
        let reff = AtomicRegU8Ref::new(&self.value);
        reff.read(offset, buf)
    }
}

impl<'a> ReadRegisterData for AtomicRegU8Ref<'a> {
    fn read(&self, offset: usize, buf: &mut [u16]) -> RegisterResult<usize> {
        if offset >= self.register_span() {
            return Err(RegisterError::OutOfBounds { offset });
        }

        let data = self.value.load(Ordering::Acquire);
        let data = [data as u16];
        let available = self.register_span() - offset;
        let to_read = buf.len().min(available);

        buf[..to_read].copy_from_slice(&data[offset..offset + to_read]);
        Ok(to_read)
    }
}

// /* WriteRegisterData */

// impl WriteRegisterData for AtomicRegU8 {
//     fn write(&mut self, offset: usize, buf: &[u16]) -> RegisterResult<usize> {
//         let mut reff = AtomicRegU8Ref::new(&self.value);
//         reff.write(offset, buf)
//     }
// }

// impl<'a> WriteRegisterData for AtomicRegU8Ref<'a> {
//     fn write(&mut self, offset: usize, buf: &[u16]) -> RegisterResult<usize> {
//         Ok(0)
//     }
// }
