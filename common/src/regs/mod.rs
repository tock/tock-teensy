//! Implementation of registers and bitfields
//!

#[macro_use]
pub mod macros;

use core::ops::{BitAnd, BitOr, Not, Shr, Shl, Add};

pub trait IntLike: BitAnd<Output=Self> +
                   BitOr<Output=Self> +
                   Not<Output=Self> +
                   Shr<u32, Output=Self> +
                   Shl<u32, Output=Self> + Copy + Clone {}

impl IntLike for u8 {}
impl IntLike for u16 {}
impl IntLike for u32 {}

pub struct RW<T: IntLike> {
    value: T,
}

pub struct RO<T: IntLike> {
    value: T,
}

pub struct WO<T: IntLike> {
    value: T,
}

#[allow(dead_code)]
impl<T: IntLike> RW<T> {
    pub const fn new(value: T) -> Self {
        RW { value: value }
    }

    #[inline]
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(&self.value) }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(&self.value as *const T as *mut T, value) }
    }

    #[inline]
    pub fn read(&self, field: Field<T>) -> T
    {
        (self.get() & (field.mask << field.shift)) >> field.shift
    }

    #[inline]
    pub fn write(&self, field: FieldValue<T>) {
        self.set(field.value);
    }

    #[inline]
    pub fn modify(&self, field: FieldValue<T>) {
        let reg: T = self.get();
        self.set((reg & !field.mask) | field.value);
    }

    #[inline]
    pub fn is_set(&self, field: Field<T>) -> bool 
        where T: PartialEq<u8> {
        self.read(field) != 0
    }

    #[inline]
    pub fn matches(&self, field: FieldValue<T>) -> bool
        where T: Eq {
        self.get() & field.mask == field.value
    }
}

#[allow(dead_code)]
impl<T: IntLike> RO<T> {
    pub const fn new(value: T) -> Self {
        RO { value: value }
    }

    #[inline]
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(&self.value) }
    }

    #[inline]
    pub fn read(&self, field: Field<T>) -> T
    {
        (self.get() & (field.mask << field.shift)) >> field.shift
    }

    #[inline]
    pub fn is_set(&self, field: Field<T>) -> bool 
        where T: PartialEq<u8> {
        self.read(field) != 0
    }

    #[inline]
    pub fn matches(&self, field: FieldValue<T>) -> bool
        where T: Eq {
        self.get() & field.mask == field.value
    }
}

#[allow(dead_code)]
impl<T: IntLike> WO<T> {
    pub const fn new(value: T) -> Self {
        WO { value: value }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(&self.value as *const T as *mut T, value) }
    }

    #[inline]
    pub fn write(&self, field: FieldValue<T>) {
        self.set(field.value);
    }
}

#[derive(Copy, Clone)]
pub struct Field<T: IntLike> {
    mask: T,
    shift: u32
}

// For the Field, the mask is unshifted, ie. the LSB should always be set
impl Field<u8> {
    pub const fn new(mask: u8, shift: u32) -> Field<u8> {
        Field {
            mask: mask,
            shift: shift
        }
    }

    pub fn val(&self, value: u8) -> FieldValue<u8> {
        FieldValue::<u8>::new(self.mask, self.shift, value)
    }
}

impl Field<u16> {
    pub const fn new(mask: u16, shift: u32) -> Field<u16> {
        Field {
            mask: mask,
            shift: shift
        }
    }

    pub fn val(&self, value: u16) -> FieldValue<u16> {
        FieldValue::<u16>::new(self.mask, self.shift, value)
    }
}

impl Field<u32> {
    pub const fn new(mask: u32, shift: u32) -> Field<u32> {
        Field {
            mask: mask,
            shift: shift
        }
    }

    pub fn val(&self, value: u32) -> FieldValue<u32> {
        FieldValue::<u32>::new(self.mask, self.shift, value)
    }
}


// For the FieldValue, the masks and values are shifted into their actual location in the register
#[derive(Copy, Clone)]
pub struct FieldValue<T: IntLike> {
    mask: T,
    value: T
}

// Necessary to split the implementation of u8 and u32 out because the bitwise math isn't treated
// as const when the type is generic
impl FieldValue<u8> {
    pub const fn new(mask: u8, shift: u32, value: u8) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value << shift) & (mask << shift)
        }
    }
}

impl FieldValue<u16> {
    pub const fn new(mask: u16, shift: u32, value: u16) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value << shift) & (mask << shift)
        }
    }
}

impl FieldValue<u32> {
    pub const fn new(mask: u32, shift: u32, value: u32) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value << shift) & (mask << shift)
        }
    }
}

// Combine two fields with the addition operator
impl<T: IntLike> Add for FieldValue<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FieldValue {
            mask: self.mask | rhs.mask,
            value: self.value | rhs.value
        }
    }
}
