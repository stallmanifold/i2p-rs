use common::I2pInt32;


/// A `TunnelId` defines an identifier that is unique to each router in a tunnel.
/// A Tunnel ID is generally greater than zero; do not use a value of zero except
/// in special cases. The most likely special case is one in which one router requests
/// a direct reply from another router.
type TunnelId = I2pInt32;


#[cfg(test)]
mod tests {

}
