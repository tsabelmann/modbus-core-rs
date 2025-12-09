use core::sync::atomic::{AtomicU8, Ordering};
use crate::register::{RegisterData, ReadRegisterData, WriteRegisterData, RegisterResult, RegisterError};

pub struct RegAtomicU8Ref<'a> {
    value: &'a AtomicU8
}

impl<'a> RegAtomicU8Ref<'a> {
    pub const fn new(value: &'a AtomicU8) -> RegAtomicU8Ref<'a> {
        RegAtomicU8Ref { value }
    }
}

/* RegisterData */

impl<'a> RegisterData for RegAtomicU8Ref<'a> {
    const COUNT: usize = 1;
}

/* ReadRegisterData */

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

impl<'a> WriteRegisterData for RegAtomicU8Ref<'a> {
    fn write(&mut self, offset: usize, buf: &[u16]) -> RegisterResult<usize> {
        Ok(0)
    }
}