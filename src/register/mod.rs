mod reg_bool;
pub use reg_bool::{RegBool, RegBoolIter, RegBoolIterMut};

mod reg_u8;
pub use reg_u8::{RegU8, RegU8Iter, RegU8IterMut};

mod reg_u16;
pub use reg_u16::{RegU16, RegU16Iter, RegU16IterMut};

mod reg_u32;
pub use reg_u32::{RegU32, RegU32Iter, RegU32IterMut};

mod reg_u64;
pub use reg_u64::{RegU64, RegU64Iter, RegU64IterMut};

// signed integer based registers

mod reg_i8;
pub use reg_i8::{RegI8, RegI8Iter, RegI8IterMut};

mod reg_i16;
pub use reg_i16::{RegI16, RegI16Iter, RegI16IterMut};

mod reg_i32;
pub use reg_i32::{RegI32, RegI32Iter, RegI32IterMut};

mod reg_i64;
pub use reg_i64::{RegI64, RegI64Iter, RegI64IterMut};

// IEEE-754 floats

mod reg_f32;
pub use reg_f32::{RegF32, RegF32Iter, RegF32IterMut};

mod reg_f64;
pub use reg_f64::{RegF64, RegF64Iter, RegF64IterMut};

/// Trait for creating an immutable register iterator. See [IntoRegIterMut] for mutable register iterator.
pub trait IntoRegIter {
    type IntoIter<'a>: Iterator<Item = &'a u16> where Self: 'a;

    fn into_reg_iter(&self) -> Self::IntoIter<'_>;
}

/// Trait for creating a mutable register iterator. See [IntoRegIter] for an immutable register iterator.
pub trait IntoRegIterMut {
    type IntoIterMut<'a>: Iterator<Item = &'a mut u16> where Self: 'a;

    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_>;
}


/// Read-only register enforced by the type system.
pub struct RO<T>
where 
    T: IntoRegIter
{
    data: T
}

impl<T: IntoRegIter> RO<T> {
    pub const fn new(value: T)-> RO<T> 
    {
        RO {
            data: value
        }
    }

    pub fn reg_iter(&self) -> T::IntoIter<'_> {
        self.data.into_reg_iter()
    }
}

impl<T: IntoRegIter> IntoRegIter for RO<T> {
    type IntoIter<'a> = T::IntoIter<'a> where Self: 'a;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        self.data.into_reg_iter()
    }
}


/// Read and writable register.
pub struct RW<T>
where 
    T: IntoRegIter + IntoRegIterMut
{
    data: T
}

impl<T: IntoRegIter + IntoRegIterMut> RW<T> {
    pub const fn new(value: T)-> RW<T> 
    {
        RW {
            data: value
        }
    }

    pub fn reg_iter(&self) -> T::IntoIter<'_> {
        self.data.into_reg_iter()
    }
}

impl<T: IntoRegIter + IntoRegIterMut> IntoRegIter for RW<T> {
    type IntoIter<'a> = T::IntoIter<'a> where Self: 'a;
    fn into_reg_iter(&self) -> Self::IntoIter<'_> {
        self.data.into_reg_iter()
    }
}

impl<T: IntoRegIter + IntoRegIterMut> IntoRegIterMut for RW<T> {
    type IntoIterMut<'a> = T::IntoIterMut<'a> where Self: 'a;
    fn into_reg_iter_mut(&mut self) -> Self::IntoIterMut<'_> {
        self.data.into_reg_iter_mut()
    }
}


#[cfg(test)]
mod register_tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn reg_iter_001() {
        let reg0 = RegBool::new(true);
        let reg1 = RegU8::new(0x12);
        let reg2 = RegU16::new(0xADAC);
        let reg3 = RegU32::new(0x11223344);
        let reg4 = RegU64::new(0x0123456789ABCDEF);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();
        let iter4 = reg4.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3).chain(iter4); 
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0x0012), iter.next());
        assert_eq!(Some(&0xADAC), iter.next());
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x0123), iter.next());
        assert_eq!(Some(&0x4567), iter.next());
        assert_eq!(Some(&0x89AB), iter.next());
        assert_eq!(Some(&0xCDEF), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_ro_register_001() {
        let reg0 = RegBool::new(true);
        let reg0 = RO::new(reg0);

        let reg1 = RegU8::new(0x12);
        let reg1 = RO::new(reg1);

        let reg2 = RegU16::new(0xADAC);
        let reg2 = RO::new(reg2);

        let reg3 = RegU32::new(0x11223344);
        let reg3 = RO::new(reg3);

        let reg4 = RegU64::new(0x0123456789ABCDEF);
        let reg4 = RO::new(reg4);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();
        let iter4 = reg4.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3).chain(iter4); 
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0x0012), iter.next());
        assert_eq!(Some(&0xADAC), iter.next());
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x0123), iter.next());
        assert_eq!(Some(&0x4567), iter.next());
        assert_eq!(Some(&0x89AB), iter.next());
        assert_eq!(Some(&0xCDEF), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn reg_iter_rw_register_001() {
        let reg0 = RegBool::new(true);
        let reg0 = RW::new(reg0);

        let reg1 = RegU8::new(0x12);
        let reg1 = RW::new(reg1);

        let reg2 = RegU16::new(0xADAC);
        let reg2 = RW::new(reg2);

        let reg3 = RegU32::new(0x11223344);
        let reg3 = RW::new(reg3);

        let reg4 = RegU64::new(0x0123456789ABCDEF);
        let reg4 = RW::new(reg4);

        let iter0 = reg0.reg_iter();
        let iter1 = reg1.reg_iter();
        let iter2 = reg2.reg_iter();
        let iter3 = reg3.reg_iter();
        let iter4 = reg4.reg_iter();

        let mut iter = iter0.chain(iter1).chain(iter2).chain(iter3).chain(iter4); 
        assert_eq!(Some(&0xFFFF), iter.next());
        assert_eq!(Some(&0x0012), iter.next());
        assert_eq!(Some(&0xADAC), iter.next());
        assert_eq!(Some(&0x1122), iter.next());
        assert_eq!(Some(&0x3344), iter.next());
        assert_eq!(Some(&0x0123), iter.next());
        assert_eq!(Some(&0x4567), iter.next());
        assert_eq!(Some(&0x89AB), iter.next());
        assert_eq!(Some(&0xCDEF), iter.next());
        assert_eq!(None, iter.next());
    }


    #[test]
    fn size_of_ro_001() {
        assert_eq!(size_of::<RO<RegBool>>(), size_of::<RegBool>());
        assert_eq!(size_of::<RO<RegU8>>(), size_of::<RegU8>());
        assert_eq!(size_of::<RO<RegU16>>(), size_of::<RegU16>());
        assert_eq!(size_of::<RO<RegU32>>(), size_of::<RegU32>());
        assert_eq!(size_of::<RO<RegU64>>(), size_of::<RegU64>());
        assert_eq!(size_of::<RO<RegI8>>(), size_of::<RegI8>());
        assert_eq!(size_of::<RO<RegI16>>(), size_of::<RegI16>());
        assert_eq!(size_of::<RO<RegI32>>(), size_of::<RegI32>());
        assert_eq!(size_of::<RO<RegI64>>(), size_of::<RegI64>());
        assert_eq!(size_of::<RO<RegF32>>(), size_of::<RegF32>());
        assert_eq!(size_of::<RO<RegF64>>(), size_of::<RegF64>());
    }

}