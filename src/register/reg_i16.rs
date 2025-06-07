use super::{IntoRegIter, IntoRegIterMut};


/// Modbus register that can be converter to and from [i16].
pub struct RegI16 {
    data: [u16; 1]
}

impl RegI16 {
    /// Creates a new [i16] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI16;
    /// 
    /// let reg = RegI16::new(42);
    /// ````
    pub const fn new(value: i16) -> RegI16 {
        RegI16 {
            data: [value as u16]
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI16;
    /// 
    /// let reg = RegI16::new(42);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegI16Iter<'_> {
        RegI16Iter::new(self)
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegI16;
    /// 
    /// let mut reg = RegI16::new(42);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegI16IterMut<'_> {
        RegI16IterMut::new(self)
    }
}

impl From<i16> for RegI16 {
    fn from(value: i16) -> Self {
        RegI16::new(value)
    }
}

impl From<&i16> for RegI16 {
    fn from(value: &i16) -> Self {
        RegI16::new(*value)
    }
}

impl From<RegI16> for i16 {
    fn from(value: RegI16) -> Self {
        i16::from(&value)
    }
}

impl From<&RegI16> for i16 {
    fn from(value: &RegI16) -> Self {
        value.data[0] as i16
    }
}

/// Immutable register iterator for [RegI16].
pub struct RegI16Iter<'a> {
    value: &'a RegI16,
    index: u8
}

/// Mutable register iterator for [RegI16].
pub struct RegI16IterMut<'a> {
    value: &'a mut RegI16,
    index: u8
}

impl<'a> RegI16Iter<'a> {
    pub const fn new(value: &'a RegI16) -> RegI16Iter<'a> {
        RegI16Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegI16IterMut<'a> {
    pub const fn new(value: &'a mut RegI16) -> RegI16IterMut<'a> {
        RegI16IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegI16Iter<'a> {
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

impl<'a> Iterator for RegI16IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                let value_ptr: *mut RegI16 = self.value;
                
                // unsafe nötig, weil wir &'a mut i16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[0])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegI16 {
    type IntoIter<'a> = RegI16Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegI16Iter::new(self)
    }
} 

impl IntoRegIterMut for RegI16 {
    type IntoIterMut<'a> = RegI16IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegI16IterMut::new(self)
    }
}


#[cfg(test)]
mod reg_i16_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegI16::new(-1);
        let reg1 = RegI16::new(-2);
        let reg2 = RegI16::new(-3);
        let reg3 = RegI16::new(-4);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFE), iter.next());
        assert_eq!(Some(&0xFFFD), iter.next());
        assert_eq!(Some(&0xFFFC), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegI16::new(-1);
        let mut reg1 = RegI16::new(-2);
        let mut reg2 = RegI16::new(-3);
        let mut reg3 = RegI16::new(-4);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFE), iter.next());
        assert_eq!(Some(&0xFFFD), iter.next());
        assert_eq!(Some(&0xFFFC), iter.next());
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
    fn from_to_i16_001() {
        let reg = RegI16::new(-2);
        let value = i16::from(reg);
        assert_eq!(value, -2);
    }
}