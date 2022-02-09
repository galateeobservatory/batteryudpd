/// Validates the CRC16-Tarom4545 checksum of a line without whitespace or newline
pub fn validate_line(line: &str) -> Result<&str, &'static str> {
    let line_len = line.len();

    if line_len < 5 {
        return Err("crc16_tarom4545::validate_line: line too short");
    }
    if u16::from_str_radix(&line[line_len - 4..], 16)
        .map_err(|_| "crc16_tarom4545::validate_line: missing trailing checksum")?
        == crc16_tarom4545(&line[..line_len - 4])
    {
        return Ok(line);
    }
    return Err("crc16_tarom4545::validate_line: checksum mismatch");
}

fn crc16_tarom4545(text: &str) -> u16 {
    let data = text.as_bytes();
    let mut crc = 0x1D0F;

    if text.len() == 0 {
        return 0;
    }

    for dbyte in data {
        crc ^= (*dbyte as u16) << 8;

        for _ in 0..8 {
            let mix = crc & 0x8000;
            crc = ((crc << 1) & 0xffff) as u16;
            if mix != 0 {
                crc = crc ^ 0x1021;
            }
        }
    }
    crc & 0xffff
}

#[cfg(test)]
mod test {
    use crate::crc16_tarom4545::crc16_tarom4545;

    #[test]
    fn test_crc() {
        assert_eq!(0xf7bb, crc16_tarom4545("1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;"));
    }

    #[test]
    fn test_crc_empty() {
        assert_eq!(0, crc16_tarom4545(""));
    }

    #[test]
    fn test_line_validation_ok() {
        assert_eq!(Ok("1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F7BB"), crate::crc16_tarom4545::validate_line("1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F7BB"));
    }

    #[test]
    fn test_line_validation_err() {
        assert_eq!(true, crate::crc16_tarom4545::validate_line("1;2021/03/01;23:26;26.2;1.1b;#;98.u0;#;-0.5;0.0g;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F7BB").is_err());
    }

    #[test]
    fn test_line_validation_err_empty() {
        assert_eq!(true, crate::crc16_tarom4545::validate_line("").is_err());
    }

    #[test]
    fn test_line_validation_err_too_short() {
        assert_eq!(true, crate::crc16_tarom4545::validate_line("7BB").is_err());
    }
}
