use super::{IntoRegIter, IntoRegIterMut};

/// Modbus register that can be converter to and from [u32].
pub struct RegU32 {
    data: [u16; 2]
}

impl RegU32 {
    /// Creates a new [u32] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU32;
    /// 
    /// let reg = RegU32::new(42);
    /// ````
    pub const fn new(value: u32) -> RegU32 {
        RegU32 {
            data: [((value >> 16) & 0xFFFF) as u16, (value & 0xFFFF) as u16]
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU32;
    /// 
    /// let reg = RegU32::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegU32Iter<'_> {
        RegU32Iter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU32;
    /// 
    /// let mut reg = RegU32::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegU32IterMut<'_> {
        RegU32IterMut::new(self)
    }
}

impl From<u32> for RegU32 {
    fn from(value: u32) -> Self {
        RegU32::new(value)
    }
}

impl From<&u32> for RegU32 {
    fn from(value: &u32) -> Self {
        RegU32::new(*value)
    }
}

impl From<RegU32> for u32 {
    fn from(value: RegU32) -> Self {
        u32::from(&value)
    }
}

impl From<&RegU32> for u32 {
    fn from(value: &RegU32) -> Self {
        ((value.data[0] as u32) << 16) | (value.data[1] as u32)
    }
}

/// Immutable register iterator for [RegU32].
pub struct RegU32Iter<'a> {
    value: &'a RegU32,
    index: u8
}

/// Mutable register iterator for [RegU32].
pub struct RegU32IterMut<'a> {
    value: &'a mut RegU32,
    index: u8
}

impl<'a> RegU32Iter<'a> {
    pub const fn new(value: &'a RegU32) -> RegU32Iter<'a> {
        RegU32Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegU32IterMut<'a> {
    pub const fn new(value: &'a mut RegU32) -> RegU32IterMut<'a> {
        RegU32IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegU32Iter<'a> {
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

impl<'a> Iterator for RegU32IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..2 => {
                let idx = self.index as usize;
                self.index += 1;
                let value_ptr: *mut RegU32 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[idx])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegU32 {
    type IntoIter<'a> = RegU32Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegU32Iter::new(self)
    }
} 

impl IntoRegIterMut for RegU32 {
    type IntoIterMut<'a> = RegU32IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegU32IterMut::new(self)
    }
}

#[cfg(test)]
mod reg_u32_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegU32::new(0x11223344);
        let reg1 = RegU32::new(0xAABBCCDD);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();

        let mut iter = iter0.chain(iter1);
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0xAABB), iter.next());
        assert_eq!(Some(&0xCCDD), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegU32::new(0x11223344);
        let mut reg1 = RegU32::new(0x55667788);
        let mut reg2 = RegU32::new(0x99AABBCC);
        let mut reg3 = RegU32::new(0xDDEEFF00);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        // check values equal initialization
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x5566), iter.next());
        assert_eq!(Some(&0x7788), iter.next());
        assert_eq!(Some(&0x99AA), iter.next());
        assert_eq!(Some(&0xBBCC), iter.next());
        assert_eq!(Some(&0xDDEE), iter.next());
        assert_eq!(Some(&0xFF00), iter.next());
        assert_eq!(None, iter.next());

        // Change values
        let iter0: RegU32IterMut<'_> = reg0.reg_iter_mut();
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
    fn from_to_u32_001() {
        let reg = RegU32::new(0x11223344);
        let value = u32::from(reg);
        assert_eq!(value, 0x11223344);
    }
}