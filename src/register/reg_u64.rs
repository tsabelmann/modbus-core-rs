use super::{IntoRegIter, IntoRegIterMut};

/// Modbus register that can be converter to and from [u64].
pub struct RegU64 {
    data: [u16; 4]
}

impl RegU64 {
    /// Creates a new [u64] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU64;
    /// 
    /// let reg = RegU64::new(0x1122334455667788);
    /// ````
    pub const fn new(value: u64) -> RegU64 {
        let data = [
            ((value >> 48) & 0xFFFF) as u16,
            ((value >> 32) & 0xFFFF) as u16,
            ((value >> 16) & 0xFFFF) as u16,
            (value & 0xFFFF) as u16,
        ];

        RegU64 {
            data
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU64;
    /// 
    /// let reg = RegU64::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegU64Iter<'_> {
        RegU64Iter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU64;
    /// 
    /// let mut reg = RegU64::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegU64IterMut<'_> {
        RegU64IterMut::new(self)
    }
}

impl From<u64> for RegU64 {
    fn from(value: u64) -> Self {
        RegU64::new(value)
    }
}

impl From<&u64> for RegU64 {
    fn from(value: &u64) -> Self {
        RegU64::new(*value)
    }
}

impl From<RegU64> for u64 {
    fn from(value: RegU64) -> Self {
        u64::from(&value)
    }
}

impl From<&RegU64> for u64 {
    fn from(value: &RegU64) -> Self {
        let result = ((value.data[0] as u64) << 48) | ((value.data[1] as u64) << 32) | ((value.data[2] as u64) << 16) | (value.data[3] as u64);
        result
    }
}

/// Immutable register iterator for [RegU64].
pub struct RegU64Iter<'a> {
    value: &'a RegU64,
    index: u8
}

/// Mutable register iterator for [RegU64].
pub struct RegU64IterMut<'a> {
    value: &'a mut RegU64,
    index: u8
}

impl<'a> RegU64Iter<'a> {
    pub const fn new(value: &'a RegU64) -> RegU64Iter<'a> {
        RegU64Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegU64IterMut<'a> {
    pub const fn new(value: &'a mut RegU64) -> RegU64IterMut<'a> {
        RegU64IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegU64Iter<'a> {
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

impl<'a> Iterator for RegU64IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..4 => {
                let idx = self.index as usize;
                self.index += 1;
                let value_ptr: *mut RegU64 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[idx])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegU64 {
    type IntoIter<'a> = RegU64Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegU64Iter::new(self)
    }
} 

impl IntoRegIterMut for RegU64 {
    type IntoIterMut<'a> = RegU64IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegU64IterMut::new(self)
    }
}

#[cfg(test)]
mod reg_u64_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegU64::new(0x1122334455667788);
        let reg1 = RegU64::new(0x8877665544332211);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();

        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x5566), iter.next());
        assert_eq!(Some(&0x7788), iter.next());
        assert_eq!(Some(&0x8877), iter.next());
        assert_eq!(Some(&0x6655), iter.next());
        assert_eq!(Some(&0x4433), iter.next());
        assert_eq!(Some(&0x2211), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegU64::new(0x11223344);
        let mut reg1 = RegU64::new(0x55667788);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();

        // check values equal initialization
        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x5566), iter.next());
        assert_eq!(Some(&0x7788), iter.next());
        assert_eq!(None, iter.next());

        // Change values
        let iter0 = reg0.reg_iter_mut();
        let iter1 = reg1.reg_iter_mut();
        
        let mut iter = iter0.chain(iter1);
        if let Some(reff) = iter.next() {
            *reff = 0x00FF;
        }

        if let Some(reff) = iter.next() {
            *reff = 0xEEDD;
        }

        if let Some(reff) = iter.next() {
            *reff = 0xCCBB;
        }

        if let Some(reff) = iter.next() {
            *reff = 0xAA99;
        }

        if let Some(reff) = iter.next() {
            *reff = 0x8877;
        }

        if let Some(reff) = iter.next() {
            *reff = 0x6655;
        }

        if let Some(reff) = iter.next() {
            *reff = 0x4433;
        }

        if let Some(reff) = iter.next() {
            *reff = 0x2211;
        }

        // check for change
        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        
        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0x00FF), iter.next());
        assert_eq!(Some(&0xEEDD), iter.next());
        assert_eq!(Some(&0xCCBB), iter.next());
        assert_eq!(Some(&0xAA99), iter.next());
        assert_eq!(Some(&0x8877), iter.next());
        assert_eq!(Some(&0x6655), iter.next());
        assert_eq!(Some(&0x4433), iter.next());
        assert_eq!(Some(&0x2211), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn from_to_u32_001() {
        let reg = RegU64::new(0x1122_3344);
        let value = u64::from(reg);
        assert_eq!(value, 0x0000_0000_1122_3344);
    }
}