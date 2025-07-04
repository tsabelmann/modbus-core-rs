use super::{GetRegisterValue, SetRegisterValue};
use core::{ops::{Index, IndexMut}};

/// Modbus register that can be converter to and from [u16; N].
#[derive(Debug, PartialEq, Clone)]
pub struct RegU16Array<const N: usize> {
    data: [u16; N]
}

impl<const N: usize> RegU16Array<N> {
    /// Creates a new [[u16; N]] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF32;
    /// 
    /// let reg = RegF32::new(42.0);
    /// ````
    pub const fn new(value: [u16; N]) -> RegU16Array<N> {
        RegU16Array { data: value }
    }

    pub const fn from_str(string: &str) -> RegU16Array<N> {
        let mut data = [0u16; N];
        let slice = string.as_bytes();
        let slice_len = slice.len();

        let mut idx = 0;
        let mut slice_idx = 0;

        while idx < N {
            let hbyte = if slice_idx < slice_len {
                // save current slice index
                let i = slice_idx;
                
                // increment slice index
                slice_idx += 1;

                // slice data
                slice[i]
            } else {
                // padded data
                b'\0'
            };

            let lbyte = if slice_idx < slice_len {
                // save current slice index
                let i = slice_idx;
                
                // increment slice index
                slice_idx += 1;

                // slice data
                slice[i]
            } else {
                // padded data
                b'\0'
            };
            // write data
            data[idx] = u16::from_be_bytes([hbyte, lbyte]);

            // increment idx
            idx += 1;
        }

        RegU16Array { data }
    }

}

impl<const N: usize> Default for RegU16Array<N> {
    fn default() -> Self {
        RegU16Array { data: [0u16; N] }
    }
}

impl<const N: usize> From<&str> for RegU16Array<N> {
    fn from(value: &str) -> Self {
        RegU16Array::from_str(value)
    }
}

impl<const N: usize> From<[u16; N]> for RegU16Array<N> {
    fn from(value: [u16; N]) -> Self {
        RegU16Array::new(value)
    }
}

impl<const N: usize> From<RegU16Array<N>> for [u16; N] {
    fn from(value: RegU16Array<N>) -> Self {
        value.data
    }
}

impl<const N: usize> From<&RegU16Array<N>> for [u16; N] {
    fn from(value: &RegU16Array<N>) -> Self {
        value.data
    }
}

impl<const N: usize> Index<usize> for RegU16Array<N> {
    type Output = u16;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<usize> for RegU16Array<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> IntoIterator for RegU16Array<N> {
    type Item = u16;
    type IntoIter = core::array::IntoIter<u16, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a RegU16Array<N> {
    type Item = &'a u16;
    type IntoIter = core::slice::Iter<'a, u16>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.data).into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut RegU16Array<N> {
    type Item = &'a mut u16;
    type IntoIter = core::slice::IterMut<'a, u16>;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.data).into_iter()
    }
}

impl<const N: usize> SetRegisterValue<[u16; N]> for RegU16Array<N> {
    fn set_value(&mut self, value: [u16; N]) {
        *self = RegU16Array::from(value);
    }
}

impl<const N: usize> GetRegisterValue<[u16; N]> for RegU16Array<N> {
    fn get_value(&self) -> [u16; N] {
        self.into()
    }
}

impl<'a, const N: usize> GetRegisterValue<&'a [u16]> for &'a RegU16Array<N> {
    fn get_value(&self) -> &'a [u16] {
        &self.data
    }
}

#[cfg(test)]
mod reg_u16_array_tests {

}