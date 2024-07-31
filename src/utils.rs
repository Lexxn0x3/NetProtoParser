pub fn calculate_checksum(header: &[u8]) -> u16 {
        let mut sum = 0u32;
        let mut i = 0;

        while i < header.len() {
            let word = if i + 1 < header.len() {
                ((header[i] as u32) << 8) + (header[i + 1] as u32)
            } else {
                (header[i] as u32) << 8
            };
            sum += word;
            if sum > 0xffff {
                sum = (sum & 0xffff) + (sum >> 16);
            }
            i += 2;
        }

        !sum as u16
    }
/// Converts a byte slice to a hexadecimal string without quotes.
pub fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| format!("{:02x}", b))
         .collect::<Vec<String>>()
         .join(" ")
}

/// Converts a byte slice to a dotted decimal IP address string without quotes.
pub fn to_ip_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| b.to_string())
         .collect::<Vec<String>>()
         .join(".")
}

/// Converts a byte slice to a MAC address string with colons between numbers.
pub fn to_mac_string(bytes: &[u8]) -> String {
    bytes.iter()
         .map(|b| format!("{:02x}", b))
         .collect::<Vec<String>>()
         .join(":")
}
