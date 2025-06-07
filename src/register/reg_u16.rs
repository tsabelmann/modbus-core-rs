use super::{IntoRegIter, IntoRegIterMut};


/// Modbus register that can be converter to and from [u16].
pub struct RegU16 {
    data: [u16; 1]
}

impl RegU16 {
    /// Creates a new [u16] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU16;
    /// 
    /// let reg = RegU16::new(42);
    /// ````
    pub const fn new(value: u16) -> RegU16 {
        RegU16 {
            data: [value]
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU16;
    /// 
    /// let reg = RegU16::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegU16Iter<'_> {
        RegU16Iter::new(self)
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU16;
    /// 
    /// let mut reg = RegU16::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegU16IterMut<'_> {
        RegU16IterMut::new(self)
    }
}

impl From<u16> for RegU16 {
    fn from(value: u16) -> Self {
        RegU16::new(value)
    }
}

impl From<&u16> for RegU16 {
    fn from(value: &u16) -> Self {
        RegU16::new(*value)
    }
}

impl From<RegU16> for u16 {
    fn from(value: RegU16) -> Self {
        u16::from(&value)
    }
}

impl From<&RegU16> for u16 {
    fn from(value: &RegU16) -> Self {
        value.data[0]
    }
}

/// Immutable register iterator for [RegU16].
pub struct RegU16Iter<'a> {
    value: &'a RegU16,
    index: u8
}

/// Mutable register iterator for [RegU16].
pub struct RegU16IterMut<'a> {
    value: &'a mut RegU16,
    index: u8
}

impl<'a> RegU16Iter<'a> {
    pub const fn new(value: &'a RegU16) -> RegU16Iter<'a> {
        RegU16Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegU16IterMut<'a> {
    pub const fn new(value: &'a mut RegU16) -> RegU16IterMut<'a> {
        RegU16IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegU16Iter<'a> {
    type Item = &'a u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(&self.value.data[0])
            },
            _ => None
        }
    }
}

impl<'a> Iterator for RegU16IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                let value_ptr: *mut RegU16 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[0])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegU16 {
    type IntoIter<'a> = RegU16Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegU16Iter::new(self)
    }
} 

impl IntoRegIterMut for RegU16 {
    type IntoIterMut<'a> = RegU16IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegU16IterMut::new(self)
    }
}


#[cfg(test)]
mod reg_u16_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegU16::new(0x1234);
        let reg1 = RegU16::new(0x5678);
        let reg2 = RegU16::new(0x9ABC);
        let reg3 = RegU16::new(0xDEF0);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0x1234), iter.next());
        assert_eq!(Some(&0x5678), iter.next());
        assert_eq!(Some(&0x9ABC), iter.next());
        assert_eq!(Some(&0xDEF0), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegU16::new(0x1234);
        let mut reg1 = RegU16::new(0x5678);
        let mut reg2 = RegU16::new(0x9ABC);
        let mut reg3 = RegU16::new(0xDEF0);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0x1234), iter.next());
        assert_eq!(Some(&0x5678), iter.next());
        assert_eq!(Some(&0x9ABC), iter.next());
        assert_eq!(Some(&0xDEF0), iter.next());
        assert_eq!(None, iter.next());

        // Change values
        let iter0 = reg0.reg_iter_mut();
        let iter1 = reg1.reg_iter_mut();
        let iter2 = reg2.reg_iter_mut();
        let iter3 = reg3.reg_iter_mut();
        
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        if let Some(reff) = iter.next() {
            *reff = 42;
        }

        if let Some(reff) = iter.next() {
            *reff = 1337;
        }

        if let Some(reff) = iter.next() {
            *reff = 65525;
        }

        if let Some(reff) = iter.next() {
            *reff = 1;
        }

        // check for change
        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();
        
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&42), iter.next());
        assert_eq!(Some(&1337), iter.next());
        assert_eq!(Some(&65525), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn from_to_u8_001() {
        let reg = RegU16::new(42);
        let value = u16::from(reg);
        assert_eq!(value, 42);
    }

    #[test]
    fn from_to_u8_002() {
        let reg = RegU16::new(0x1234);
        let value = u16::from(reg);
        assert_eq!(value, 0x1234);
    }

}