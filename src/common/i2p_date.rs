use std::error;
use common::i2p_integer::I2pInt64;
use chrono::datetime::DateTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc;
use std::fmt;
use std::u64;
use rand;
use serialize;


const I2P_DATE_LENGTH_BYTES: usize = 8;

#[derive(Copy, Clone, Debug)]
pub enum I2pDateError {
    GotZeroMilliseconds
}

impl fmt::Display for I2pDateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            I2pDateError::GotZeroMilliseconds => {
                writeln!(f, "Error: Got a value of zero milliseconds for a date.")
            }
        }
    }
}

impl error::Error for I2pDateError {
    fn description(&self) -> &str {
        match *self {
            I2pDateError::GotZeroMilliseconds => {
                "The I2P specification forbids the passing of zero milliseconds for a date."
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// The `Date` type counts the number of milliseconds since January 1, 1970 (UNIX time)
/// in the GMT timezone. If the number is 0, the date is undefined or null.
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug)]
pub struct I2pDate {
    milliseconds: I2pInt64
}

impl I2pDate {
    /// Creates a new I2pDate.
    /// # Panics
    /// When milliseconds is zero.
    pub fn new(milliseconds: I2pInt64) -> I2pDate {
        if milliseconds > I2pInt64::from(0) {
            I2pDate {
                milliseconds: milliseconds
            }
        } else {
            panic!("Got a zero value for milliseconds.");
        }
    }

    /// Creates a new I2pDate without panicing when milliseconds is zero.
    pub fn checked_new(milliseconds: I2pInt64) -> Result<I2pDate, I2pDateError> {
        if milliseconds > I2pInt64::from(0) {
            let i2p_date = I2pDate {
                milliseconds: milliseconds
            };

            Ok(i2p_date)
        } else {
            Err(I2pDateError::GotZeroMilliseconds)
        }
    }

    pub fn len(&self) -> usize {
        I2P_DATE_LENGTH_BYTES
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.milliseconds.to_bytes_be()
    }

    pub fn to_rfc3339(&self) -> String {
        let datetime: DateTime<utc::UTC> = self.to_datetime();

        datetime.to_rfc3339()
    }

    pub fn to_datetime(&self) -> DateTime<utc::UTC> {
        let msec_to_sec  = I2pInt64::new(1000);
        let msec_to_nsec = I2pInt64::new(1_000_000);
        let seconds      = self.milliseconds / msec_to_sec;
        let remainder    = self.milliseconds % msec_to_sec;
        let nanoseconds  = remainder * msec_to_nsec;
        let naive_dt     = NaiveDateTime::from_timestamp(seconds.to_u64() as i64, nanoseconds.to_u64() as u32);

        DateTime::from_utc(naive_dt, utc::UTC)
    }

    pub fn min_value() -> I2pDate {
        I2pDate::new(I2pInt64::new(1))
    }

    pub fn max_value() -> I2pDate {
        I2pDate::new(I2pInt64::new(u64::max_value()))
    }
}

impl Default for I2pDate {
    fn default() -> I2pDate {
        I2pDate::min_value()
    }
}

impl fmt::Display for I2pDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

impl rand::Rand for I2pDate {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let random_date = I2pInt64::new(rng.next_u64());

        I2pDate::new(random_date + I2pInt64::new(1))
    }
}

impl serialize::Serialize for I2pDate {
    fn serialize(&self, buf: &mut [u8]) -> serialize::Result<usize> {
        if buf.len() >= I2P_DATE_LENGTH_BYTES {
            let bytes = self.to_bytes_be();
            let byte_slice: &[u8] = bytes.as_ref();
            assert_eq!(byte_slice.len(), 8);
            for i in 0..buf.len() {
                buf[i] = byte_slice[i];
            }
            Ok(byte_slice.len())
        } else {
            Err(serialize::Error::buffer_too_small(I2P_DATE_LENGTH_BYTES, buf.len()))
        }
    }
}

// According to the I2P spec, An I2pDate is just an 8 byte big endian
// (network byte order) Integer.
impl serialize::Deserialize for I2pDate {
    type Output = I2pDate;

    fn deserialize(buf: &[u8]) -> serialize::Result<I2pDate> {
        let i2p_integer = match <I2pInt64 as serialize::Deserialize>::deserialize(buf) {
            Ok(integer) => integer,
            Err(err) => return Err(serialize::Error::Decoding(Box::new(err)))
        };
        let i2p_date = match I2pDate::checked_new(i2p_integer) {
            Ok(date) => date,
            Err(err) => return Err(serialize::Error::Decoding(Box::new(err)))
        };

        Ok(i2p_date)
    }
}


#[cfg(test)]
mod tests {
    use super::I2pDate;
    use common::i2p_integer::I2pInt64;


    #[test]
    #[should_panic]
    fn test_constructor_should_panic_when_milliseconds_is_zero() {
        I2pDate::new(I2pInt64::new(0));

        // This test fails if no panic occurs.
        assert!(false);
    }
}
