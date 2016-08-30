use std::fmt;
use std::mem;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ByteOrder {
    BigEndian,
    LittleEndian,
}

impl fmt::Display for ByteOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ByteOrder::BigEndian    => write!(f, "Big Endian"),
            ByteOrder::LittleEndian => write!(f, "Little Endian"),
        }
    }
}

#[allow(unused_assignments)]
pub fn byte_order() -> ByteOrder {
    let mut arr: [u8; 2] = [0x00; 2];
    arr = unsafe {
            mem::transmute(0xBEEF as u16)
        };

    if arr[0] == 0xEF {
        return ByteOrder::LittleEndian;
    }

    ByteOrder::BigEndian

}

pub fn is_big_endian() -> bool {
    match byte_order() {
        ByteOrder::BigEndian    => true,
        ByteOrder::LittleEndian => false,
    }
}

pub fn is_little_endian() -> bool {
    match byte_order() {
        ByteOrder::BigEndian    => false,
        ByteOrder::LittleEndian => true,
    }
}


#[cfg(test)]
mod tests {
    use super::{ByteOrder, byte_order};
    use std::mem;


    #[test]
    #[allow(unused_assignments)]
    fn test_byte_order_correctly_calculated() {
        let mut arr: [u8; 2] = [0 as u8, 0 as u8];
        arr = unsafe { mem::transmute(0xABCD as u16) };

        if arr[0] == 0xAB as u8 {
            // The system is big endian.
            assert_eq!(ByteOrder::BigEndian, byte_order());
        } else if arr[0] == 0xCD as u8 {
            // The system is big endian.
            assert_eq!(ByteOrder::LittleEndian, byte_order());
        } else {
            // Something went wrong. Automatically Fail.
            assert!(false, "Something went wrong with byte_order.");
        }
    }

    #[test]
    #[should_panic]
    fn test_byte_order_differs_from_writing_array_bytes() {
        let arr: [u8; 2] = [0xAA as u8, 0xBB as u8];

        if arr[0] == 0xBB as u8 {
            assert_eq!(ByteOrder::LittleEndian, byte_order());
        } else if arr[0] == 0xAA as u8 {
            assert_eq!(ByteOrder::BigEndian, byte_order());
        } else {
            // Something went wrong. Automatically Fail.
            assert!(true, "Something went wrong with byte_order.");
        }
    }
}
