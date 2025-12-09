use core::sync::atomic::{AtomicU8, Ordering};
use crate::register::{RegisterData, ReadRegisterData, WriteRegisterData, RegisterResult, RegisterError};

pub struct RegAtomicU8 {
    value: AtomicU8
}

impl<'a> RegAtomicU8 {
    pub const fn new(value: u8) -> RegAtomicU8 {
        RegAtomicU8 { value: AtomicU8::new(value) }
    }
}

pub struct RegAtomicU8Ref<'a> {
    value: &'a AtomicU8
}

impl<'a> RegAtomicU8Ref<'a> {
    pub const fn new(value: &'a AtomicU8) -> RegAtomicU8Ref<'a> {
        RegAtomicU8Ref { value }
    }
}

/* RegisterData */

impl RegisterData for RegAtomicU8 {
    const COUNT: usize = 1;
}

impl<'a> RegisterData for RegAtomicU8Ref<'a> {
    const COUNT: usize = 1;
}

/* ReadRegisterData */

impl ReadRegisterData for RegAtomicU8 {
    fn read(&self, offset: usize, buf: &mut [u16]) -> RegisterResult<usize> {
        let reff = RegAtomicU8Ref::new(&self.value);
        reff.read(offset, buf)
    }
}

impl<'a> ReadRegisterData for RegAtomicU8Ref<'a> {
    fn read(&self, offset: usize, buf: &mut [u16]) -> RegisterResult<usize> {
        if offset >= Self::COUNT {
            return Err(RegisterError::OutOfBounds { offset });
        }

        let data = self.value.load(Ordering::Acquire);
        let data = [data as u16];
        let available = Self::COUNT - offset;
        let to_read = buf.len().min(available);

        buf[..to_read].copy_from_slice(&data[offset..offset + to_read]);
        Ok(to_read)
    }
}

/* WriteRegisterData */

impl WriteRegisterData for RegAtomicU8 {
    fn write(&mut self, offset: usize, buf: &[u16]) -> RegisterResult<usize> {
        let mut reff = RegAtomicU8Ref::new(&self.value);
        reff.write(offset, buf)
    }
}

impl<'a> WriteRegisterData for RegAtomicU8Ref<'a> {
    fn write(&mut self, offset: usize, buf: &[u16]) -> RegisterResult<usize> {
        Ok(0)
    }
}
