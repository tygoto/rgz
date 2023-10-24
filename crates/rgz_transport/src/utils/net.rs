use local_ip_address::list_afinet_netifas;
use std::env;
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};
use std::str::FromStr;

/// Get the preferred local IP address.
fn preferred_public_ip() -> Result<Ipv4Addr, String> {
    let host_name = hostname();
    // We don't want "localhost" to be our hostname.
    if host_name == "localhost" {
        return Err("localhost is not a valid hostname".to_string());
    }

    if let Ok(host_ip) = hostname_to_ip(&host_name) {
        if host_ip.is_private() || host_ip.is_loopback() {
            return Err(format!(
                "{} is not a valid target: it is private or loopback.",
                host_ip
            ));
        }
        // Get the complete list of compatible interfaces.
        if let Ok(interfaces) = determine_interfaces() {
            // Make sure that this interface is compatible with Discovery.
            return if interfaces.iter().any(|socket_addr| socket_addr == &host_ip) {
                Ok(host_ip)
            } else {
                Err(format!("{} is not a compatible interface", host_ip))
            };
        }
    }
    Err(format!("No public IP address found for {}", host_name))
}

/// Convert a hostname to an IP address.
fn hostname_to_ip(hostname: &str) -> Result<Ipv4Addr, String> {
    let addr_iter = (hostname, 0)
        .to_socket_addrs()
        .map_err(|e| format!("Unable to resolve hostname: {}", e))?;

    for addr in addr_iter {
        if let IpAddr::V4(ipv4) = addr.ip() {
            return Ok(ipv4);
        }
    }
    return Err(format!("No IPv4 address found for {}", hostname));
}

pub fn determine_host() -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    // First, did the user set GZ_IP?
    if let Ok(gz_ip) = env::var("GZ_IP") {
        if !gz_ip.is_empty() {
            let ip = Ipv4Addr::from_str(&gz_ip)?;
            return Ok(ip);
        }
    }
    // Second, try the preferred local and public IP address.
    if let Ok(public_ip) = preferred_public_ip() {
        return Ok(public_ip);
    }

    // Third, fall back on interface search, which will yield an IP address
    let interfaces = determine_interfaces()?;
    for iface in &interfaces {
        if !iface.is_private() {
            return Ok(*iface);
        }
    }

    interfaces
        .first()
        .map(|iface| *iface)
        .ok_or_else(|| "No interfaces found".into())
}

pub fn determine_interfaces() -> Result<Vec<Ipv4Addr>, Box<dyn std::error::Error>> {
    let mut host_interfaces = Vec::<Ipv4Addr>::new();
    if let Ok(itr) = list_afinet_netifas() {
        for (_name, ipaddr) in itr {
            if let IpAddr::V4(ipv4addr) = ipaddr {
                // Prefer non-loopback IPs
                if !ipv4addr.is_loopback() {
                    host_interfaces.push(ipv4addr);
                }
            }
        }
    }
    Ok(host_interfaces)
}

pub fn hostname() -> String {
    whoami::hostname()
}
pub fn username() -> String {
    whoami::username()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostname_to_ip() {
        let hostname = hostname();
        let ip_addresses = hostname_to_ip(&hostname).unwrap();

        // // env::set_var("GZ_IP", "0.0.0.0");
        // let host = determine_host();
        // assert!(!host.is_empty());
    }

    // #[test]
    // fn test_determine_host() {
    //     // env::set_var("GZ_IP", "0.0.0.0");
    //     let host = determine_host().unwrap();
    //     assert!(!host.is_unspecified());
    // }

    #[test]
    fn test_hostname() {
        let hostname = hostname();
        assert!(!hostname.is_empty());
    }
    #[test]
    fn test_username() {
        let username = username();
        assert!(!username.is_empty());
    }
}
