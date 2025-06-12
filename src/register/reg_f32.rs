use super::{IntoRegIter, IntoRegIterMut, GetRegisterValue, SetRegisterValue};

/// Modbus register that can be converter to and from [f32].
pub struct RegF32 {
    data: [u16; 2]
}

impl RegF32 {
    /// Creates a new [f32] based modbus register.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF32;
    /// 
    /// let reg = RegF32::new(42.0);
    /// ````
    pub const fn new(value: f32) -> RegF32 {
        let data = f32::to_be_bytes(value);
        let data = [
            ((data[0] as u16) << 8) | (data[1] as u16),
            ((data[2] as u16) << 8) | (data[3] as u16),
        ];
        
        RegF32 {
            data
        }
    }

    /// Creates an immutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF32;
    /// 
    /// let reg = RegF32::new(42.0);
    /// let iter = reg.reg_iter();
    /// for value in iter {
    ///     println!("value={}", value);
    /// }
    /// ````
    pub const fn reg_iter(&self) -> RegF32Iter<'_> {
        RegF32Iter::new(self)
    }

    /// Creates a mutable register iterator.
    /// 
    /// # Example
    ///
    /// ```
    /// use modbus_stack::register::RegF32;
    /// 
    /// let mut reg = RegF32::new(42.0);
    /// let iter = reg.reg_iter_mut();
    /// for value in iter {
    ///     *value = 0xADAC;
    /// }
    /// ````
    pub const fn reg_iter_mut(&mut self) -> RegF32IterMut<'_> {
        RegF32IterMut::new(self)
    }
}

impl From<f32> for RegF32 {
    fn from(value: f32) -> Self {
        RegF32::new(value)
    }
}

impl From<&f32> for RegF32 {
    fn from(value: &f32) -> Self {
        RegF32::new(*value)
    }
}

impl From<RegF32> for f32 {
    fn from(value: RegF32) -> Self {
        f32::from(&value)
    }
}

impl From<&RegF32> for f32 {
    fn from(value: &RegF32) -> Self {
        let mut data = [0u8; 4];
        data[0] = ((value.data[0] >> 8) & 0xFF) as u8;
        data[1] = (value.data[0] & 0xFF) as u8;
        data[2] = ((value.data[1] >> 8) & 0xFF) as u8;
        data[3] = (value.data[1] & 0xFF) as u8;
        f32::from_be_bytes(data)
    }
}

impl SetRegisterValue<f32> for RegF32 {
    fn set_value(&mut self, value: f32) {
        *self = RegF32::from(value);
    }
}

impl GetRegisterValue<f32> for RegF32 {
    fn get_value(&self) -> f32 {
        f32::from(self)
    }
}

/// Immutable register iterator for [RegF32].
pub struct RegF32Iter<'a> {
    value: &'a RegF32,
    index: u8
}

/// Mutable register iterator for [RegF32].
pub struct RegF32IterMut<'a> {
    value: &'a mut RegF32,
    index: u8
}

impl<'a> RegF32Iter<'a> {
    pub const fn new(value: &'a RegF32) -> RegF32Iter<'a> {
        RegF32Iter {
            value,
            index: 0
        }
    }
}

impl<'a> RegF32IterMut<'a> {
    pub const fn new(value: &'a mut RegF32) -> RegF32IterMut<'a> {
        RegF32IterMut {
            value,
            index: 0
        }
    }
}

impl<'a> Iterator for RegF32Iter<'a> {
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

impl<'a> Iterator for RegF32IterMut<'a> {
    type Item = &'a mut u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0..2 => {
                let idx = self.index as usize;
                self.index += 1;
                let value_ptr: *mut RegF32 = self.value;
                
                // unsafe nötig, weil wir &'a mut u16 aus &mut self.value extrahieren wollen
                unsafe {
                    Some(&mut (*value_ptr).data[idx])
                }
            },
            _ => None
        }
    }
}

impl IntoRegIter for RegF32 {
    type IntoIter<'a> = RegF32Iter<'a>;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        RegF32Iter::new(self)
    }
} 

impl IntoRegIterMut for RegF32 {
    type IntoIterMut<'a> = RegF32IterMut<'a>;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        RegF32IterMut::new(self)
    }
}

#[cfg(test)]
mod reg_f32_tests {
    use super::*;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegF32::new(0.0);
        let mut iter = reg0.reg_iter();
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_mut_001() {
        let mut reg0 = RegF32::new(0.0);

        // check values equal initialization
        let mut iter = reg0.reg_iter();
        assert_eq!(Some(&0x0000), iter.next());
        assert_eq!(Some(&0x0000), iter.next());

        let data = {
            let data = 42.0f32.to_be_bytes();
            let data = [
                ((data[0] as u16) << 8) | (data[1] as u16),
                ((data[2] as u16) << 8) | (data[3] as u16),
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

        // check for change
        let value = f32::from(reg0);
        assert_eq!(value, 42.0f32);
    }

    #[test]
    fn from_to_f32_001() {
        let reg = RegF32::new(42.0);
        let value = f32::from(reg);
        assert_eq!(value, 42.0);
    }
}