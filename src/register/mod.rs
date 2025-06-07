mod reg_bool;

mod reg_u8;
pub use reg_u8::{RegU8, RegU8Iter, RegU8IterMut};

mod reg_u16;
pub use reg_u16::{RegU16, RegU16Iter, RegU16IterMut};

mod reg_u32;
pub use reg_u32::{RegU32, RegU32Iter, RegU32IterMut};

mod reg_u64;

// signed integer based registers

mod reg_i8;
mod reg_i16;
mod reg_i32;
mod reg_i64;

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
