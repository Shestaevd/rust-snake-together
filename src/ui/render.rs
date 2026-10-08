use crate::config::config::TerminalMinSize;
use crate::model::error::ProgramError;
use crate::model::game_objects::{SnakeHead, SnakeTail, Wall};
use crate::states::choose_difficulty_state::ChooseDifficultyState;
use crate::states::choose_level_state::ChooseLevelState;
use crate::states::game_state::GameState;
use crate::states::log_state::LogState;
use crate::states::main_menu_state::MainMenuState;
use crate::ui::ui::TerminalRenderer;
use crate::utils::{get_connected_render_chars, get_snake_render_chars, is_terminal_too_small};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use std::sync::RwLock;


fn header_body_footer(area: Rect, body_constraint: Constraint) -> (Rect, Rect, Rect) {
    let layout = Layout::vertical([Constraint::Fill(1), body_constraint, Constraint::Fill(1)])
        .split(area);
    (layout[0], layout[1], layout[2])
}

fn render_too_small_message(frame: &mut Frame, area: Rect) {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .split(area);

    frame.render_widget(
        Paragraph::new("Increase terminal size").alignment(Alignment::Center),
        vertical[1],
    );
}

static FOOD_CHAR: char = '✦';
static SNAKE_HEAD_CHAR: char = '◉';

pub fn fill_walls(grid: &mut Vec<Vec<Span<'_>>>, map: &[Wall]) {
    let wall_positions: Vec<(u64, u64)> = map.iter().map(|wall| (wall.x, wall.y)).collect();

    for wall in get_connected_render_chars(&wall_positions) {
        if let Some(cell) = grid
            .get_mut(wall.y as usize)
            .and_then(|row| row.get_mut(wall.x as usize))
        {
            *cell = to_colored_grey(wall.render_char)
        }
    }
}

pub fn render_game_state(game_state: &GameState) -> Result<Paragraph<'_>, ProgramError> {
    let mut grid: Vec<Vec<Span<'_>>> =
        vec![
            vec![Span::raw(" "); game_state.grid_size.width as usize];
            game_state.grid_size.height as usize
        ];

    fill_walls(&mut grid, &game_state.map);

    for food in &game_state.food {
        if let Some(cell) = grid
            .get_mut(food.y as usize)
            .and_then(|row| row.get_mut(food.x as usize))
        {
            *cell = to_colored_orange(FOOD_CHAR)
        }
    }

    let snake_positions = snake_positions(&game_state.player);
    for (i, segment) in get_snake_render_chars(&snake_positions)
        .into_iter()
        .enumerate()
    {
        if let Some(cell) = grid
            .get_mut(segment.y as usize)
            .and_then(|row| row.get_mut(segment.x as usize))
        {
            *cell = if i == 0 {
                to_colored_green(SNAKE_HEAD_CHAR)
            } else {
                to_colored_green(segment.render_char)
            };
        }
    }

    let game_to_render: Vec<Line> = grid.into_iter().map(Line::from).collect();

    Ok(Paragraph::new(game_to_render).block(Block::default().bold().borders(Borders::ALL)))
}

fn to_colored_green<'a>(chr: char) -> Span<'a> {
    Span::styled(chr.to_string(), Style::default().fg(Color::Green))
}

fn to_colored_orange<'a>(chr: char) -> Span<'a> {
    Span::styled(chr.to_string(), Style::default().fg(Color::LightRed))
}

fn to_colored_grey<'a>(chr: char) -> Span<'a> {
    Span::styled(chr.to_string(), Style::default().fg(Color::Gray))
}

fn snake_positions(player: &SnakeHead) -> Vec<(u64, u64)> {
    let mut positions = vec![(player.x, player.y)];

    let mut segment: &SnakeTail = &player.tail;
    positions.push((segment.x, segment.y));
    while let Some(next) = &segment.next {
        positions.push((next.x, next.y));
        segment = next;
    }

    positions
}

pub fn render_logging<'a>(log_state: &RwLock<LogState>) -> Paragraph<'a> {
    let print_string: String = match log_state.read() {
        Ok(log_state) => log_state.get_logs(),
        Err(error) => String::from(error.to_string()),
    };
    Paragraph::new(print_string)
        .wrap(Wrap { trim: true })
        .block(Block::default().title("Log").borders(Borders::ALL))
}

pub fn render_game(
    term: &mut DefaultTerminal,
    game_state: &GameState,
    log_state: &RwLock<LogState>,
    min_size: &TerminalMinSize,
) {
    term.draw(|frame| {
        if is_terminal_too_small(frame.area(), min_size) {
            render_too_small_message(frame, frame.area());
        } else {
            let width = game_state.grid_size.width;
            let height = game_state.grid_size.height;
            let (_header, body, footer) =
                header_body_footer(frame.area(), Constraint::Max(height as u16 + 2));

            let game_space = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Min(width as u16 + 2),
                Constraint::Fill(1),
            ])
            .split(body);

            if let Ok(game_render) = render_game_state(game_state) {
                frame.render_widget(game_render, game_space[1]);
            }
            frame.render_widget(render_logging(log_state), footer);
        }
    })
    .expect("TODO: panic message");
}

pub fn render_menu(
    delta: &u64,
    term: &mut DefaultTerminal,
    menu_state: &mut MainMenuState,
    log_state: &RwLock<LogState>
) {
    term.draw(|frame| {
        if is_terminal_too_small(frame.area(), &menu_state.config.terminal_min_size) {
            render_too_small_message(frame, frame.area());
        } else {
            let (header, body, footer) = header_body_footer(
                frame.area(),
                Constraint::Min(menu_state.menu_box.menu_items.len() as u16 + 6),
            );

            menu_state.animation.borrow_mut().render(delta, frame, header);
            menu_state.menu_box.render(delta, frame, body);
            frame.render_widget(render_logging(log_state), footer);
        }
    })
    .expect("TODO: panic message");
}

pub fn render_choose_difficulty(
    delta: &u64,
    term: &mut DefaultTerminal,
    difficulty_state: &mut ChooseDifficultyState,
    log_state: &RwLock<LogState>
) {
    term.draw(|frame| {
        if is_terminal_too_small(frame.area(), &difficulty_state.config.terminal_min_size) {
            render_too_small_message(frame, frame.area());
        } else {
            let (header, body, footer) = header_body_footer(
                frame.area(),
                Constraint::Min(difficulty_state.menu_box.menu_items.len() as u16 + 6),
            );

            difficulty_state
                .animation
                .borrow_mut()
                .render(delta, frame, header);
            difficulty_state.menu_box.render(delta, frame, body);
            frame.render_widget(render_logging(log_state), footer);
        }
    })
    .expect("TODO: panic message");
}

pub fn render_choose_level(
    delta: &u64,
    term: &mut DefaultTerminal,
    choose_level_state: &mut ChooseLevelState,
    log_state: &RwLock<LogState>
) {
    term.draw(|frame| {
        if is_terminal_too_small(frame.area(), &choose_level_state.config.terminal_min_size) {
            render_too_small_message(frame, frame.area());
        } else {
            let (header, body, footer) =
                header_body_footer(frame.area(), Constraint::Fill(2));

            let columns =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(body);

            choose_level_state
                .animation
                .borrow_mut()
                .render(delta, frame, header);
            choose_level_state.cursor_box.render(delta, frame, columns[0]);
            choose_level_state.menu_box.render(delta, frame, columns[1]);
            frame.render_widget(render_logging(log_state), footer);
        }
    })
    .expect("TODO: panic message");
}
