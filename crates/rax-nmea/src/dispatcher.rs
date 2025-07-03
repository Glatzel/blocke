use std::collections::HashMap;
use std::str::FromStr;

use rax_parser::io::IRaxReader;

use crate::data::{Identifier, Talker};

/// Dispatcher reads and groups sentences, handling both single and multi-line
/// messages.
pub struct Dispatcher<'a, R>
where
    R: IRaxReader,
{
    reader: &'a mut R,
    buffer: HashMap<(Talker, Identifier), String>, // (total count, accumulated sentence)
}

impl<'a, R> Dispatcher<'a, R>
where
    R: IRaxReader,
{
    /// Create a new dispatcher with the given reader.
    pub fn new(reader: &'a mut R) -> Self {
        Self {
            reader,
            buffer: HashMap::new(),
        }
    }

    /// Read and parse a line, returning its talker, identifier, and the
    /// sentence.
    fn preprocess(&mut self) -> Option<(Talker, Identifier, String)> {
        loop {
            match self.reader.read_line() {
                Ok(Some(sentence)) => {
                    let talker = match Talker::from_str(&sentence) {
                        Ok(t) => t,
                        Err(e) => {
                            clerk::warn!("{}", e);
                            continue;
                        }
                    };
                    let identifier = match Identifier::from_str(&sentence) {
                        Ok(i) => i,
                        Err(e) => {
                            clerk::warn!("{}", e);
                            continue;
                        }
                    };
                    return Some((talker, identifier, sentence));
                }
                Ok(None) => return None,
                Err(e) => {
                    clerk::warn!("{}", e);
                    continue;
                }
            }
        }
    }

    /// Handle multi-line sentences (e.g., GSV, TXT).
    fn process_multilines(
        &mut self,
        talker: Talker,
        identifier: Identifier,
        sentence: String,
    ) -> Option<(Talker, Identifier, String)> {
        let parts: Vec<&str> = sentence.split(',').collect();
        let count: Option<usize> = parts.get(1).and_then(|s| s.parse().ok());
        let idx: Option<usize> = parts.get(2).and_then(|s| s.parse().ok());
        let (count, idx) = match (count, idx) {
            (Some(c), Some(i)) => (c, i),
            _ => {
                clerk::warn!("Malformed sentence: {}", sentence);
                return None;
            }
        };

        match (
            idx == 1,
            count == idx,
            self.buffer.get(&(talker, identifier)),
        ) {
            (true, true, _) => Some((talker, identifier, sentence)),
            // First line of multi-line, buffer it
            (true, false, None) => {
                self.buffer.insert((talker, identifier), sentence);
                None
            }
            // Newer first line arrived, replace old buffer
            (true, false, Some(old)) => {
                clerk::warn!(
                    "A newer `{}{}` arrived, remove older one: {}",
                    talker,
                    identifier,
                    old
                );
                self.buffer.insert((talker, identifier), sentence);
                None
            }
            // Last line, combine with buffer and return
            (false, true, Some(v)) => {
                clerk::debug!("`{}{}` is complete.", talker, identifier);
                let combined = format!("{}{}", v, sentence);
                self.buffer.remove(&(talker, identifier));
                Some((talker, identifier, combined))
            }
            // Out-of-order line, skip
            (false, _, None) => {
                clerk::warn!(
                    "Former `{}{}` doesn't exist, will skip this sentence: {}",
                    talker,
                    identifier,
                    sentence
                );
                None
            }
            // Middle line, append to buffer
            (false, false, Some(_)) => {
                clerk::debug!(
                    "Append new sentence to `{}{}`: {}",
                    talker,
                    identifier,
                    sentence
                );
                if let Some(entry) = self.buffer.get_mut(&(talker, identifier)) {
                    entry.push_str(&sentence);
                }
                None
            }
        }
    }

    /// Dispatches sentences, handling both single and multi-line types.
    fn dispatch_by_lines(&mut self) -> Option<(Talker, Identifier, String)> {
        loop {
            if let Some((talker, identifier, sentence)) = self.preprocess() {
                match identifier {
                    // Single-line sentences
                    Identifier::DHV
                    | Identifier::GGA
                    | Identifier::GLL
                    | Identifier::GSA
                    | Identifier::GST
                    | Identifier::RMC
                    | Identifier::VTG
                    | Identifier::ZDA => return Some((talker, identifier, sentence)),

                    // Multi-line sentences
                    Identifier::GSV | Identifier::Txt => {
                        if let Some(result) = self.process_multilines(talker, identifier, sentence)
                        {
                            return Some(result);
                        }
                    }
                }
            } else {
                return None;
            }
        }
    }
}

impl<'a, R> Iterator for Dispatcher<'a, R>
where
    R: IRaxReader,
{
    type Item = (Talker, Identifier, String);

    fn next(&mut self) -> Option<Self::Item> { self.dispatch_by_lines() }
}
#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io;

    use clerk::init_log_with_level;
    use clerk::tracing::level_filters::LevelFilter;
    use miette::IntoDiagnostic;
    use rax_parser::io::RaxReader;

    use crate::Dispatcher;

    #[test]
    fn test_dispatcher() -> miette::Result<()> {
        init_log_with_level(LevelFilter::TRACE);
        for f in [
            "data/nmea1.log",
            "data/nmea2.log",
            "data/nmea_with_sat_info.log",
        ] {
            let mut reader = RaxReader::new(io::BufReader::new(File::open(f).into_diagnostic()?));
            let dispatcher = Dispatcher::new(&mut reader);
            for _ in dispatcher {}
        }

        Ok(())
    }
}
