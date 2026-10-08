use crate::states::log_state::LogState;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Rect;
use std::sync::RwLock;

pub trait TerminalRenderer {
    fn render(&mut self, delta: &u64, frame: &mut Frame, rect: Rect);
}

pub trait StateTerminalDrawer {
    fn draw(&mut self, delta: &u64, terminal: &mut DefaultTerminal, log_state: &RwLock<LogState>);
}