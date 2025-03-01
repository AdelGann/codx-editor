use color_eyre::Result;
use editor::config::Config;
use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};
mod editor {
    pub mod config;
}

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        let mut conf = Config::new();
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            conf.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from("Codx Editor")
            .bold()
            .blue()
            .centered();
        let text = "Welcome to Codx Editor!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Alt-q` to quit";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }
}
