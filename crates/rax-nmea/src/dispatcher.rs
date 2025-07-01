use std::collections::HashMap;
use std::str::FromStr;

use rax_parser::io::IRaxReader;

use crate::data::{Identifier, Talker};

/// Dispatcher reads and groups NMEA sentences, handling both single and
/// multi-line messages.
pub struct Dispatcher<'a, R>
where
    R: IRaxReader,
{
    reader: &'a mut R,
    buffer: HashMap<(Talker, Identifier), (usize, usize, String)>,
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
    fn preprocess(&mut self) -> (Talker, Identifier, String) {
        loop {
            let sentence = match self.reader.read_line() {
                Ok(Some(s)) => s,
                Ok(None) => {
                    clerk::warn!("Sentence is none.");
                    continue;
                }
                Err(e) => {
                    clerk::warn!("{}", e);
                    continue;
                }
            };

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

            return (talker, identifier, sentence);
        }
    }

    /// Handle multi-line sentences (currently only GSV).
    fn process_multilines(
        &mut self,
        talker: Talker,
        identifier: Identifier,
        sentence: String,
    ) -> Option<(Talker, Identifier, String)> {
        // Only GSV is supported as multiline
        let (count, idx) = if let Identifier::GSV = identifier {
            let parts: Vec<&str> = sentence.split(',').collect();
            let c = parts.get(1).and_then(|s| s.parse().ok());
            let i = parts.get(2).and_then(|s| s.parse().ok());
            match (c, i) {
                (Some(c), Some(i)) => (c, i),
                _ => {
                    clerk::warn!("Malformed GSV sentence: {}", sentence);
                    return None;
                }
            }
        } else {
            panic!("Identifier `{:?}` is not a multiline nmea.", identifier);
        };

        match (
            idx == 1,
            count == idx,
            self.buffer.get(&(talker, identifier)),
        ) {
            // Only one line, return immediately
            (true, true, _) => Some((talker, identifier, sentence)),
            // First line of multi-line, buffer it
            (true, false, None) => {
                self.buffer
                    .insert((talker, identifier), (1, count, sentence));
                None
            }
            // Newer first line arrived, replace old buffer
            (true, false, Some(s)) => {
                clerk::warn!(
                    "A newer `{}{}` arrived, remove older one: {}",
                    talker,
                    identifier,
                    s.2
                );
                self.buffer.remove(&(talker, identifier));
                self.buffer
                    .insert((talker, identifier), (1, count, sentence));
                None
            }
            // Last line, combine with buffer and return
            (false, true, Some(v)) => {
                clerk::debug!("`{}{}` is complete.", talker, identifier);
                Some((talker, identifier, format!("{}{}", v.2, sentence)))
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
            (false, false, Some(v)) => {
                clerk::debug!(
                    "Append new sentence to `{}{}`: {}",
                    talker,
                    identifier,
                    sentence
                );
                if let Some(val) = self.buffer.get_mut(&(talker, identifier)) {
                    // Increase the index and append the sentence
                    *val = (val.0, val.1 + 1, format!("{}{}", val.2, sentence));
                }
                None
            }
        }
    }

    /// Dispatches sentences, handling both single and multi-line types.
    fn dispatch_by_lines(&mut self) -> (Talker, Identifier, String) {
        loop {
            let (talker, identifier, sentence) = self.preprocess();
            match identifier {
                // Single-line sentences
                Identifier::DHV
                | Identifier::GGA
                | Identifier::GLL
                | Identifier::GSA
                | Identifier::GST
                | Identifier::RMC
                | Identifier::VTG
                | Identifier::ZDA => return (talker, identifier, sentence),

                // Multi-line sentences
                Identifier::GSV => {
                    if let Some(result) = self.process_multilines(talker, identifier, sentence) {
                        return result;
                    }
                }
            }
        }
    }
}

impl<'a, R> Iterator for Dispatcher<'a, R>
where
    R: IRaxReader,
{
    type Item = (Talker, Identifier, String);

    fn next(&mut self) -> Option<Self::Item> { Some(self.dispatch_by_lines()) }
}
