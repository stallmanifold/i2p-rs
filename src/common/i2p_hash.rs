use std::fmt;


const I2P_SHA256_HASH_LENGTH: usize = 32;

#[derive(Eq, Copy)]
pub struct I2pHash {
    data: [u8; I2P_SHA256_HASH_LENGTH]
}

impl I2pHash {
    fn new(data: [u8; I2P_SHA256_HASH_LENGTH]) -> I2pHash {
        I2pHash {
            data: data
        }
    }
}

impl Default for I2pHash {
    fn default() -> I2pHash {
        I2pHash::new([0x00; I2P_SHA256_HASH_LENGTH])
    }
}

impl Clone for I2pHash {
    fn clone(&self) -> I2pHash {
        let mut cloned_hash = [0x00; I2P_SHA256_HASH_LENGTH];
        for i in 0..self.data.len() {
            cloned_hash[i] = self.data[i];
        }

        I2pHash::new(cloned_hash)
    }
}

impl PartialEq for I2pHash {
    fn eq(&self, other: &I2pHash) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}


#[cfg(test)]
mod tests {

}
