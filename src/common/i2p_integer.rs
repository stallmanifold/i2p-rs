use std::marker::PhantomData;
use std::fmt;


pub const I2P_INTEGER_SIZE: usize = 8;

trait I2pIntSize {}

struct _1 {}
struct _2 {}
struct _3 {}
struct _4 {}
struct _5 {}
struct _6 {}
struct _7 {}
struct _8 {}

impl I2pIntSize for _1 {}
impl I2pIntSize for _2 {}
impl I2pIntSize for _3 {}
impl I2pIntSize for _4 {}
impl I2pIntSize for _5 {}
impl I2pIntSize for _6 {}
impl I2pIntSize for _7 {}
impl I2pIntSize for _8 {}


// I2P Integer
// Represents a variable sized integer from 1 to 8 bytes long.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct I2pInteger<I: I2pIntSize> {
    data: u64,
    __pd: PhantomData<I>,
}

impl<I> I2pInteger<I> where I: I2pIntSize + I2pIntMask {
    pub fn new(data: u64) -> I2pInteger<I> {
        let mask = <I as I2pIntMask>::mask();

        I2pInteger {
            data: data & mask,
            __pd: PhantomData,
        }
    }

    /// Converts a byte sequence into an I2pInteger, in network byte (big endian) order.
    /// If a byte array is of length zero, then `from_bytes_be` returns zero. 
    /// If a byte array is longer than the maximum number bytes for an I2pInteger, it return None.
    pub fn from_bytes_be(bytes: &[u8]) -> Option<I2pInteger<I>> {
        if bytes.len() >= I2P_INTEGER_SIZE {
            return None;
        }

        let mask = <I as I2pIntMask>::mask();
        let mut result: u64 = 0x00;
        
        for byte in bytes {
            result = (result | *byte as u64) << 8;
        }

        Some(I2pInteger::new(result & mask))
    }
}

trait I2pIntMask {
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


type I2pInt8  = I2pInteger<_1>;
type I2pInt16 = I2pInteger<_2>;
type I2pInt24 = I2pInteger<_3>;
type I2pInt32 = I2pInteger<_4>;
type I2pInt40 = I2pInteger<_5>;
type I2pInt48 = I2pInteger<_6>;
type I2pInt56 = I2pInteger<_7>;
type I2pInt64 = I2pInteger<_8>;

impl<I> fmt::Display for I2pInteger<I> where I: I2pIntSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
