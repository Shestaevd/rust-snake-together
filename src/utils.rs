use crate::config::config::TerminalMinSize;
use crate::core::input_loop::{InputMap, InputState};
use crate::states::log_state::LogState;
use crate::states::main_menu_state::State;
use crate::ui::menu_box::MenuBox;
use crate::ui::ui::StateTerminalDrawer;
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use std::collections::HashSet;
use std::sync::RwLock;

pub trait HasMenuBox {
    fn menu_box_mut(&mut self) -> &mut MenuBox;
}

pub fn run_navigable_state<S>(
    state: &mut S,
    delta: &u64,
    ls: &RwLock<LogState>,
    is: &RwLock<InputState>,
    terminal: &mut DefaultTerminal,
    other: Option<Box<dyn FnOnce(InputMap, &RwLock<LogState>) -> Option<Box<dyn State>>>>,
) -> Option<Box<dyn State>>
where
    S: StateTerminalDrawer + HasMenuBox,
{
    let pressed: Option<InputMap> = is
        .write()
        .ok()
        .and_then(|mut input_state| input_state.current_input.take());

    let next_state: Option<Box<dyn State>> = if let Some(im) = pressed {
        navigate_or(state.menu_box_mut(), im, ls, other)
    } else {
        None
    };

    state.draw(delta, terminal, ls);

    next_state
}

pub fn navigate_or(
    menu_box: &mut MenuBox,
    input: InputMap,
    ls: &RwLock<LogState>,
    other: Option<Box<dyn FnOnce(InputMap, &RwLock<LogState>) -> Option<Box<dyn State>>>>,
) -> Option<Box<dyn State>> {
    match input {
        InputMap::Up => {
            menu_box.move_up();
            None
        }
        InputMap::Down => {
            menu_box.move_down();
            None
        }
        InputMap::Left => {
            menu_box.move_left();
            None
        }
        InputMap::Right => {
            menu_box.move_right();
            None
        }
        InputMap::Enter => menu_box.cursor_item().map(|item| (item.pressed)()),
        input => match other {
            Some(handler) => handler(input, ls),
            None => {
                if let Ok(mut lsw) = ls.write() {
                    lsw.push_warning(format!("Unsupported input {}", input))
                }
                None
            }
        },
    }
}

pub fn is_terminal_too_small(area: Rect, min_size: &TerminalMinSize) -> bool {
    area.width < min_size.width || area.height < min_size.height
}

pub struct RenderedCell {
    pub x: u64,
    pub y: u64,
    pub render_char: char,
}

fn connector_char(up: bool, down: bool, left: bool, right: bool) -> char {
    match (up, down, left, right) {
        (false, false, false, false) => '─',
        (false, false, false, true) => '─',
        (false, false, true, false) => '─',
        (false, false, true, true) => '─',
        (false, true, false, false) => '│',
        (false, true, false, true) => '┌',
        (false, true, true, false) => '┐',
        (false, true, true, true) => '┬',
        (true, false, false, false) => '│',
        (true, false, false, true) => '└',
        (true, false, true, false) => '┘',
        (true, false, true, true) => '┴',
        (true, true, false, false) => '│',
        (true, true, false, true) => '├',
        (true, true, true, false) => '┤',
        (true, true, true, true) => '┼',
    }
}

pub fn get_connected_render_chars(positions: &[(u64, u64)]) -> Vec<RenderedCell> {
    let position_set: HashSet<(u64, u64)> = positions.iter().copied().collect();

    positions
        .iter()
        .map(|&(x, y)| {
            let up = y > 0 && position_set.contains(&(x, y - 1));
            let down = position_set.contains(&(x, y + 1));
            let left = x > 0 && position_set.contains(&(x - 1, y));
            let right = position_set.contains(&(x + 1, y));

            RenderedCell {
                x,
                y,
                render_char: connector_char(up, down, left, right),
            }
        })
        .collect()
}

pub fn get_snake_render_chars(segments: &[(u64, u64)]) -> Vec<RenderedCell> {
    segments
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| {
            let prev = i.checked_sub(1).and_then(|idx| segments.get(idx));
            let next = segments.get(i + 1);

            let is_above = |&(ox, oy): &(u64, u64)| ox == x && oy + 1 == y;
            let is_below = |&(ox, oy): &(u64, u64)| ox == x && oy == y + 1;
            let is_left = |&(ox, oy): &(u64, u64)| oy == y && ox + 1 == x;
            let is_right = |&(ox, oy): &(u64, u64)| oy == y && ox == x + 1;

            let up = prev.is_some_and(is_above) || next.is_some_and(is_above);
            let down = prev.is_some_and(is_below) || next.is_some_and(is_below);
            let left = prev.is_some_and(is_left) || next.is_some_and(is_left);
            let right = prev.is_some_and(is_right) || next.is_some_and(is_right);

            RenderedCell {
                x,
                y,
                render_char: connector_char(up, down, left, right),
            }
        })
        .collect()
}
