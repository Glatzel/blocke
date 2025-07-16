use crossterm::event::KeyCode;
use ratatui::widgets::Widget;

pub struct TabNmea {}
impl Default for TabNmea {
    fn default() -> Self { Self {} }
}
impl super::ITab for TabNmea {
    fn handle_key(&mut self, key: KeyCode) { todo!() }
}
impl Widget for TabNmea {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
