use crate::config::config::{AppConfig, DifficultyLevel};
use crate::core::input_loop::{InputMap, InputState};
use crate::model::game_objects::Wall;
use crate::model::level_prefab::prefab_to_walls;
use crate::states::choose_level_state::ChooseLevelState;
use crate::states::game_state::{GameState, GridSize};
use crate::states::log_state::LogState;
use crate::states::main_menu_state::State;
use crate::ui::animation::Animation;
use crate::ui::menu_box::{MenuBox, MenuItem};
use crate::ui::render::render_choose_difficulty;
use crate::ui::ui::StateTerminalDrawer;
use crate::utils::{HasMenuBox, run_navigable_state};
use ratatui::DefaultTerminal;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;

pub struct ChooseDifficultyState {
    pub menu_box: MenuBox,
    pub animation: Rc<RefCell<Animation>>,
    pub config: Rc<AppConfig>,
}

impl HasMenuBox for ChooseDifficultyState {
    fn menu_box_mut(&mut self) -> &mut MenuBox {
        &mut self.menu_box
    }
}

impl State for ChooseDifficultyState {
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
                InputMap::Back => Some(Box::new(ChooseLevelState::new(
                    Some(animation),
                    config,
                ))),
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

impl StateTerminalDrawer for ChooseDifficultyState {
    fn draw(&mut self, delta: &u64, terminal: &mut DefaultTerminal, log_state: &RwLock<LogState>) {
        render_choose_difficulty(delta, terminal, self, log_state);
    }
}

impl ChooseDifficultyState {
    pub fn new(
        level_fn: fn() -> Vec<Vec<char>>,
        config: Rc<AppConfig>,
        animation: Option<Rc<RefCell<Animation>>>,
    ) -> Self {
        let difficulty = &config.difficulty;
        let difficulties: [(&str, DifficultyLevel); 3] = [
            ("Easy", difficulty.easy),
            ("Advanced", difficulty.advanced),
            ("Hard", difficulty.hard),
        ];

        let menu_items = difficulties
            .into_iter()
            .enumerate()
            .map(|(i, (name, level_difficulty))| {
                let item_config = Rc::clone(&config);

                MenuItem {
                    index_x: 0,
                    index_y: i as u8,
                    text: String::from(name),
                    pressed: Box::new(move || {
                        let level: (Vec<Wall>, GridSize) = prefab_to_walls(level_fn())
                            .expect("built-in level prefab is valid");
                        Box::new(GameState::new(level, level_difficulty, Rc::clone(&item_config)))
                            as Box<dyn State>
                    }),
                }
            })
            .collect();

        ChooseDifficultyState {
            menu_box: MenuBox::new(menu_items),
            animation: animation.unwrap_or_else(|| Rc::new(RefCell::new(Animation::reveal()))),
            config,
        }
    }
}
