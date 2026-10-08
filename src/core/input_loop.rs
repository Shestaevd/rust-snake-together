use crate::config::config::AppConfig;
pub(crate) use crate::states::input_state::{InputMap, InputState, InputType};
use crate::states::log_state::LogState;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;

pub fn input_loop(
    app_config: &AppConfig,
    log_state: Arc<RwLock<LogState>>,
    input_state: Arc<RwLock<InputState>>,
) -> JoinHandle<()> {
    let app_config = app_config.clone();
    tokio::spawn(async move {
        loop {
            
            match event::read() {
                Ok(Event::Key(key_event)) => {
                    if let Ok(mut ls) = log_state.write() {
                        ls.push_info(format!("{:?}", key_event.code));
                    }

                    if let Ok(mut is) = input_state.write() {
                        match is.input_type {
                            InputType::Navigate => {
                                handle_navigate(&app_config, &mut is, &key_event)
                            }
                            InputType::InputValue => {
                                handle_input_value(&app_config, &mut is, &key_event)
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(err) => {
                    if let Ok(mut ls) = log_state.write() {
                        ls.push_error(format!("{:?}", err))
                    }
                }
            }
        }
    })
}

fn handle_navigate(app_config: &AppConfig, input_state: &mut InputState, key_event: &KeyEvent) {
    let input_config = &app_config.input_config;
    let code = key_event.code.to_string();

    let message = if code == input_config.up {
        Some(InputMap::Up)
    } else if code == input_config.down {
        Some(InputMap::Down)
    } else if code == input_config.left {
        Some(InputMap::Left)
    } else if code == input_config.right {
        Some(InputMap::Right)
    } else if code == input_config.back {
        Some(InputMap::Back)
    } else if code == input_config.enter {
        Some(InputMap::Enter)
    } else {
        None
    };

    set_current_input(input_state, message);
}

fn handle_input_value(app_config: &AppConfig, input_state: &mut InputState, key_event: &KeyEvent) {
    let input_config = &app_config.input_config;

    let message = match key_event.code {
        KeyCode::Char(c) if c.is_ascii_digit() => {
            c.to_digit(10).map(|digit| InputMap::Num(digit as u64))
        }
        KeyCode::Char(c) => Some(InputMap::Char(c)),
        _ => {
            let code = key_event.code.to_string();
            if code == input_config.back {
                Some(InputMap::Back)
            } else if code == input_config.enter {
                Some(InputMap::Enter)
            } else {
                None
            }
        }
    };

    set_current_input(input_state, message);
}

fn set_current_input(input_state: &mut InputState, message: Option<InputMap>) {
    if let Some(message) = message {
        input_state.current_input = Some(message);
    }
}
