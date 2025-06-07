use super::{IntoRegIter, IntoRegIterMut};


/// Modbus register that can be converter to and from [bool].
pub struct RegBool {
    data: [u16; 1]
}

impl RegBool {
    /// Creates a new [bool] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegBool;
    /// 
    /// let reg = RegBool::new(true);
    /// ````
    pub const fn new(value: bool) -> RegBool {
        let data = match value {
            false => [0x0000],
            true => [0xFFFF]
        };
        
        RegBool {
            data
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegBool;
    /// 
    /// let reg = RegBool::new(false);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegBoolIter<'_> {
        RegBoolIter::new(self)
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegBool;
    /// 
    /// let mut reg = RegBool::new(false);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegBoolIterMut<'_> {
        RegBoolIterMut::new(self)
    }
}

impl From<bool> for RegBool {
    fn from(value: bool) -> Self {
        RegBool::new(value)
    }
}

impl From<&bool> for RegBool {
    fn from(value: &bool) -> Self {
        RegBool::new(*value)
    }
}

impl From<RegBool> for bool {
    fn from(value: RegBool) -> Self {
        bool::from(&value)
    }
}

impl From<&RegBool> for bool {
    fn from(value: &RegBool) -> Self {
        match value.data[0] {
            0x0000 => false,
            _ => true
        }
    }
}

/// Immutable register iterator for [RegBool].
pub struct RegBoolIter<'a> {
    value: &'a RegBool,
    index: u8
}

/// Mutable register iterator for [RegBool].
pub struct RegBoolIterMut<'a> {
    value: &'a mut RegBool,
    index: u8
}

impl<'a> RegBoolIter<'a> {
    pub const fn new(value: &'a RegBool) -> RegBoolIter<'a> {
        RegBoolIter {
            value,
            index: 0
        }
    }
}

impl<'a> RegBoolIterMut<'a> {
    pub const fn new(value: &'a mut RegBool) -> RegBoolIterMut<'a> {
        RegBoolIterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegBoolIter<'a> {
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

impl<'a> Iterator for RegBoolIterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                let value_ptr: *mut RegBool = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[0])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegBool {
    type IntoIter<'a> = RegBoolIter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegBoolIter::new(self)
    }
} 

impl IntoRegIterMut for RegBool {
    type IntoIterMut<'a> = RegBoolIterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegBoolIterMut::new(self)
    }
}


#[cfg(test)]
mod reg_bool_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegBool::new(false);
        let reg1 = RegBool::new(true);
        let reg2 = RegBool::new(false);
        let reg3 = RegBool::new(true);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegBool::new(false);
        let mut reg1 = RegBool::new(true);
        let mut reg2 = RegBool::new(true);
        let mut reg3 = RegBool::new(false);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();

        // check values equal initialization
        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3);
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
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
        let reg = RegBool::new(true);
        let value = bool::from(reg);
        assert_eq!(value, true);
    }

}