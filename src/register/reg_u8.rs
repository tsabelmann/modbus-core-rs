use super::{IntoRegIter, IntoRegIterMut};


/// Modbus register that can be converter to and from [u8].
pub struct RegU8 {
    data: [u16; 1]
}

impl RegU8 {
    /// Creates a new u8 based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU8;
    /// 
    /// let reg = RegU8::new(42);
    /// ````
    pub const fn new(value: u8) -> RegU8 {
        RegU8 {
            data: [(value as u16) & 0xFF]
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU8;
    /// 
    /// let reg = RegU8::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegU8Iter<'_> {
        RegU8Iter::new(self)
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegU8;
    /// 
    /// let mut reg = RegU8::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegU8IterMut<'_> {
        RegU8IterMut::new(self)
    }
}

impl From<u8> for RegU8 {
    fn from(value: u8) -> Self {
        RegU8::new(value)
    }
}

impl From<&u8> for RegU8 {
    fn from(value: &u8) -> Self {
        RegU8::new(*value)
    }
}

impl From<RegU8> for u8 {
    fn from(value: RegU8) -> Self {
        u8::from(&value)
    }
}

impl From<&RegU8> for u8 {
    fn from(value: &RegU8) -> Self {
        (value.data[0] & 0xFF) as u8
    }
}

/// Immutable register iterator for [RegU8].
pub struct RegU8Iter<'a> {
    value: &'a RegU8,
    index: u8
}

/// Mutable register iterator for [RegU8].
pub struct RegU8IterMut<'a> {
    value: &'a mut RegU8,
    index: u8
}

impl<'a> RegU8Iter<'a> {
    pub const fn new(value: &'a RegU8) -> RegU8Iter<'a> {
        RegU8Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegU8IterMut<'a> {
    pub const fn new(value: &'a mut RegU8) -> RegU8IterMut<'a> {
        RegU8IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegU8Iter<'a> {
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

impl<'a> Iterator for RegU8IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                let value_ptr: *mut RegU8 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[0])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegU8 {
    type IntoIter<'a> = RegU8Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegU8Iter::new(self)
    }
} 

impl IntoRegIterMut for RegU8 {
    type IntoIterMut<'a> = RegU8IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegU8IterMut::new(self)
    }
}


#[cfg(test)]
mod reg_u8_tests {
    use super::*;

    #[test]
    fn iter_001() {
        let reg0 = RegU8::new(0x00);
        let reg1 = RegU8::new(0x01);
        let reg2 = RegU8::new(0x02);
        let reg3 = RegU8::new(0x03);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut_001() {
        let mut reg0 = RegU8::new(0x00);
        let mut reg1 = RegU8::new(0x01);
        let mut reg2 = RegU8::new(0x02);
        let mut reg3 = RegU8::new(0x03);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        // check values equal initialization
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
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
        let reg = RegU8::new(42);
        let value = u8::from(reg);
        assert_eq!(value, 42);
    }

}