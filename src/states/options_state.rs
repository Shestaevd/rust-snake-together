use crate::config::config::AppConfig;
use crate::core::input_loop::{InputMap, InputState};
use crate::states::log_state::LogState;
use crate::states::main_menu_state::{MainMenuState, State};
use crate::ui::animation::Animation;
use crate::ui::menu_box::MenuBox;
use crate::ui::ui::StateTerminalDrawer;
use ratatui::DefaultTerminal;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;
use crate::utils::{run_navigable_state, HasMenuBox};

pub struct OptionsState {
    pub menu_box: MenuBox,
    pub animation: Rc<RefCell<Animation>>,
    pub config: Rc<AppConfig>,
}

impl OptionsState {
    pub fn new(animation: Rc<RefCell<Animation>>, config: Rc<AppConfig>) -> OptionsState {
        
        todo!()
    }
}

impl State for OptionsState {
    fn run(
        self: &mut Self,
        delta: &u64,
        ls: &RwLock<LogState>,
        is: &RwLock<InputState>,
        terminal: &mut DefaultTerminal,
    ) -> Option<Box<dyn State>> {
        let animation = Rc::clone(&self.animation);
        let config = Rc::clone(&self.config);

        run_navigable_state(
            self,
            delta,
            ls,
            is,
            terminal,
            Some(Box::new(move |input, ls| match input {
                InputMap::Back => {
                    Some(Box::new(MainMenuState::new(Some(animation), config)) as Box<dyn State>)
                }
                other => {
                    if let Ok(mut lsw) = ls.write() {
                        lsw.push_warning(format!("Unsupported input {}", other))
                    }
                    None
                }
            })),
        )
    }
}

impl StateTerminalDrawer for OptionsState {
    fn draw(&mut self, delta: &u64, terminal: &mut DefaultTerminal, log_state: &RwLock<LogState>) {
        todo!()
    }
}

impl HasMenuBox for OptionsState {
    fn menu_box_mut(&mut self) -> &mut MenuBox {
        &mut self.menu_box
    }
}
