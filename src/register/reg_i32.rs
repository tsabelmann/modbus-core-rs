use super::{IntoRegIter, IntoRegIterMut, GetRegisterValue, SetRegisterValue};

/// Modbus register that can be converter to and from [u8].
pub struct RegI32 {
    data: [u16; 2]
}

impl RegI32 {
    /// Creates a new [i32] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI32;
    /// 
    /// let reg = RegI32::new(42);
    /// ````
    pub const fn new(value: i32) -> RegI32 {
        RegI32 {
            data: [((value >> 16) & 0xFFFF) as u16, (value & 0xFFFF) as u16]
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI32;
    /// 
    /// let reg = RegI32::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegI32Iter<'_> {
        RegI32Iter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI32;
    /// 
    /// let mut reg = RegI32::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegI32IterMut<'_> {
        RegI32IterMut::new(self)
    }
}

impl From<i32> for RegI32 {
    fn from(value: i32) -> Self {
        RegI32::new(value)
    }
}

impl From<&i32> for RegI32 {
    fn from(value: &i32) -> Self {
        RegI32::new(*value)
    }
}

impl From<RegI32> for i32 {
    fn from(value: RegI32) -> Self {
        i32::from(&value)
    }
}

impl From<&RegI32> for i32 {
    fn from(value: &RegI32) -> Self {
        ((value.data[0] as i32) << 16) | (value.data[1] as i32)
    }
}

impl SetRegisterValue<i32> for RegI32 {
    fn set_value(&mut self, value: i32) {
        *self = RegI32::from(value);
    }
}

impl GetRegisterValue<i32> for RegI32 {
    fn get_value(&self) -> i32 {
        i32::from(self)
    }
}

/// Immutable register iterator for [RegI32].
pub struct RegI32Iter<'a> {
    value: &'a RegI32,
    index: u8
}

/// Mutable register iterator for [RegI32].
pub struct RegI32IterMut<'a> {
    value: &'a mut RegI32,
    index: u8
}

impl<'a> RegI32Iter<'a> {
    pub const fn new(value: &'a RegI32) -> RegI32Iter<'a> {
        RegI32Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegI32IterMut<'a> {
    pub const fn new(value: &'a mut RegI32) -> RegI32IterMut<'a> {
        RegI32IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegI32Iter<'a> {
    type Item = &'a u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..2 => {
                let idx = self.index as usize;
                self.index += 1;
                Some(&self.value.data[idx])
            },
            _ => None
        }
    }
}

impl<'a> Iterator for RegI32IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..2 => {
                let idx = self.index as usize;
                self.index += 1;
                let value_ptr: *mut RegI32 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[idx])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegI32 {
    type IntoIter<'a> = RegI32Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegI32Iter::new(self)
    }
} 

impl IntoRegIterMut for RegI32 {
    type IntoIterMut<'a> = RegI32IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegI32IterMut::new(self)
    }
}

#[cfg(test)]
mod reg_i32_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegI32::new(-1);
        let reg1 = RegI32::new(-2);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();

        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFE), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegI32::new(-1);
        let mut reg1 = RegI32::new(-2);
        let mut reg2 = RegI32::new(-3);
        let mut reg3 = RegI32::new(-4);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        // check values equal initialization
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFE), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFD), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFC), iter.next());
        assert_eq!(None, iter.next());

        // Change values
        let iter0: RegI32IterMut<'_> = reg0.reg_iter_mut();
        let iter1 = reg1.reg_iter_mut();
        let iter2 = reg2.reg_iter_mut();
        let iter3 = reg3.reg_iter_mut();
        
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
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
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();
        
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
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
    fn from_to_i32_001() {
        let reg = RegI32::new(-1);
        let value = i32::from(reg);
        assert_eq!(value, -1);
    }
}