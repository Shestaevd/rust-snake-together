use crate::config::config::AppConfig;
use crate::core::input_loop::{InputMap, InputState};
use crate::model::game_objects::{Food, SnakeHead, SnakeTail, Wall};
use crate::states::log_state::LogState;
use crate::states::main_menu_state::{MainMenuState, State};
use crate::ui::render::render_game;
use crate::ui::ui::StateTerminalDrawer;
use rand::Rng;
use ratatui::DefaultTerminal;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::RwLock;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MoveDirection {
    fn is_opposite(&self, other: &MoveDirection) -> bool {
        matches!(
            (self, other),
            (MoveDirection::Left, MoveDirection::Right)
                | (MoveDirection::Right, MoveDirection::Left)
                | (MoveDirection::Up, MoveDirection::Down)
                | (MoveDirection::Down, MoveDirection::Up)
        )
    }
}

pub struct GridSize {
    pub width: u64,
    pub height: u64,
}

pub struct GameState {
    pub tick_ms: u64,
    pub current_tick_ms: u64,
    pub tick_count: u32,
    pub score: f32,
    pub player: SnakeHead,
    pub grid_size: GridSize,
    pub map: Vec<Wall>,
    pub food: Vec<Food>,
    pub move_direction: MoveDirection,
    pub config: Rc<AppConfig>,
}

impl State for GameState {
    fn run(
        self: &mut Self,
        delta: &u64,
        ls: &RwLock<LogState>,
        is: &RwLock<InputState>,
        terminal: &mut DefaultTerminal,
    ) -> Option<Box<dyn State>> {
        let pressed: Option<InputMap> = is
            .write()
            .ok()
            .and_then(|mut state| state.current_input.take());

        if let Some(im) = pressed {
            let new_direction = match im {
                InputMap::Up => Some(MoveDirection::Up),
                InputMap::Down => Some(MoveDirection::Down),
                InputMap::Left => Some(MoveDirection::Left),
                InputMap::Right => Some(MoveDirection::Right),
                InputMap::Back => {
                    return Some(Box::new(MainMenuState::new(Rc::clone(&self.config))));
                }
                input => {
                    if let Ok(mut lsw) = ls.write() {
                        lsw.push_warning(format!("Unsupported input {}", input))
                    }
                    None
                }
            };

            if let Some(direction) = new_direction
                && direction != self.move_direction
                && !direction.is_opposite(&self.move_direction)
            {
                self.move_direction = direction;
            }
        };

        self.current_tick_ms += delta;
        if self.current_tick_ms >= self.tick_ms {
            self.current_tick_ms = 0;

            let next_player_position: (u64, u64) =
                next_cord(self.player.x, self.player.y, &self.move_direction);

            let is_next_food: Option<(usize, &Food)> = self
                .food
                .iter()
                .enumerate()
                .find(|(_, f)| f.x == next_player_position.0 && f.y == next_player_position.1);

            let mut add_segment: bool = false;

            self.tick_count += 1;
            if let Some((index, _)) = is_next_food {
                add_segment = true;
                self.score += 1f32;
                self.food.remove(index);
                spawn_next_food(self);
            }

            move_snake_to(
                next_player_position.0,
                next_player_position.1,
                &mut self.player,
                add_segment,
            );

            let is_next_wall: bool = self
                .map
                .iter()
                .any(|f| f.x == next_player_position.0 && f.y == next_player_position.1);

            if is_next_wall {
                return Some(Box::new(MainMenuState::new(Rc::clone(&self.config))));
            }
        }

        self.draw(delta, terminal, ls);

        None
    }
}

impl StateTerminalDrawer for GameState {
    fn draw(
        &mut self,
        _delta: &u64,
        terminal: &mut DefaultTerminal,
        log_state: &RwLock<LogState>,
    ) {
        let config = Rc::clone(&self.config);
        render_game(terminal, self, log_state, &config.terminal_min_size);
    }
}

fn spawn_next_food(game_state: &mut GameState) {
    let mut occupied: HashSet<(u64, u64)> =
        game_state.map.iter().map(|wall| (wall.x, wall.y)).collect();
    occupied.extend(game_state.food.iter().map(|food| (food.x, food.y)));

    occupied.insert((game_state.player.x, game_state.player.y));
    let mut segment: Option<&SnakeTail> = Some(&game_state.player.tail);
    while let Some(tail) = segment {
        occupied.insert((tail.x, tail.y));
        segment = tail.next.as_deref();
    }

    let head = (game_state.player.x, game_state.player.y);
    let adjacent_to_head: HashSet<(u64, u64)> = [
        (head.0.wrapping_sub(1), head.1),
        (head.0 + 1, head.1),
        (head.0, head.1.wrapping_sub(1)),
        (head.0, head.1 + 1),
    ]
        .into_iter()
        .collect();

    let mut free_cells: Vec<(u64, u64)> = Vec::new();
    for x in 0..game_state.grid_size.width {
        for y in 0..game_state.grid_size.height {
            if !occupied.contains(&(x, y)) {
                free_cells.push((x, y));
            }
        }
    }

    if free_cells.is_empty() {
        return;
    }

    let preferred: Vec<(u64, u64)> = free_cells
        .iter()
        .copied()
        .filter(|pos| !adjacent_to_head.contains(pos))
        .collect();

    let candidates = if preferred.is_empty() {
        &free_cells
    } else {
        &preferred
    };

    let index = rand::thread_rng().gen_range(0..candidates.len());
    let (x, y) = candidates[index];

    game_state.food.push(Food { x, y });
}

fn next_cord(x: u64, y: u64, direction: &MoveDirection) -> (u64, u64) {
    match direction {
        MoveDirection::Down => (x, y + 1),
        MoveDirection::Up => (x, y - 1),
        MoveDirection::Left => (x - 1, y),
        MoveDirection::Right => (x + 1, y),
    }
}

fn move_snake_to(x: u64, y: u64, snake: &mut SnakeHead, add_segment: bool) {
    let mut prev_x: u64 = snake.x;
    let mut prev_y: u64 = snake.y;
    snake.x = x;
    snake.y = y;

    let mut next: Option<&mut SnakeTail> = Some(&mut snake.tail);
    while let Some(segment) = next {
        std::mem::swap(&mut prev_x, &mut segment.x);
        std::mem::swap(&mut prev_y, &mut segment.y);
        if add_segment && segment.next.is_none() {
            segment.next = Some(Box::new(SnakeTail {
                x: prev_x,
                y: prev_y,
                next: None,
            }));
            break;
        }
        next = segment.next.as_deref_mut();
    }
}

impl GameState {
    pub fn new(level: (Vec<Wall>, GridSize), tick_ms: u64, config: Rc<AppConfig>) -> Self {
        let (map, grid_size) = level;

        GameState {
            tick_ms,
            current_tick_ms: 0,
            tick_count: 0,
            score: 0f32,
            player: SnakeHead {
                x: 3,
                y: 1,
                tail: SnakeTail {
                    x: 2,
                    y: 1,
                    next: Some(Box::new(SnakeTail {
                        x: 1,
                        y: 1,
                        next: Some(Box::new(SnakeTail {
                            x: 1,
                            y: 2,
                            next: None,
                        })),
                    })),
                },
            },
            grid_size,
            map,
            food: vec![Food { x: 5, y: 1 }],
            move_direction: MoveDirection::Right,
            config,
        }
    }
}
