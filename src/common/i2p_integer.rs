use std::marker::PhantomData;
use std::fmt;
use std::convert::From;
use std::ops::{Add, Sub, Mul, Div, Rem, Not, BitAnd, BitOr, BitXor};
use std::ops::{Shl, Shr};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use std::cmp::{PartialOrd, Ord, Ordering};
use serialize::byteorder;
use serialize;
use rand;


pub const I2P_INTEGER_SIZE: usize = 8;

pub trait I2pIntSize: Clone + Copy + Eq + PartialEq {
    fn len() -> usize;
}

macro_rules! i2p_int_size_impl {
    ($TYPE_NAME:ty, $LENGTH:expr) => {
        impl I2pIntSize for $TYPE_NAME {
            fn len() -> usize { $LENGTH }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _1 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _2 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _3 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _4 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _5 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _6 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _7 {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum _8 {}

i2p_int_size_impl!(_1, 1);
i2p_int_size_impl!(_2, 2);
i2p_int_size_impl!(_3, 3);
i2p_int_size_impl!(_4, 4);
i2p_int_size_impl!(_5, 5);
i2p_int_size_impl!(_6, 6);
i2p_int_size_impl!(_7, 7);
i2p_int_size_impl!(_8, 8);


pub trait I2pIntMask {
    fn mask() -> u64;
}

macro_rules! mask_impl {
    ( $ T : ty , $ mask : expr ) => {
        impl I2pIntMask for $T {
            #[inline]
            fn mask() -> u64 {
                $mask as u64
            }
        }
    }
}

mask_impl!(_1, 0x00FF);
mask_impl!(_2, 0xFFFF);
mask_impl!(_3, 0x00FF_FFFF);
mask_impl!(_4, 0xFFFF_FFFF);
mask_impl!(_5, 0x00FF_FFFF_FFFF);
mask_impl!(_6, 0xFFFF_FFFF_FFFF);
mask_impl!(_7, 0x00FF_FFFF_FFFF_FFFF);
mask_impl!(_8, 0xFFFF_FFFF_FFFF_FFFF);


/// I2P Integer
/// Represents a variable sized integer from 1 to 8 bytes long in
/// network (big endian) byte order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Hash)]
pub struct I2pInteger<I> {
    data: u64,
    _marker: PhantomData<I>
}

impl<I> I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    pub fn new(data: u64) -> I2pInteger<I> {
        let mask = <I as I2pIntMask>::mask();

        I2pInteger {
            data: data & mask,
            _marker: PhantomData
        }
    }

    /// Converts a byte sequence into an I2pInteger, in network byte (big endian) order.
    /// If a byte array is of length zero, then `from_bytes_be` returns zero.
    /// If a byte array is longer than the maximum number bytes for an I2pInteger,
    /// it return None.
    pub fn from_bytes_be(bytes: &[u8]) -> Option<I2pInteger<I>> {
        if bytes.len() > I2P_INTEGER_SIZE {
            return None;
        }

        let mask = <I as I2pIntMask>::mask();
        let mut result: u64 = 0x00;

        for byte in bytes {
            result <<= 8;
            result = result | (*byte as u64);
        }

        if byteorder::is_big_endian() {
            result = result.swap_bytes();
        }

        Some(I2pInteger::new(result & mask))
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut data  = self.data.to_be();
        let mask = 0xFF;

        while data > 0 {
            let byte = (data & mask) as u8;
            data >>= 8;
            bytes.push(byte);
        }

        bytes
    }

    pub fn to_u64(&self) -> u64 {
        self.data
    }
}

// Type synonyms for I2pInteger sizes.
pub type I2pInt8  = I2pInteger<_1>;
pub type I2pInt16 = I2pInteger<_2>;
pub type I2pInt24 = I2pInteger<_3>;
pub type I2pInt32 = I2pInteger<_4>;
pub type I2pInt40 = I2pInteger<_5>;
pub type I2pInt48 = I2pInteger<_6>;
pub type I2pInt56 = I2pInteger<_7>;
pub type I2pInt64 = I2pInteger<_8>;


impl<I> fmt::Display for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<I> serialize::Serialize for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn serialize(&self, buf: &mut [u8]) -> serialize::Result<usize> {
        if I::len() <= buf.len() {
            let bytes      = self.to_bytes_be();
            let byte_slice: &[u8] = bytes.as_ref();
            for i in 0..byte_slice.len() {
                buf[i] = byte_slice[i];
            }
            Ok(byte_slice.len())
        } else {
            Err(serialize::Error::buffer_too_small(I::len(), buf.len()))
        }
    }
}

impl<I> serialize::Deserialize for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn deserialize(buf: &[u8]) -> serialize::Result<I2pInteger<I>> {
        if I::len() <= buf.len() {
            let mask = <I as I2pIntMask>::mask();
            let mut result: u64 = 0x00;

            for i in 0..I::len() {
                result <<= 8;
                result = result | (buf[i] as u64);
            }

            if byteorder::is_big_endian() {
                result = result.swap_bytes();
            }

            Ok(I2pInteger::new(result & mask))
        } else {
            Err(serialize::Error::buffer_too_small(I::len(), buf.len()))
        }
    }
}


macro_rules! primitive_int_type_to_i2p_int_from_impl {
    ( $ T : ty ) => {
        impl<I> From<$T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            fn from(val: $T) -> I2pInteger<I> {
                let mask: u64 = <I as I2pIntMask>::mask();
                let data: u64 = 0x00;

                I2pInteger::<I>::new((data | val as u64) & mask)
            }
        }
    }
}

primitive_int_type_to_i2p_int_from_impl!(u8);
primitive_int_type_to_i2p_int_from_impl!(u16);
primitive_int_type_to_i2p_int_from_impl!(u32);
primitive_int_type_to_i2p_int_from_impl!(u64);
primitive_int_type_to_i2p_int_from_impl!(usize);
primitive_int_type_to_i2p_int_from_impl!(i8);
primitive_int_type_to_i2p_int_from_impl!(i16);
primitive_int_type_to_i2p_int_from_impl!(i32);
primitive_int_type_to_i2p_int_from_impl!(i64);
primitive_int_type_to_i2p_int_from_impl!(isize);


impl<I> Add<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn add(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data + other.data)
    }
}

impl<'a, I> Add<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn add(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data + other.data)
    }
}

impl<'a, I> Add<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn add(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data + other.data)
    }
}

impl<'a, 'b, I> Add<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn add(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data + other.data)
    }
}

impl<I> Sub<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn sub(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data - other.data)
    }
}

impl<'a, I> Sub<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn sub(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data - other.data)
    }
}

impl<'a, I> Sub<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn sub(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data - other.data)
    }
}

impl<'a, 'b, I> Sub<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn sub(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data - other.data)
    }
}

impl<I> Mul<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn mul(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data * other.data)
    }
}

impl<'a, I> Mul<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn mul(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data * other.data)
    }
}

impl<'a, I> Mul<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn mul(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data * other.data)
    }
}

impl<'a, 'b, I> Mul<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn mul(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data * other.data)
    }
}

impl<I> Div<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn div(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data / other.data)
    }
}

impl<'a, I> Div<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn div(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data / other.data)
    }
}

impl<'a, I> Div<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn div(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data / other.data)
    }
}

impl<'a, 'b, I> Div<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn div(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data / other.data)
    }
}

impl<I> Rem<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn rem(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data % other.data)
    }
}

impl<'a, I> Rem<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn rem(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data % other.data)
    }
}

impl<'a, I> Rem<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn rem(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data % other.data)
    }
}

impl<'a, 'b, I> Rem<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn rem(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data % other.data)
    }
}

impl<I> Not for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn not(self) -> I2pInteger<I> {
        I2pInteger::new(!self.data)
    }
}

impl<'a, I> Not for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn not(self) -> I2pInteger<I> {
        I2pInteger::new(!self.data)
    }
}

impl<I> BitAnd<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitand(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data & other.data)
    }
}

impl<'a, I> BitAnd<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitand(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data & other.data)
    }
}

impl<'a, I> BitAnd<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitand(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data & other.data)
    }
}

impl<'a, 'b, I> BitAnd<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitand(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data & other.data)
    }
}

impl<I> BitOr<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitor(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data | other.data)
    }
}

impl<'a, I> BitOr<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitor(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data | other.data)
    }
}

impl<'a, I> BitOr<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitor(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data | other.data)
    }
}

impl<'a, 'b, I> BitOr<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitor(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data | other.data)
    }
}

impl<I> BitXor<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitxor(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data ^ other.data)
    }
}

impl<'a, I> BitXor<I2pInteger<I>> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitxor(self, other: I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data ^ other.data)
    }
}

impl<'a, I> BitXor<&'a I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitxor(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data ^ other.data)
    }
}

impl<'a, 'b, I> BitXor<&'a I2pInteger<I>> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    type Output = I2pInteger<I>;

    fn bitxor(self, other: &'a I2pInteger<I>) -> I2pInteger<I> {
        I2pInteger::new(self.data ^ other.data)
    }
}

macro_rules! shift_left_impl {
    ( $ T : ty ) => {
        impl<I> Shl<$T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shl(self, other: $T) -> I2pInteger<I> {
                I2pInteger::new(self.data << other)
            }
        }

        impl<'a, I> Shl<$T> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shl(self, other: $T) -> I2pInteger<I> {
                I2pInteger::new(self.data << other)
            }
        }

        impl<'a, I> Shl<&'a $T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shl(self, other: &'a $T) -> I2pInteger<I> {
                I2pInteger::new(self.data << other)
            }
        }

        impl<'a, 'b, I> Shl<&'a $T> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shl(self, other: &'a $T) -> I2pInteger<I> {
                I2pInteger::new(self.data << other)
            }
        }
    }
}

shift_left_impl!(u8);
shift_left_impl!(u16);
shift_left_impl!(u32);
shift_left_impl!(u64);
shift_left_impl!(usize);
shift_left_impl!(i8);
shift_left_impl!(i16);
shift_left_impl!(i32);
shift_left_impl!(i64);
shift_left_impl!(isize);


macro_rules! shift_right_impl {
    ( $ T : ty ) => {
        impl<I> Shr<$T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shr(self, other: $T) -> I2pInteger<I> {
                I2pInteger::new(self.data >> other)
            }
        }

        impl<'a, I> Shr<$T> for &'a I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shr(self, other: $T) -> I2pInteger<I> {
                I2pInteger::new(self.data >> other)
            }
        }

        impl<'a, I> Shr<&'a $T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shr(self, other: &'a $T) -> I2pInteger<I> {
                I2pInteger::new(self.data >> other)
            }
        }

        impl<'a, 'b, I> Shr<&'a $T> for &'b I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            type Output = I2pInteger<I>;

            fn shr(self, other: &'a $T) -> I2pInteger<I> {
                I2pInteger::new(self.data >> other)
            }
        }
    }
}

shift_right_impl!(u8);
shift_right_impl!(u16);
shift_right_impl!(u32);
shift_right_impl!(u64);
shift_right_impl!(usize);
shift_right_impl!(i8);
shift_right_impl!(i16);
shift_right_impl!(i32);
shift_right_impl!(i64);
shift_right_impl!(isize);


impl<I> AddAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn add_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() + other;
    }
}

impl<I> SubAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn sub_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() - other;
    }
}

impl<I> MulAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn mul_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() * other;
    }
}

impl<I> DivAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn div_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() % other;
    }
}

impl<I> RemAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn rem_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() % other;
    }
}

impl<I> BitAndAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn bitand_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() & other;
    }
}

impl<I> BitOrAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn bitor_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() | other;
    }
}

impl<I> BitXorAssign<I2pInteger<I>> for I2pInteger<I>  where I: I2pIntSize + I2pIntMask {
    fn bitxor_assign(&mut self, other: I2pInteger<I>) {
        *self = self.clone() ^ other;
    }
}


macro_rules! shl_assign_impl {
    ($ T : ty ) => {
        impl<I> ShlAssign<$T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            fn shl_assign(&mut self, other: $T) {
                *self = self.clone() << other;
            }
        }
    }
}

macro_rules! shr_assign_impl {
    ($ T : ty ) => {
        impl<I> ShrAssign<$T> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
            fn shr_assign(&mut self, other: $T) {
                *self = self.clone() >> other;
            }
        }
    }
}

macro_rules! shift_assign_impl {
    ( $ T : ty ) => {
        shl_assign_impl!($T);
        shr_assign_impl!($T);
    }
}

shift_assign_impl!(u8);
shift_assign_impl!(u16);
shift_assign_impl!(u32);
shift_assign_impl!(u64);
shift_assign_impl!(usize);
shift_assign_impl!(i8);
shift_assign_impl!(i16);
shift_assign_impl!(i32);
shift_assign_impl!(i64);
shift_assign_impl!(isize);


impl<I> PartialOrd<I2pInteger<I>> for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn partial_cmp(&self, other: &I2pInteger<I>) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<I> Ord for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn cmp(&self, other: &I2pInteger<I>) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<I> fmt::Binary for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.data;

        // Delegate to underlying u64's implmentation
        write!(f, "{:b}", val)
    }
}

impl<I> fmt::Octal for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.data;

        // Delegate to underlying u64's implmentation
        write!(f, "{:b}", val)
    }
}

impl<I> fmt::LowerHex for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.data;

        // Delegate to underlying u64's implmentation
        write!(f, "{:b}", val)
    }
}

impl<I> fmt::UpperHex for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.data;

        // Delegate to underlying u64's implmentation
        write!(f, "{:b}", val)
    }
}

impl<I> rand::Rand for I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    fn rand<R: rand::Rng>(rng: &mut R) -> I2pInteger<I> {
        I2pInteger::new(rng.next_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::{I2pInt64, I2pInt16, I2pInt24};


    struct TestCase {
        arg1: I2pInt64,
        arg2: I2pInt64,
        result: I2pInt64
    }

    struct Test {
        data: Vec<TestCase>
    }

    fn i2p_integer_addition_test_cases() -> Test {
        Test {
            data: vec![
                TestCase {
                    arg1:   I2pInt64::new(7476217),
                    arg2:   I2pInt64::new(225026),
                    result: I2pInt64::new(7701243)
                },
                TestCase {
                    arg1:   I2pInt64::new(8959170),
                    arg2:   I2pInt64::new(3654202),
                    result: I2pInt64::new(12613372)
                },
                TestCase {
                    arg1:   I2pInt64::new(4615797),
                    arg2:   I2pInt64::new(5679303),
                    result: I2pInt64::new(10295100)
                },
                TestCase {
                    arg1:   I2pInt64::new(7303216),
                    arg2:   I2pInt64::new(9510201),
                    result: I2pInt64::new(16813417)
                }
            ]
        }
    }

    fn i2p_integer_multiplication_test_cases() -> Test {
        Test {
            data: vec![
                TestCase {
                    arg1:   I2pInt64::new(7476217),
                    arg2:   I2pInt64::new(225026),
                    result: I2pInt64::new(1682343206642)
                },
                TestCase {
                    arg1:   I2pInt64::new(8959170),
                    arg2:   I2pInt64::new(3654202),
                    result: I2pInt64::new(32738616932340)
                },
                TestCase {
                    arg1:   I2pInt64::new(4615797),
                    arg2:   I2pInt64::new(5679303),
                    result: I2pInt64::new(26214509749491)
                },
                TestCase {
                    arg1:   I2pInt64::new(7303216),
                    arg2:   I2pInt64::new(9510201),
                    result: I2pInt64::new(69455052106416)
                }
            ]
        }
    }

    fn i2p_integer_subtraction_test_cases() -> Test {
        Test {
            data: vec![
                TestCase {
                    arg1:   I2pInt64::new(7476217),
                    arg2:   I2pInt64::new(225026),
                    result: I2pInt64::new(7251191)
                },
                TestCase {
                    arg1:   I2pInt64::new(8959170),
                    arg2:   I2pInt64::new(3654202),
                    result: I2pInt64::new(5304968)
                },
                TestCase {
                    arg1:   I2pInt64::new(5679303),
                    arg2:   I2pInt64::new(4615797),
                    result: I2pInt64::new(1063506)
                },
                TestCase {
                    arg1:   I2pInt64::new(9510201),
                    arg2:   I2pInt64::new(7303216),
                    result: I2pInt64::new(2206985)
                }
            ]
        }
    }

    fn i2p_integer_division_test_cases() -> Test {
        Test {
            data: vec![
                TestCase {
                    arg1:   I2pInt64::new(7476217),
                    arg2:   I2pInt64::new(225026),
                    result: I2pInt64::new(33)
                },
                TestCase {
                    arg1:   I2pInt64::new(8959170),
                    arg2:   I2pInt64::new(36542),
                    result: I2pInt64::new(245)
                },
                TestCase {
                    arg1:   I2pInt64::new(5679303),
                    arg2:   I2pInt64::new(4615),
                    result: I2pInt64::new(1230)
                },
                TestCase {
                    arg1:   I2pInt64::new(9510201),
                    arg2:   I2pInt64::new(73032),
                    result: I2pInt64::new(130)
                }
            ]
        }
    }

    fn i2p_integer_remainder_test_cases() -> Test {
        Test {
            data: vec![
                TestCase {
                    arg1:   I2pInt64::new(7476217),
                    arg2:   I2pInt64::new(225026),
                    result: I2pInt64::new(50359)
                },
                TestCase {
                    arg1:   I2pInt64::new(8959170),
                    arg2:   I2pInt64::new(36542),
                    result: I2pInt64::new(6380)
                },
                TestCase {
                    arg1:   I2pInt64::new(5679303),
                    arg2:   I2pInt64::new(4615),
                    result: I2pInt64::new(2853)
                },
                TestCase {
                    arg1:   I2pInt64::new(9510201),
                    arg2:   I2pInt64::new(73032),
                    result: I2pInt64::new(16041)
                }
            ]
        }
    }

    #[test]
    fn test_i2p_integer_addition() {
        let tests = i2p_integer_addition_test_cases();
        for test_case in tests.data {
            let result = test_case.arg1 + test_case.arg2;
            assert_eq!(result, test_case.result);
        }
    }

    #[test]
    fn test_i2p_integer_multiplication() {
        let tests = i2p_integer_multiplication_test_cases();
        for test_case in tests.data {
            let result = test_case.arg1 * test_case.arg2;
            assert_eq!(result, test_case.result);
        }
    }

    #[test]
    fn test_i2p_integer_subtraction() {
        let tests = i2p_integer_subtraction_test_cases();
        for test_case in tests.data {
            let result = test_case.arg1 - test_case.arg2;
            assert_eq!(result, test_case.result);
        }
    }

    #[test]
    fn test_i2p_integer_division() {
        let tests = i2p_integer_division_test_cases();
        for test_case in tests.data {
            let result = test_case.arg1 / test_case.arg2;
            assert_eq!(result, test_case.result);
        }
    }

    #[test]
    fn test_i2p_integer_remainder() {
        let tests = i2p_integer_remainder_test_cases();
        for test_case in tests.data {
            let result = test_case.arg1 % test_case.arg2;
            assert_eq!(result, test_case.result);
        }
    }

    #[test]
    fn test_from_bytes_be_should_return_zero_on_empty_slice() {
        let vec = vec![];

        let result: I2pInt64 = I2pInt64::from_bytes_be(vec.as_ref()).unwrap();

        assert_eq!(result, I2pInt64::from(0));
    }

    #[test]
    fn test_conversion_should_truncate_higher_order_bits() {
        let val: u64         = 0x1_FFFF_FFFF;
        let result: I2pInt16 = I2pInt16::from(val);
        let mask: u64        = 0xFFFF_FFFF;
        // The higher-order bits should be getting truncated to fit.
        let expected         = I2pInt16::from(val & mask);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_arithmetic_overflow() {
        let val: u64 = 0xFFFF_FFFF_FFFF + 1;
        let result: I2pInt24 = I2pInt24::from(val);
        let expected: I2pInt24 = I2pInt24::from(0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bytes_should_be_reversible() {
        let value = I2pInt64::new(0x0102030405060708);
        let value_be: [u8; 8] = [0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08];
        let bytes = value.to_bytes_be();
        assert_eq!(value_be.as_ref(), bytes.as_slice());
        // Convert a value to raw bytes and back and check is we get out old result back.
        let result = I2pInt64::from_bytes_be(bytes.as_slice()).unwrap();
        assert_eq!(value, result);
    }
}
