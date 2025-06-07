use super::{IntoRegIter, IntoRegIterMut};


/// Modbus register that can be converter to and from [i8].
pub struct RegI8 {
    data: [u16; 1]
}

impl RegI8 {
    /// Creates a new i8 based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI8;
    /// 
    /// let reg = RegI8::new(42);
    /// ````
    pub const fn new(value: i8) -> RegI8 {
        RegI8 {
            data: [(value as u16) & 0xFF]
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI8;
    /// 
    /// let reg = RegI8::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegI8Iter<'_> {
        RegI8Iter::new(self)
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI8;
    /// 
    /// let mut reg = RegI8::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegI8IterMut<'_> {
        RegI8IterMut::new(self)
    }
}

impl From<i8> for RegI8 {
    fn from(value: i8) -> Self {
        RegI8::new(value)
    }
}

impl From<&i8> for RegI8 {
    fn from(value: &i8) -> Self {
        RegI8::new(*value)
    }
}

impl From<RegI8> for i8 {
    fn from(value: RegI8) -> Self {
        i8::from(&value)
    }
}

impl From<&RegI8> for i8 {
    fn from(value: &RegI8) -> Self {
        (value.data[0] & 0xFF) as i8
    }
}

/// Immutable register iterator for [RegI8].
pub struct RegI8Iter<'a> {
    value: &'a RegI8,
    index: i8
}

/// Mutable register iterator for [RegI8].
pub struct RegI8IterMut<'a> {
    value: &'a mut RegI8,
    index: i8
}

impl<'a> RegI8Iter<'a> {
    pub const fn new(value: &'a RegI8) -> RegI8Iter<'a> {
        RegI8Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegI8IterMut<'a> {
    pub const fn new(value: &'a mut RegI8) -> RegI8IterMut<'a> {
        RegI8IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegI8Iter<'a> {
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

impl<'a> Iterator for RegI8IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                let value_ptr: *mut RegI8 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[0])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegI8 {
    type IntoIter<'a> = RegI8Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegI8Iter::new(self)
    }
} 

impl IntoRegIterMut for RegI8 {
    type IntoIterMut<'a> = RegI8IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegI8IterMut::new(self)
    }
}


#[cfg(test)]
mod reg_i8_tests {
    use super::*;

    #[test]
    fn iter_001() {
        let reg0 = RegI8::new(-1);
        let reg1 = RegI8::new(-2);
        let reg2 = RegI8::new(-3);
        let reg3 = RegI8::new(-4);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0x00FF), iter.next());
        assert_eq!(Some(&0x00FE), iter.next());
        assert_eq!(Some(&0x00FD), iter.next());
        assert_eq!(Some(&0x00FC), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut_001() {
        let mut reg0 = RegI8::new(0x00);
        let mut reg1 = RegI8::new(0x01);
        let mut reg2 = RegI8::new(0x02);
        let mut reg3 = RegI8::new(0x03);

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
    fn from_to_i8_001() {
        let reg = RegI8::new(-1);
        let value = i8::from(reg);
        assert_eq!(value, -1);
    }

}