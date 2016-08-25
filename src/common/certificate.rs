use common::{I2pInt8, I2pInt16};


pub enum CertificateType {
    NULL     = 0,
    HASHCASH = 1,
    HIDDEN   = 2,
    SIGNED   = 3,
    MULTIPLE = 4,
    KEY      = 5
}

pub struct Certificate {
    certificate_type: CertificateType,
    length: I2pInt16,
    payload: Vec<u8>
}

impl Certificate {
    fn new(certificate_type, length: I2pInt16, payload: &[u8]) -> Certificate {
        Certificate {
            certificate_type: certificate_type,
            length: length,
            payload: payload
        }
    }
}


#[cfg(test)]
mod tests {

}
