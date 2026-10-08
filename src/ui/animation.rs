use crate::ui::logo::{SNAKE_TOGETHER_FRAME_COUNT, snake_together_frame};
use crate::ui::ui::TerminalRenderer;
use ratatui::Frame;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, Padding, Paragraph};

pub struct Animation {
    pub terminal_animation_space_ms: u64,
    pub terminal_animation_position_ms: u64,
    pub terminal_animation_frame: u8,
    pub frame_count: u8,
    pub get_frame: Box<dyn Fn(usize) -> Option<&'static str>>,
    pub repeat: bool
}

impl Animation {
    
    pub fn reveal() -> Self {
        Animation {
            repeat: false,
            terminal_animation_space_ms: 100,
            terminal_animation_position_ms: 0,
            terminal_animation_frame: 0,
            frame_count: SNAKE_TOGETHER_FRAME_COUNT as u8,
            get_frame: Box::new(snake_together_frame),
        }
    }

    pub fn progress(&mut self, delta_ms: &u64) -> u8 {
        let last_frame = self.frame_count.saturating_sub(1);

        if self.terminal_animation_frame >= last_frame {
            self.terminal_animation_frame = last_frame;
            return last_frame;
        }

        self.terminal_animation_position_ms += delta_ms;
        if self.terminal_animation_position_ms > self.terminal_animation_space_ms {
            self.terminal_animation_position_ms = 0;
            self.terminal_animation_frame += 1;
        }

        self.terminal_animation_frame.min(last_frame)
    }

    pub fn reset(&mut self) {
        self.terminal_animation_position_ms = 0;
        self.terminal_animation_frame = 0;
    }
}

impl TerminalRenderer for Animation {
    fn render(&mut self, delta: &u64, frame: &mut Frame, rect: Rect) {
        let frame_index: u8 = self.progress(delta);

        if let Some(banner) = (self.get_frame)(frame_index as usize) {
            frame.render_widget(
                Paragraph::new(banner)
                    .block(Block::default().padding(Padding::new(2, 2, 5, 5)))
                    .alignment(Alignment::Center),
                rect,
            );
        }

        if self.repeat && self.terminal_animation_frame + 1 == self.frame_count {
            self.reset()
        }
    }
}
