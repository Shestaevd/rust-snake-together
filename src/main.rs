use crate::config::config::{AppConfig, get_config};
use crate::core::input_loop::input_loop;
use crate::states::input_state::{InputState, InputType};
use crate::states::log_state::LogState;
use crate::states::main_menu_state::{MainMenuState, State};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use ratatui::DefaultTerminal;

mod config;
mod core;
mod model;
mod states;
mod ui;
mod utils;

#[tokio::main]
async fn main() {
    let config: Rc<AppConfig> = Rc::new(get_config().unwrap());

    let input_state: Arc<RwLock<InputState>> = Arc::new(RwLock::new(InputState {
        input_type: InputType::Navigate,
        current_input: None,
    }));

    let log_state: Arc<RwLock<LogState>> = Arc::new(RwLock::new(LogState::new()));
    let log_state_il: Arc<RwLock<LogState>> = Arc::clone(&log_state);

    let _ = input_loop(&config, log_state_il, Arc::clone(&input_state));

    let mut terminal: DefaultTerminal = ratatui::init();
    let mut current_state: Box<dyn State> = Box::new(MainMenuState::new(None, Rc::clone(&config)));

    let frame_interval = Duration::from_millis(16);
    let mut last_tick: Instant = Instant::now();

    loop {
        tokio::time::sleep(frame_interval).await;

        let now: Instant = Instant::now();
        let delta: u64 = now.duration_since(last_tick).as_millis() as u64;

        last_tick = now;

        if let Some(next_state) =
            current_state.run(&delta, &log_state, &input_state, &mut terminal)
        {
            current_state = next_state;
        }
    }
}
