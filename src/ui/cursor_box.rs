use crate::ui::menu_box::Cursor;
use crate::ui::ui::TerminalRenderer;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CursorItem {
    pub index_x: u8,
    pub index_y: u8,
    pub text: String,
}

pub struct CursorBox {
    pub items: Vec<CursorItem>,
    pub cursor: Rc<RefCell<Cursor>>,
}

impl CursorBox {
    pub fn new(items: Vec<CursorItem>, cursor: Rc<RefCell<Cursor>>) -> Self {
        CursorBox { items, cursor }
    }
}

impl TerminalRenderer for CursorBox {
    fn render(&mut self, _: &u64, frame: &mut Frame, rect: Rect) {
        let cursor = self.cursor.borrow();
        let item = self
            .items
            .iter()
            .find(|c| c.index_y == cursor.index_y && c.index_x == cursor.index_x);

        let text: String = item
            .map(|i| i.text.clone())
            .unwrap_or(String::from("Empty"));

        frame.render_widget(
            Paragraph::new(text).block(Block::default().borders(Borders::ALL)),
            rect,
        );
    }
}
