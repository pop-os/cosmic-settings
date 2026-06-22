#[derive(Clone, PartialEq, Eq, Default, Debug, PartialOrd, Ord)]
pub struct HwAddress {
    pub octets: Vec<u8>,
}

impl HwAddress {
    pub fn from_str(arg: &str) -> Option<Self> {
        let segments: Vec<&str> = arg.split(":").collect();

        // Only accept 6-byte (MAC-48) or 8-byte (EUI-64) addresses
        if segments.len() != 6 && segments.len() != 8 {
            return None;
        }

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
        let hex_parts: Vec<String> = self
            .octets
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

    #[test]
    fn test_display_6_byte_mac() {
        let hw_addr: HwAddress = HwAddress {
            octets: vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
        };

        assert_eq!(format!("{}", hw_addr), "00:11:22:33:44:55");
    }

    #[test]
    fn test_parse_valid_8_byte_mac() {
        let mac: &str = "00:11:22:33:44:55:66:77";
        let hw_addr: HwAddress = HwAddress::from_str(mac).expect("should parse valid EUI-64 MAC");

        assert_eq!(hw_addr.octets.len(), 8);
        assert_eq!(
            hw_addr.octets,
            vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]
        );
    }

    #[test]
    fn test_display_8_byte_mac() {
        let hw_addr: HwAddress = HwAddress {
            octets: vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77],
        };

        assert_eq!(format!("{}", hw_addr), "00:11:22:33:44:55:66:77");
    }

    #[test]
    fn test_reject_invalid_length_macs() {
        let invalid_macs: Vec<(&str, &str)> = vec![
            ("00", "1-byte MAC"),
            ("00:11:22:33", "4-byte MAC"),
            ("00:11:22:33:44", "5-byte MAC"),
            ("00:11:22:33:44:55:66", "7-byte MAC"),
            ("00:11:22:33:44:55:66:77:88", "9-byte MAC"),
            ("00:11:22:33:44:55:66:77:88:99:aa:bb", "12-byte MAC"),
        ];

        for (mac, description) in invalid_macs {
            assert!(
                HwAddress::from_str(mac).is_none(),
                "should reject {}",
                description
            );
        }
    }
}
