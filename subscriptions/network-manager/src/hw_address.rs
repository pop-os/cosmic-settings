#[derive(Clone, PartialEq, Eq, Default, Debug, PartialOrd, Ord)]
pub struct HwAddress {
    pub octets: Vec<u8>,
}

impl HwAddress {
    pub fn from_str(arg: &str) -> Option<Self> {
        let segments: Vec<&str> = arg.split(":").collect();
        
        let mut octets: Vec<u8> = Vec::new();
        for segment in segments {
            if segment.len() != 2 {
                return None;
            }
            let byte: u8 = u8::from_str_radix(segment, 16).ok()?;
            octets.push(byte);
        }
        
        Some(HwAddress { octets })
    }
    
    pub fn from_string(arg: &str) -> Option<Self> {
        HwAddress::from_str(arg)
    }
}

impl std::fmt::Display for HwAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex_parts: Vec<String> = self.octets
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
        write!(f, "{}", hex_parts.join(":"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_6_byte_mac() {
        let mac: &str = "00:11:22:33:44:55";
        let hw_addr: HwAddress = HwAddress::from_str(mac).expect("should parse valid MAC");
        
        // Access the internal octets field
        assert_eq!(hw_addr.octets.len(), 6);
        assert_eq!(hw_addr.octets, vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }
}
