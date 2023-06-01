use std::net::Ipv4Addr;
use std::convert::TryInto;
use anyhow::Error;

pub fn create_ipv4_addr(ip_addr: Ipv4Addr) -> Result<u32, Error> {
    let octets = ip_addr.octets();
    let ip_addr: Ipv4Addr = Ipv4Addr::new(octets[0],octets[1], octets[2],octets[3]).try_into()?;
    let ip_value: u32 = u32::from(ip_addr);
    Ok(ip_value)
}
pub fn create_ipv4_addr_param(ip_addr: Ipv4Addr) -> Result<u32, Error> {
        let ip_value: u32 = u32::from(ip_addr);
        Ok(ip_value)
    }
/*
fn main() {
    let ip_addr = create_ipv4_addr().unwrap();

    println!("Created Ipv4Addr: {}", ip_addr);

    // Use ip_addr variable here
    // ...
}*/
