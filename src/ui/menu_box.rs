use crate::states::main_menu_state::State;
use crate::ui::ui::TerminalRenderer;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MenuItem {
    pub index_x: u8,
    pub index_y: u8,
    pub text: String,
    pub pressed: Box<dyn Fn() -> Box<dyn State>>,
}

pub struct Cursor {
    pub index_x: u8,
    pub index_y: u8,
}

pub struct MenuBox {
    pub cursor: Rc<RefCell<Cursor>>,
    pub menu_items: Vec<MenuItem>,
}

impl MenuBox {
    pub fn new(menu_items: Vec<MenuItem>) -> Self {
        MenuBox {
            cursor: Rc::new(RefCell::new(Cursor {
                index_x: 0,
                index_y: 0,
            })),
            menu_items,
        }
    }

    pub fn cursor_handle(&self) -> Rc<RefCell<Cursor>> {
        Rc::clone(&self.cursor)
    }

    fn cursor_pos(&self) -> (u8, u8) {
        let cursor = self.cursor.borrow();
        (cursor.index_x, cursor.index_y)
    }

    pub fn move_up(&mut self) {
        let (index_x, index_y) = self.cursor_pos();
        if let Some(index_y) = index_y.checked_sub(1) {
            self.try_move_cursor(index_x, index_y);
        }
    }

    pub fn move_right(&mut self) {
        let (index_x, index_y) = self.cursor_pos();
        if let Some(index_x) = index_x.checked_add(1) {
            self.try_move_cursor(index_x, index_y);
        }
    }

    pub fn move_down(&mut self) {
        let (index_x, index_y) = self.cursor_pos();
        if let Some(index_y) = index_y.checked_add(1) {
            self.try_move_cursor(index_x, index_y);
        }
    }

    pub fn move_left(&mut self) {
        let (index_x, index_y) = self.cursor_pos();
        if let Some(index_x) = index_x.checked_sub(1) {
            self.try_move_cursor(index_x, index_y);
        }
    }

    fn try_move_cursor(&mut self, index_x: u8, index_y: u8) {
        let exists = self
            .menu_items
            .iter()
            .any(|item| item.index_x == index_x && item.index_y == index_y);

        if exists {
            let mut cursor = self.cursor.borrow_mut();
            cursor.index_x = index_x;
            cursor.index_y = index_y;
        }
    }

    pub fn cursor_item(&self) -> Option<&MenuItem> {
        let (index_x, index_y) = self.cursor_pos();
        self.menu_items
            .iter()
            .find(|s| s.index_x == index_x && s.index_y == index_y)
    }
}

impl TerminalRenderer for MenuBox {
    fn render(&mut self, _: &u64, frame: &mut Frame, rect: Rect) {
        let mut items: Vec<_> = self.menu_items.iter().collect();
        items.sort_by_key(|item| (item.index_y, item.index_x));

        let (cursor_x, cursor_y) = self.cursor_pos();
        let lines: Vec<Line> = items
            .into_iter()
            .map(|item| {
                let is_selected = item.index_x == cursor_x && item.index_y == cursor_y;
                let text: String = if is_selected {
                    format!(">{}<", item.text)
                } else {
                    item.text.clone()
                };
                Line::from(text)
            })
            .collect();

        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .padding(Padding::new(2, 2, 2, 2))
                        .borders(Borders::ALL),
                )
                .alignment(Alignment::Center),
            rect,
        )
    }
}
