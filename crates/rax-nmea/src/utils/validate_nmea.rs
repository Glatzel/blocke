pub struct ValidateNmea();
impl<'a> rax::str_parser::IStrGlobalRules<'a, miette::Result<()>> for ValidateNmea {
    fn name(&self) -> &str { todo!() }

    fn apply(&self, input: &'a str) -> miette::Result<()> {
        if !input.starts_with('$') {
            miette::bail!("sentence doesn't start with `$`");
        }

        let Some(star_pos) = input.find('*') else {
            miette::bail!("Missing checksum delimiter `*`");
        };

        let (data, checksum_str) = input[1..].split_at(star_pos - 1); // skip $
        let checksum_str = &checksum_str[1..];
        clerk::debug!("data: `{}`,checksum_str: `{}`", data, checksum_str);

        if checksum_str.len() != 2 {
            miette::bail!("require checksum_str length 2, get {}", checksum_str.len());
        }

        let expected = u8::from_str_radix(checksum_str, 16);
        let Ok(expected) = expected else {
            miette::bail!("Invalid hex checksum");
        };

        let calculated = data.bytes().fold(0u8, |acc, b| acc ^ b);

        if calculated != expected {
            miette::bail!(
                "Checksum mismatch: calculated {:02X}, expected {:02X}",
                calculated,
                expected
            );
        }
        Ok(())
    }
}
