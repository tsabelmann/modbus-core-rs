mod reg_u8;
pub use reg_u8::{AtomicRegU8, AtomicRegU8Ref};

// use core::sync::atomic::AtomicU32;
// use core::sync::atomic::Ordering;

// use crate::register::{WriteRegister, ReadRegister};


// #[derive(Debug, Clone)]
// pub struct RegAtomicU32<'a> {
//     ptr: &'a AtomicU32
// }

// impl<'a> RegAtomicU32<'a> {
//     pub const fn new(value: &'a AtomicU32) -> RegAtomicU32<'a> {
//         RegAtomicU32 { ptr: value }
//     }
// }

// impl<'a> WriteRegister<u32> for RegAtomicU32<'a> {
//     fn write(&mut self, value: u32) {
//         self.ptr.store(value, Ordering::Release);
//     }
// }

// impl<'a> ReadRegister<u32> for RegAtomicU32<'a> {
//     fn read(self) -> u32 {
//         self.ptr.load(Ordering::Acquire)
//     }
// }

// pub trait LendingIterator {
//     type Item<'item> where Self: 'item;

//     fn next(&mut self) -> Option<Self::Item<'_>>;
// }


// pub struct AtomicU32Iter<'a> {
//     ptr: &'a AtomicU32,
//     cached: u32,
//     value: u16,
//     idx: u8,
// }

// impl<'a> Iterator for AtomicU32Iter<'a> {
//     type Item = &'a u16 where Self: 'a;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.idx {
//             0 => {
//                 let value = self.ptr.load(Ordering::Acquire);
//                 self.value = (value >> 16) as u16;
//                 self.idx += 1;
//                 Some(unsafe { &*(&self.value as *const u16) })
//             },
//             1 => {
//                 let value = self.ptr.load(Ordering::Acquire);
//                 self.value = value as u16;
//                 self.idx += 1;
//                 Some(unsafe { &*(&self.value as *const u16) })
//             },
//             _ => None
//         }
//     }   
// }

// impl<'a> AtomicU32Iter<'a> {
//     pub const fn new(ptr: &'a AtomicU32) -> AtomicU32Iter<'a> {
//         AtomicU32Iter {
//             ptr,
//             cached: 0,
//             value: 0,
//             idx: 0,
//         }
//     }

//     fn write_back(&mut self) {
//         if self.idx == 0 {
//             return;
//         }
//         let mask = 0xFFFF_u32 << (16 * (2 - self.idx));
//         self.cached &= !mask;
//         self.cached |= (self.value as u32) << (16 * (2 - self.idx));
//         self.ptr.store(self.cached, Ordering::Release);
//     }
// }

// impl<'a> LendingIterator for AtomicU32Iter<'a> {
//     type Item<'item> = &'item mut u16 where Self: 'item;

//     fn next(self: &mut Self) -> Option<Self::Item<'_>> {
//         match self.idx {
//             0 => {
//                 self.cached = self.ptr.load(Ordering::Relaxed);
//                 self.value = (self.cached >> 16) as u16;
//                 self.idx = 1;
//                 Some(&mut self.value)
//             }
//             1 => {
//                 self.value = (self.cached & 0xFFFF) as u16;
//                 self.idx = 2;
//                 Some(&mut self.value)
//             }
//             _ => None,
//         }
//     }
// }

// // fn iter<T>(mut iterator: T)
// // where
// //     T: LendingIterator,
// //     for<'item> T::Item<'item>: Deref<Target = u16>,
// // {
// //     while let Some(elem) = iterator.next() {
// //         let reff = elem.deref();
// //         println!("reff: {}")
// //     }

// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use core::sync::atomic::AtomicU32;
//     use core::iter::Iterator;

//     #[test]
//     fn my_test() {
//         let atomic = AtomicU32::new(0x1234_5678);
//         let mut iter = AtomicU32Iter::new(&atomic);

//         assert_eq!(Iterator::next(&mut iter), Some(&0x1234));
//         assert_eq!(Iterator::next(&mut iter), Some(&0x5678));
//         assert_eq!(Iterator::next(&mut iter), None);
//     }
// }