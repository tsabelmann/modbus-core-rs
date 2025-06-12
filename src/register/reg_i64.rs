use super::{IntoRegIter, IntoRegIterMut, GetRegisterValue, SetRegisterValue};

/// Modbus register that can be converter to and from [i64].
pub struct RegI64 {
    data: [u16; 4]
}

impl RegI64 {
    /// Creates a new [i64] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI64;
    /// 
    /// let reg = RegI64::new(0x1122334455667788);
    /// ````
    pub const fn new(value: i64) -> RegI64 {
        let data = [
            ((value >> 48) & 0xFFFF) as u16,
            ((value >> 32) & 0xFFFF) as u16,
            ((value >> 16) & 0xFFFF) as u16,
            (value & 0xFFFF) as u16,
        ];

        RegI64 {
            data
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI64;
    /// 
    /// let reg = RegI64::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegI64Iter<'_> {
        RegI64Iter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI64;
    /// 
    /// let mut reg = RegI64::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegI64IterMut<'_> {
        RegI64IterMut::new(self)
    }
}

impl From<i64> for RegI64 {
    fn from(value: i64) -> Self {
        RegI64::new(value)
    }
}

impl From<&i64> for RegI64 {
    fn from(value: &i64) -> Self {
        RegI64::new(*value)
    }
}

impl From<RegI64> for i64 {
    fn from(value: RegI64) -> Self {
        i64::from(&value)
    }
}

impl From<&RegI64> for i64 {
    fn from(value: &RegI64) -> Self {
        let result = ((value.data[0] as i64) << 48) | ((value.data[1] as i64) << 32) | ((value.data[2] as i64) << 16) | (value.data[3] as i64);
        result
    }
}

impl SetRegisterValue<i64> for RegI64 {
    fn set_value(&mut self, value: i64) {
        *self = RegI64::from(value);
    }
}

impl GetRegisterValue<i64> for RegI64 {
    fn get_value(&self) -> i64 {
        i64::from(self)
    }
}

/// Immutable register iterator for [RegI64].
pub struct RegI64Iter<'a> {
    value: &'a RegI64,
    index: u8
}

/// Mutable register iterator for [RegI64].
pub struct RegI64IterMut<'a> {
    value: &'a mut RegI64,
    index: u8
}

impl<'a> RegI64Iter<'a> {
    pub const fn new(value: &'a RegI64) -> RegI64Iter<'a> {
        RegI64Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegI64IterMut<'a> {
    pub const fn new(value: &'a mut RegI64) -> RegI64IterMut<'a> {
        RegI64IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegI64Iter<'a> {
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

impl<'a> Iterator for RegI64IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..4 => {
                let idx = self.index as usize;
                self.index += 1;
                let value_ptr: *mut RegI64 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[idx])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegI64 {
    type IntoIter<'a> = RegI64Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegI64Iter::new(self)
    }
} 

impl IntoRegIterMut for RegI64 {
    type IntoIterMut<'a> = RegI64IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegI64IterMut::new(self)
    }
}

#[cfg(test)]
mod reg_i64_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegI64::new(-1);
        let reg1 = RegI64::new(-2);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();

        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFE), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegI64::new(-1);
        let mut reg1 = RegI64::new(-2);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();

        // check values equal initialization
        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFE), iter.next());
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
    fn reg_iter_mut_002() {
        let mut reg0 = RegI64::new(0x1122_3344_5566_7788);

        // check values equal initialization
        let mut iter = reg0.reg_iter();
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x5566), iter.next());
        assert_eq!(Some(&0x7788), iter.next());
        assert_eq!(None, iter.next());

        // change values
        let mut iter = reg0.reg_iter_mut();
        if let Some(reff) = iter.next() {
            *reff = 0xFFFF;
        }

        if let Some(reff) = iter.next() {
            *reff = 0xFFFF;
        }

        if let Some(reff) = iter.next() {
            *reff = 0xFFFF;
        }

        if let Some(reff) = iter.next() {
            *reff = 0xFFFF;
        }

        // check values equal initialization
        let mut iter = reg0.reg_iter();
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(None, iter.next());

        // check conversion
        let value = i64::from(reg0);
        assert_eq!(value, -1);
    }

    #[test]
    fn from_to_i64_001() {
        let reg = RegI64::new(-1);
        let value = i64::from(reg);
        assert_eq!(value, -1);
    }
}