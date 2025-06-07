use super::{IntoRegIter, IntoRegIterMut};

/// Modbus register that can be converter to and from [f64].
pub struct RegF64 {
    data: [u16; 4]
}

impl RegF64 {
    /// Creates a new [f64] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF64;
    /// 
    /// let reg = RegF64::new(42.0);
    /// ````
    pub const fn new(value: f64) -> RegF64 {
        let data = f64::to_be_bytes(value);
        let data = [
            ((data[0] as u16) << 8) | (data[1] as u16),
            ((data[2] as u16) << 8) | (data[3] as u16),
            ((data[4] as u16) << 8) | (data[5] as u16),
            ((data[6] as u16) << 8) | (data[7] as u16),
        ];
        
        RegF64 {
            data
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF64;
    /// 
    /// let reg = RegF64::new(42.0);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegF64Iter<'_> {
        RegF64Iter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF64;
    /// 
    /// let mut reg = RegF64::new(42.0);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegF64IterMut<'_> {
        RegF64IterMut::new(self)
    }
}

impl From<f64> for RegF64 {
    fn from(value: f64) -> Self {
        RegF64::new(value)
    }
}

impl From<&f64> for RegF64 {
    fn from(value: &f64) -> Self {
        RegF64::new(*value)
    }
}

impl From<RegF64> for f64 {
    fn from(value: RegF64) -> Self {
        f64::from(&value)
    }
}

impl From<&RegF64> for f64 {
    fn from(value: &RegF64) -> Self {
        let mut data = [0u8; 8];
        data[0] = ((value.data[0] >> 8) & 0xFF) as u8;
        data[1] = (value.data[0] & 0xFF) as u8;
        data[2] = ((value.data[1] >> 8) & 0xFF) as u8;
        data[3] = (value.data[1] & 0xFF) as u8;
        data[4] = ((value.data[2] >> 8) & 0xFF) as u8;
        data[5] = (value.data[2] & 0xFF) as u8;
        data[6] = ((value.data[3] >> 8) & 0xFF) as u8;
        data[7] = (value.data[3] & 0xFF) as u8;
        f64::from_be_bytes(data)
    }
}

/// Immutable register iterator for [RegF64].
pub struct RegF64Iter<'a> {
    value: &'a RegF64,
    index: u8
}

/// Mutable register iterator for [RegF64].
pub struct RegF64IterMut<'a> {
    value: &'a mut RegF64,
    index: u8
}

impl<'a> RegF64Iter<'a> {
    pub const fn new(value: &'a RegF64) -> RegF64Iter<'a> {
        RegF64Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegF64IterMut<'a> {
    pub const fn new(value: &'a mut RegF64) -> RegF64IterMut<'a> {
        RegF64IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegF64Iter<'a> {
    type Item = &'a u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..4 => {
                let idx = self.index as usize;
                self.index += 1;
                Some(&self.value.data[idx])
            },
            _ => None
        }
    }
}

impl<'a> Iterator for RegF64IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..4 => {
                let idx = self.index as usize;
                self.index += 1;
                let value_ptr: *mut RegF64 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[idx])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegF64 {
    type IntoIter<'a> = RegF64Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegF64Iter::new(self)
    }
} 

impl IntoRegIterMut for RegF64 {
    type IntoIterMut<'a> = RegF64IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegF64IterMut::new(self)
    }
}

#[cfg(test)]
mod reg_f64_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegF64::new(0.0);
        let mut iter = reg0.reg_iter();
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegF64::new(0.0);

        // check values equal initialization
        let mut iter = reg0.reg_iter();
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());

        let data = {
            let data = 42.0f64.to_be_bytes();
            let data = [
                ((data[0] as u16) << 8) | (data[1] as u16),
                ((data[2] as u16) << 8) | (data[3] as u16),
                ((data[4] as u16) << 8) | (data[5] as u16),
                ((data[6] as u16) << 8) | (data[7] as u16),
            ];
            data
        };

        // change values
        let mut iter = reg0.reg_iter_mut();
        if let Some(reff) = iter.next() {
            *reff = data[0];
        }

        if let Some(reff) = iter.next() {
            *reff = data[1];
        }

        if let Some(reff) = iter.next() {
            *reff = data[2];
        }

        if let Some(reff) = iter.next() {
            *reff = data[3];
        }

        // check for change
        let value = f64::from(reg0);
        assert_eq!(value, 42.0f64);
    }

    #[test]
    fn from_to_f64_001() {
        let reg = RegF64::new(42.0);
        let value = f64::from(reg);
        assert_eq!(value, 42.0);
    }
}