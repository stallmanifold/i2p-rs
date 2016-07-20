use i2p_integer::I2pInt64;


/// The `Date` type counts the number of milliseconds since January 1, 1970 (UNIX time).
/// If the number is 0, the date is undefined or null. 
///
pub struct Date {
    milliseconds: I2pInt64, 
}

impl Date {
    fn new(milliseconds: I2pInt64) -> Date {
        Date {
            milliseconds: milliseconds,
        }
    }
}