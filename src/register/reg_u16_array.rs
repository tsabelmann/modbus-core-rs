use super::{IntoRegIter, IntoRegIterMut, GetRegisterValue, SetRegisterValue};
use core::{ops::{Index, IndexMut}};


/// Modbus register that can be converter to and from [u16; N].
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

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU16Array;
    /// 
    /// let reg = RegU16Array::new([42u16; 42]);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegU16ArrayIter<'_, N> {
        RegU16ArrayIter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU16Array;
    /// 
    /// let mut reg = RegU16Array::new([42u16; 42]);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegU16ArrayIterMut<'_, N> {
        RegU16ArrayIterMut::new(self)
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

impl<const N: usize> Default for RegU16Array<N> {
    fn default() -> Self {
        RegU16Array { data: [0u16; N] }
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

/// Immutable register iterator for [RegU16Array].
pub struct RegU16ArrayIter<'a, const N: usize> {
    value: &'a RegU16Array<N>,
    index: usize
}

/// Mutable register iterator for [RegF32].
pub struct RegU16ArrayIterMut<'a, const N: usize> {
    value: &'a mut RegU16Array<N>,
    index: usize
}

impl<'a, const N: usize> RegU16ArrayIter<'a, N> {
    pub const fn new(value: &'a RegU16Array<N>) -> RegU16ArrayIter<'a, N> {
        RegU16ArrayIter {
            value,
            index: 0
        }
    }
}

impl<'a, const N: usize> RegU16ArrayIterMut<'a, N> {
    pub const fn new(value: &'a mut RegU16Array<N>) -> RegU16ArrayIterMut<'a, N> {
        RegU16ArrayIterMut {
            value,
            index: 0
        }
    }
}

impl<'a, const N: usize> Iterator for RegU16ArrayIter<'a, N> {
    type Item = &'a u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < N {
            let idx = self.index;
            self.index += 1;
            Some(&self.value.data[idx])
        } else {
            None
        }
    }
}

impl<'a, const N: usize> Iterator for RegU16ArrayIterMut<'a, N> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < N {
            let idx = self.index as usize;
            self.index += 1;
            let value_ptr: *mut RegU16Array<N> = self.value;
            
            // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
            unsafe {
                Some(&mut (*value_ptr).data[idx])
            }
        } else {
            None
        }
    }
}

impl<const N: usize> IntoRegIter for RegU16Array<N> {
    type IntoIter<'a> = RegU16ArrayIter<'a, N>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegU16ArrayIter::new(self)
    }
} 

impl<const N: usize> IntoRegIterMut for RegU16Array<N> {
    type IntoIterMut<'a> = RegU16ArrayIterMut<'a, N>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegU16ArrayIterMut::new(self)
    }
}

#[cfg(test)]
mod reg_u16_array_tests {

}