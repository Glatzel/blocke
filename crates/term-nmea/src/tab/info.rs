use std::collections::VecDeque;
use std::f64::NAN;

use ratatui::widgets::{Block, Paragraph};
use rax::str_parser::StrParserContext;
use rax_nmea::data::{Gga, INmeaData, Identifier, Talker};

#[derive(Default)]
pub struct TabInfo {
    ctx: StrParserContext,
}
impl super::ITab for TabInfo {
    fn handle_key(&mut self, _key: crossterm::event::KeyEvent) {}
    fn handle_mouse(&mut self, _mouse: crossterm::event::MouseEvent) {}
    fn draw(
        &mut self,
        f: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        raw_nmea: &VecDeque<(Talker, Identifier, String)>,
    ) {
        let last_gga = raw_nmea
            .iter()
            .rev()
            .find(|(_, id, _)| id == &Identifier::GGA);
        if let Some((talker, _identifier, sentence)) = last_gga {
            match Gga::new(self.ctx.init(sentence.clone()), *talker) {
                Ok(gga) => {
                    let p = Paragraph::new(format!(
                        "Longitude: {}\nLatitude: {}",
                        gga.lon().unwrap_or(NAN),
                        gga.lat().unwrap_or(NAN)
                    ));
                    f.render_widget(p, area);
                }
                Err(_) => {
                    let p = Paragraph::new("Todo!").block(Block::default().title("Info"));
                    f.render_widget(p, area);
                }
            }
        }
    }
    fn hint(&mut self) -> &'static [&'static str] { &[] }
}
