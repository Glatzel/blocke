use rax::str_parser::IRule;

/// Rule to validate an NMEA sentence for correct start character and checksum.
/// Returns Ok(()) if the sentence is valid, otherwise returns a miette error.
pub struct NmeaValidate();

impl IRule for NmeaValidate {
    fn name(&self) -> &str { "NmeaValidate" }
}

impl<'a> rax::str_parser::IStrGlobalRule<'a> for NmeaValidate {
    type Output = miette::Result<()>;
    /// Applies the NmeaValidate rule to the input string.
    /// Checks that the sentence starts with '$', contains a checksum delimiter
    /// '*', and that the calculated checksum matches the provided checksum.
    /// Logs each step for debugging.
    fn apply(&self, input: &'a str) -> miette::Result<()> {
        // Log the input at trace level.
        clerk::trace!("NmeaValidate rule: input='{}'", input);
        let input = input.trim_end();

        // Check if the sentence starts with '$'.
        if !input.starts_with('$') {
            clerk::warn!("NmeaValidate: sentence does not start with '$'");
            miette::bail!("sentence doesn't start with `$`");
        }

        // Find the position of the '*' checksum delimiter.
        let Some(star_pos) = input.find('*') else {
            clerk::warn!("NmeaValidate: missing checksum delimiter '*'");
            miette::bail!("Missing checksum delimiter `*`");
        };

        // Split the input into data and checksum string.
        let (data, checksum_str) = input[1..].split_at(star_pos - 1); // skip $
        let checksum_str = &checksum_str[1..];
        clerk::debug!(
            "NmeaValidate: data='{}', checksum_str='{}'",
            data,
            checksum_str
        );

        // Check that the checksum string is exactly 2 characters.
        if checksum_str.len() != 2 {
            clerk::warn!(
                "NmeaValidate: checksum_str length is {}, expected 2",
                checksum_str.len()
            );
            miette::bail!("require checksum_str length 2, get {}", checksum_str.len());
        }

        // Parse the expected checksum from hex.
        let expected = u8::from_str_radix(checksum_str, 16);
        let Ok(expected) = expected else {
            clerk::warn!("NmeaValidate: invalid hex checksum '{}'", checksum_str);
            miette::bail!("Invalid hex checksum");
        };

        // Calculate the checksum by XOR'ing all data bytes.
        let calculated = data.bytes().fold(0u8, |acc, b| acc ^ b);
        clerk::debug!(
            "NmeaValidate: calculated checksum={:02X}, expected={:02X}",
            calculated,
            expected
        );

        // Compare calculated and expected checksums.
        if calculated != expected {
            clerk::warn!(
                "NmeaValidate: checksum mismatch: calculated {:02X}, expected {:02X}",
                calculated,
                expected
            );
            miette::bail!(
                "Checksum mismatch: calculated {:02X}, expected {:02X}",
                calculated,
                expected
            );
        }
        clerk::info!("NmeaValidate: sentence is valid: {input}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rax::str_parser::IStrGlobalRule;

    use super::*;

    #[test]
    fn test_valid_sentence() {
        let rule = NmeaValidate();
        // Example: $GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        assert!(rule.apply(input).is_ok());
    }

    #[test]
    fn test_invalid_checksum() {
        let rule = NmeaValidate();
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Checksum mismatch"));
    }

    #[test]
    fn test_missing_dollar() {
        let rule = NmeaValidate();
        let input = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("doesn't start with"));
    }

    #[test]
    fn test_missing_star() {
        let rule = NmeaValidate();
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,47";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Missing checksum delimiter"));
    }

    #[test]
    fn test_invalid_hex_checksum() {
        let rule = NmeaValidate();
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*ZZ";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("Invalid hex checksum"));
    }

    #[test]
    fn test_short_checksum() {
        let rule = NmeaValidate();
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*4";
        let result = rule.apply(input);
        assert!(result.is_err());
        let msg = format!("{result:?}");
        assert!(msg.contains("require checksum_str length 2"));
    }
}
