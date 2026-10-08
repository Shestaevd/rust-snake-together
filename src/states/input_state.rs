use std::fmt;
use std::fmt::Display;

pub enum InputMap {
    Up,
    Down,
    Left,
    Right,
    Back,
    Enter,
    Num(u64),
    Char(char),
}

impl Display for InputMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputMap::Up => write!(f, "Up"),
            InputMap::Down => write!(f, "Down"),
            InputMap::Left => write!(f, "Left"),
            InputMap::Right => write!(f, "Right"),
            InputMap::Back => write!(f, "Back"),
            InputMap::Enter => write!(f, "Enter"),
            InputMap::Num(n) => write!(f, "Num({n})"),
            InputMap::Char(c) => write!(f, "Char({c})"),
        }
    }
}



pub enum InputType {
    Navigate,
    InputValue,
}

pub struct InputState {
    pub input_type: InputType,
    pub current_input: Option<InputMap>,
}
