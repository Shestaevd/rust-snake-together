use crate::config::config::AppConfig;
use crate::core::input_loop::{InputMap, InputState};
use crate::model::level_prefab::{
    level_1, level_2, level_3, level_4, prefab_to_walls,
};
use crate::states::choose_difficulty_state::ChooseDifficultyState;
use crate::states::log_state::LogState;
use crate::states::main_menu_state::{MainMenuState, State};
use crate::ui::animation::Animation;
use crate::ui::cursor_box::{CursorBox, CursorItem};
use crate::ui::menu_box::{MenuBox, MenuItem};
use crate::ui::render::{fill_walls, render_choose_level};
use crate::ui::ui::StateTerminalDrawer;
use crate::utils::{HasMenuBox, run_navigable_state};
use ratatui::DefaultTerminal;
use ratatui::prelude::Span;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::RwLock;

pub struct ChooseLevelState {
    pub menu_box: MenuBox,
    pub cursor_box: CursorBox,
    pub animation: Rc<RefCell<Animation>>,
    pub config: Rc<AppConfig>,
}

impl HasMenuBox for ChooseLevelState {
    fn menu_box_mut(&mut self) -> &mut MenuBox {
        &mut self.menu_box
    }
}

impl State for ChooseLevelState {
    fn run(
        self: &mut Self,
        delta: &u64,
        ls: &RwLock<LogState>,
        is: &RwLock<InputState>,
        terminal: &mut DefaultTerminal,
    ) -> Option<Box<dyn State>> {
        let config = Rc::clone(&self.config);

        run_navigable_state(
            self,
            delta,
            ls,
            is,
            terminal,
            Some(Box::new(move |input, ls| match input {
                InputMap::Back => {
                    Some(Box::new(MainMenuState::new(config)) as Box<dyn State>)
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

impl StateTerminalDrawer for ChooseLevelState {
    fn draw(&mut self, delta: &u64, terminal: &mut DefaultTerminal, log_state: &RwLock<LogState>) {
        render_choose_level(delta, terminal, self, log_state);
    }
}

impl ChooseLevelState {
    pub fn new(animation: Option<Rc<RefCell<Animation>>>, config: Rc<AppConfig>) -> Self {
        let animation = animation.unwrap_or_else(|| Rc::new(RefCell::new(Animation::reveal())));

        let levels: [(&str, fn() -> Vec<Vec<char>>); 4] = [
            ("Level 1", level_1),
            ("Level 2", level_2),
            ("Level 3", level_3),
            ("Level 4", level_4),
        ];

        let mut menu_items = Vec::new();
        let mut cursor_items = Vec::new();

        for (i, (name, level_fn)) in levels.into_iter().enumerate() {
            let index_y = i as u8;

            let (walls, grid_size) =
                prefab_to_walls(level_fn()).expect("built-in level prefab is valid");

            let mut grid: Vec<Vec<Span<'_>>> =
                vec![vec![Span::raw(" "); grid_size.width as usize]; grid_size.height as usize];

            fill_walls(&mut grid, &walls);

            let preview = grid
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|span| span.to_string())
                        .collect::<Vec<String>>()
                        .concat()
                })
                .collect::<Vec<String>>()
                .join("\n");

            cursor_items.push(CursorItem {
                index_x: 0,
                index_y,
                text: preview,
            });

            let item_config = Rc::clone(&config);
            let item_animation = Rc::clone(&animation);

            menu_items.push(MenuItem {
                index_x: 0,
                index_y,
                text: String::from(name),
                pressed: Box::new(move || {
                    Box::new(ChooseDifficultyState::new(
                        level_fn,
                        Rc::clone(&item_config),
                        Some(Rc::clone(&item_animation)),
                    ))
                }),
            });
        }

        let menu_box = MenuBox::new(menu_items);
        let cursor_box = CursorBox::new(cursor_items, menu_box.cursor_handle());

        ChooseLevelState {
            menu_box,
            cursor_box,
            animation,
            config,
        }
    }
}
