use crate::config::config::AppConfig;
use crate::core::input_loop::InputState;
use crate::states::choose_level_state::ChooseLevelState;
use crate::states::log_state::LogState;
use crate::ui::animation::Animation;
use crate::ui::menu_box::{MenuBox, MenuItem};
use crate::ui::render::render_menu;
use crate::ui::ui::StateTerminalDrawer;
use crate::utils::{HasMenuBox, run_navigable_state};
use ratatui::DefaultTerminal;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;

pub trait State {
    fn run(
        self: &mut Self,
        delta: &u64,
        ls: &RwLock<LogState>,
        is: &RwLock<InputState>,
        terminal: &mut DefaultTerminal,
    ) -> Option<Box<dyn State>>;
}

pub struct MainMenuState {
    pub menu_box: MenuBox,
    pub animation: Rc<RefCell<Animation>>,
    pub config: Rc<AppConfig>,
}

impl HasMenuBox for MainMenuState {
    fn menu_box_mut(&mut self) -> &mut MenuBox {
        &mut self.menu_box
    }
}

impl State for MainMenuState {
    fn run(
        self: &mut Self,
        delta: &u64,
        ls: &RwLock<LogState>,
        is: &RwLock<InputState>,
        terminal: &mut DefaultTerminal,
    ) -> Option<Box<dyn State>> {
        run_navigable_state(self, delta, ls, is, terminal, None)
    }
}

impl StateTerminalDrawer for MainMenuState {
    fn draw(&mut self, delta: &u64, terminal: &mut DefaultTerminal, log_state: &RwLock<LogState>) {
        render_menu(delta, terminal, self, log_state);
    }
}

impl MainMenuState {
    pub fn new(config: Rc<AppConfig>) -> Self {
        let animation = Rc::new(RefCell::new(Animation::reveal()));

        let start = MenuItem {
            index_x: 0,
            index_y: 0,
            text: String::from("Start"),
            pressed: {
                let animation = Rc::clone(&animation);
                let config = Rc::clone(&config);
                Box::new(move || {
                    Box::new(ChooseLevelState::new(
                        Some(Rc::clone(&animation)),
                        Rc::clone(&config),
                    ))
                })
            },
        };

        let create_room = MenuItem {
            index_x: 0,
            index_y: 1,
            text: String::from("Create room"),
            pressed: Box::new(|| todo!()),
        };

        let options = MenuItem {
            index_x: 0,
            index_y: 2,
            text: String::from("Options"),
            pressed: Box::new(|| todo!()),
        };

        let exit = MenuItem {
            index_x: 0,
            index_y: 3,
            text: String::from("Exit"),
            pressed: Box::new(|| {
                ratatui::restore();
                std::process::exit(0);
            }),
        };

        MainMenuState {
            menu_box: MenuBox::new(vec![start, create_room, options, exit]),
            animation,
            config,
        }
    }
}
