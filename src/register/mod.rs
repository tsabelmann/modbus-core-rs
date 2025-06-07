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
mod reg_f64;

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
