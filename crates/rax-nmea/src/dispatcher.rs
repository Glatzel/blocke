use std::collections::HashMap;
use std::str::FromStr;

use rax_parser::io::IRaxReader;

use crate::data::{Identifier, Talker};

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
    pub fn new(reader: &'a mut R) -> Self {
        Self {
            reader,
            buffer: HashMap::new(),
        }
    }
    fn preprocess(&mut self) -> (Talker, Identifier, std::string::String) {
        let mut talker: Talker;
        let identifier: Identifier;
        let mut sentence: String;
        loop {
            match self.reader.read_line() {
                Ok(s) => match s {
                    Some(s) => {
                        sentence = s;
                    }
                    None => {
                        clerk::warn!("Sentense is none.");
                        continue;
                    }
                },
                Err(e) => {
                    clerk::warn!("{}", e);
                    continue;
                }
            }
            match Talker::from_str(&sentence) {
                Ok(t) => talker = t,
                Err(e) => {
                    clerk::warn!("{}", e);
                    continue;
                }
            };

            match Identifier::from_str(&sentence) {
                Ok(i) => identifier = i,
                Err(e) => {
                    clerk::warn!("{}", e);
                    continue;
                }
            }
            break;
        }
        (talker, identifier, sentence)
    }
    fn process_multilines(
        &mut self,
        talker: Talker,
        identifier: Identifier,
        sentence: String,
    ) -> Option<(Talker, Identifier, std::string::String)> {
        let (count, idx): (usize, usize) = match identifier {
            Identifier::GSV => {
                let parts = sentence.split(",").collect::<Vec<&str>>();
                let c = match parts.get(1) {
                    Some(c) => c,
                    None => {
                        clerk::warn!("Sentense has no count: {}", sentence);
                        return None;
                    }
                };
                let i = match parts.get(2) {
                    Some(c) => c,
                    None => {
                        clerk::warn!("Sentense has no ordinal position: {}", sentence);
                        return None;
                    }
                };
                let c: usize = match c.parse() {
                    Ok(c) => c,
                    Err(e) => {
                        clerk::warn!("{}", e);
                        return None;
                    }
                };
                let i: usize = match i.parse() {
                    Ok(i) => i,
                    Err(e) => {
                        clerk::warn!("{}", e);
                        return None;
                    }
                };
                (c, i)
            }
            //single line
            i => panic!("Identifier `{:?}` is not a multiline nmea.", i),
        };
        if count == idx { todo!() } else { todo!() }
    }
    fn dispatch_by_lines(&mut self) -> (Talker, Identifier, std::string::String) {
        loop {
            let (talker, identifier, sentence) = self.preprocess();
            match identifier {
                // one line
                Identifier::DHV
                | Identifier::GGA
                | Identifier::GLL
                | Identifier::GSA
                | Identifier::GST
                | Identifier::RMC
                | Identifier::VTG
                | Identifier::ZDA => return (talker, identifier, sentence),

                //multiline
                Identifier::GSV => {
                    if let Some(result) = self.process_multilines(talker, identifier, sentence) {
                        return result;
                    } else {
                        continue;
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
    type Item = (Talker, Identifier, std::string::String);

    fn next(&mut self) -> Option<Self::Item> { Some(self.dispatch_by_lines()) }
}
