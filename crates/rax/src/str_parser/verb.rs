impl<'a> super::IStrParserContext<'a> {
    pub fn take<R, O>(&mut self, rule: R) -> Option<O>
    where
        R: super::rules::IRule<'a, O>,
    {
        match rule.apply_rule(self.rest) {
            Some(result) => {
                self.rest = result.1;
                Some(result.0)
            }
            None => None,
        }
    }
    pub fn take_strict<R, O>(&mut self, rule: R) -> miette::Result<O>
    where
        R: super::rules::IRule<'a, O>,
    {
        match self.take(rule) {
            Some(s) => Ok(s),
            None => miette::bail!("input string is shorter than requested count"),
        }
    }
}

impl<'a> super::IStrParserContext<'a> {
    pub fn skip<R, O>(&mut self, rule: R) -> &mut Self
    where
        R: super::rules::IRule<'a, O>,
    {
        self.take(rule);
        self
    }
    pub fn skip_strict<R, O>(&mut self, rule: R) -> miette::Result<&mut Self>
    where
        R: super::rules::IRule<'a, O>,
    {
        self.take_strict(rule)?;
        Ok(self)
    }
}
