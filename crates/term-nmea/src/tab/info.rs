use ratatui::widgets::Widget;

pub struct TabInfo {}
impl Default for TabInfo {
    fn default() -> Self { Self {} }
}
impl super::ITab for TabInfo {
    fn handle_key(&mut self, key: crossterm::event::KeyCode) { todo!() }
}
impl Widget for TabInfo {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
