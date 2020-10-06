#[cfg(test)]
mod tests {
    extern crate ipfix;

    use std::io::prelude::*;
    use std::fs::File;
    use self::ipfix::{IpfixConsumer, IpfixPrinter};

    #[test]
    fn test_parse() {
        // contains templates 500, 999, 501
        let template_bytes: [u8; 292] =
            [0x00, 0x0A, 0x01, 0x24, 0x58, 0x34, 0x94, 0xCA, 0x08, 0xF3, 0x62, 0x93, 0x00, 0x00,
             0x00, 0x00, 0x00, 0x02, 0x01, 0x14, 0x01, 0xF4, 0x00, 0x1B, 0x00, 0x01, 0x00, 0x08,
             0x00, 0x02, 0x00, 0x08, 0x00, 0x04, 0x00, 0x01, 0x00, 0x05, 0x00, 0x01, 0x00, 0x06,
             0x00, 0x02, 0x00, 0x07, 0x00, 0x02, 0x00, 0x08, 0x00, 0x04, 0x00, 0x09, 0x00, 0x01,
             0x00, 0x0A, 0x00, 0x04, 0x00, 0x0B, 0x00, 0x02, 0x00, 0x0C, 0x00, 0x04, 0x00, 0x0D,
             0x00, 0x01, 0x00, 0x0E, 0x00, 0x04, 0x00, 0x0F, 0x00, 0x04, 0x00, 0x10, 0x00, 0x04,
             0x00, 0x11, 0x00, 0x04, 0x00, 0x20, 0x00, 0x02, 0x00, 0x34, 0x00, 0x01, 0x00, 0x35,
             0x00, 0x01, 0x00, 0x3A, 0x00, 0x02, 0x00, 0x3D, 0x00, 0x01, 0x00, 0x46, 0x00, 0x03,
             0x00, 0x88, 0x00, 0x01, 0x00, 0x98, 0x00, 0x08, 0x00, 0x99, 0x00, 0x08, 0x00, 0xF3,
             0x00, 0x02, 0x00, 0xF5, 0x00, 0x02, 0x03, 0xE7, 0x00, 0x0B, 0x00, 0x01, 0x00, 0x08,
             0x00, 0x02, 0x00, 0x08, 0x00, 0x04, 0x00, 0x01, 0x00, 0x07, 0x00, 0x02, 0x00, 0x08,
             0x00, 0x04, 0x00, 0x0B, 0x00, 0x02, 0x00, 0x0C, 0x00, 0x04, 0x00, 0x20, 0x00, 0x02,
             0x00, 0x3A, 0x00, 0x02, 0x00, 0x98, 0x00, 0x08, 0x00, 0x99, 0x00, 0x08, 0x01, 0xF5,
             0x00, 0x1B, 0x00, 0x01, 0x00, 0x08, 0x00, 0x02, 0x00, 0x08, 0x00, 0x04, 0x00, 0x01,
             0x00, 0x05, 0x00, 0x01, 0x00, 0x06, 0x00, 0x02, 0x00, 0x07, 0x00, 0x02, 0x00, 0x0A,
             0x00, 0x04, 0x00, 0x0B, 0x00, 0x02, 0x00, 0x0E, 0x00, 0x04, 0x00, 0x10, 0x00, 0x04,
             0x00, 0x11, 0x00, 0x04, 0x00, 0x1B, 0x00, 0x10, 0x00, 0x1C, 0x00, 0x10, 0x00, 0x1D,
             0x00, 0x01, 0x00, 0x1E, 0x00, 0x01, 0x00, 0x34, 0x00, 0x01, 0x00, 0x35, 0x00, 0x01,
             0x00, 0x3A, 0x00, 0x02, 0x00, 0x3D, 0x00, 0x01, 0x00, 0x3E, 0x00, 0x10, 0x00, 0x46,
             0x00, 0x03, 0x00, 0x88, 0x00, 0x01, 0x00, 0x8B, 0x00, 0x02, 0x00, 0x98, 0x00, 0x08,
             0x00, 0x99, 0x00, 0x08, 0x00, 0xF3, 0x00, 0x02, 0x00, 0xF5, 0x00, 0x02];
        let template = Box::new(template_bytes);

        // contains data sets for templates 999, 500, 999
        let data_bytes: [u8; 1093] =
            [0x00, 0x0A, 0x04, 0x45, 0x58, 0x34, 0x94, 0xCA, 0x08, 0xF3, 0x66, 0x48, 0x00, 0x00,
             0x00, 0x00, 0x03, 0xE7, 0x02, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x11,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x11, 0xFC, 0x16, 0xAC, 0x13, 0xDB,
             0x32, 0x00, 0x35, 0xA5, 0x82, 0x01, 0x09, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x2A, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x0E, 0x06, 0x13, 0xC5, 0xA5, 0x82, 0x48, 0x9A, 0xE6, 0x8E, 0xAC, 0x13, 0xC9, 0xA4,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x64, 0xF9, 0x39, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x90,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x06, 0x1B, 0x58, 0x97, 0x8C, 0x56,
             0xF5, 0x93, 0x27, 0x97, 0x8C, 0x05, 0x4D, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x64, 0x7E, 0xBF, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xAC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x04, 0x06, 0x84, 0x79, 0x97, 0x8C, 0x65, 0x89, 0x27, 0x0D, 0x0A, 0x42, 0x22, 0x18,
             0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x36, 0x0D, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x36, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0xEC, 0xF7, 0xAC, 0x10, 0x91,
             0x2C, 0x01, 0xBB, 0xA8, 0x3D, 0x95, 0x11, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x65, 0x36, 0x86, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x36, 0x86, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, 0x11, 0x00, 0x35, 0x97, 0x8C, 0x01, 0x8F, 0xDA, 0x28, 0xAC, 0x1D, 0xEC, 0x52,
             0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x55,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x06, 0x00, 0x50, 0x17, 0x49, 0x02,
             0xDF, 0xB7, 0xEA, 0xCF, 0x0B, 0x01, 0xA2, 0x00, 0x00, 0x02, 0x58, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x65, 0x0B, 0x46, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, 0x06, 0x00, 0x50, 0xCF, 0x0B, 0x1F, 0x7A, 0xA5, 0xF5, 0x68, 0x81, 0xC2, 0x37,
             0x00, 0x00, 0x02, 0x58, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0C,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x06, 0xC0, 0x39, 0x97, 0x8C, 0x01,
             0x80, 0xD6, 0x84, 0xAC, 0x15, 0x8D, 0xA3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x65, 0x0D, 0xD2, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x02, 0x11, 0xC7, 0x6F, 0xAC, 0x1D, 0xED, 0x52, 0x00, 0x35, 0x97, 0x8C, 0x01, 0x8F,
             0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0xE5,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x06, 0x01, 0xBB, 0xC0, 0x7F, 0xE0,
             0x10, 0xC1, 0x33, 0x0A, 0xC1, 0xD6, 0xBB, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x65, 0x33, 0x14, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x36, 0x86, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x03, 0x06, 0x01, 0xBB, 0xD0, 0x59, 0x0C, 0x9D, 0x5F, 0xC2, 0xCF, 0x0B, 0x01, 0xA4,
             0x00, 0x00, 0x02, 0x58, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x36, 0x74, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x36, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0xF7, 0x81, 0x0A, 0x85, 0xF1,
             0x65, 0x01, 0xBD, 0x0A, 0x4A, 0x16, 0x44, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01,
             0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x01,
             0xF4, 0x00, 0x59, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xA8, 0x12,
             0x0C, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x0A, 0x9D, 0xE8, 0x1E, 0x00, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x08, 0x00, 0x3F, 0x3F, 0x02, 0x5C, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
             0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77,
             0x00, 0x00, 0x02, 0x5C, 0x03, 0xE7, 0x01, 0x5B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x06, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x06, 0xB3, 0x88, 0xCF,
             0x0B, 0x01, 0xA3, 0x01, 0xBB, 0x0D, 0x5C, 0x1A, 0x3E, 0x03, 0x03, 0x02, 0x7C, 0x00,
             0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x42, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F,
             0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x00, 0x04, 0x11, 0xEB, 0x47, 0x97, 0x8C, 0x80, 0x7A, 0x00, 0x35, 0xA5, 0x82,
             0x01, 0x09, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78,
             0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x6D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x06, 0x0D, 0x3D, 0xA5,
             0x82, 0xDD, 0x0A, 0xFA, 0x50, 0x97, 0x8C, 0x72, 0x8B, 0x0B, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F,
             0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x00, 0x08, 0x06, 0x01, 0xBB, 0xA5, 0x82, 0xE6, 0xE6, 0xE1, 0x93, 0xAA, 0x08,
             0xAA, 0x53, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78,
             0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x79, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x8E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x11, 0x41, 0x71, 0xAC,
             0x1D, 0xED, 0x52, 0x00, 0x35, 0x97, 0x8C, 0x01, 0x8F, 0x08, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F,
             0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xF7, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x00, 0x0B, 0x06, 0xCC, 0x12, 0xAC, 0x13, 0xBE, 0x95, 0x01, 0xBB, 0xC7, 0x5B,
             0x8B, 0xC8, 0x00, 0x00, 0x02, 0x58, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0E, 0x7D,
             0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x11, 0xB8, 0x78, 0x97,
             0x8C, 0x42, 0xA3, 0x00, 0x35, 0xAC, 0x18, 0x8F, 0x2A, 0x08, 0x00, 0x00, 0x00, 0x00,
             0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F, 0x78, 0x00, 0x00, 0x01, 0x58, 0x8D, 0x65, 0x0F,
             0x78];
        let data = Box::new(data_bytes);

        let mut parser = IpfixConsumer::new();
        
        let printer = IpfixPrinter::new();

        assert!(parser.parse_message(&*template).is_ok());
        
        if let Ok(datarecords) = parser.parse_message(&*data) {
            let mut test_string = String::new();
            for datarecord in datarecords {
                let flows = printer.print_json(datarecord);
                for flow in flows {
                    test_string += &flow;
                }
            }

            let mut f = File::open("tests/string.txt").unwrap();
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            assert_eq!(s, test_string);
        }
    }
}
