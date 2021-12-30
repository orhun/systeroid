use tui::backend::Backend;
use tui::widgets::Paragraph;
use tui::Frame;

/// Renders the user interface.
pub fn render<B: Backend>(frame: &mut Frame<'_, B>) {
    frame.render_widget(Paragraph::new(env!("CARGO_PKG_NAME")), frame.size());
}
