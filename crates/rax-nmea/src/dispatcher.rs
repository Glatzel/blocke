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
}
impl<'a, R> Iterator for Dispatcher<'a, R>
where
    R: IRaxReader,
{
    type Item = (Talker, Identifier, std::string::String);

    fn next(&mut self) -> Option<Self::Item> {
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
            | Identifier::ZDA => Some((talker, identifier, sentence)),

            //multiline
            Identifier::GSV => todo!(),
        }
    }
}
