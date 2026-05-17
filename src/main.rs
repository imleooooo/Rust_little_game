mod space_defender;
mod angry_birds;

use raylib::prelude::*;
use space_defender::SpaceDefenderGame;
use angry_birds::AngryBirdsGame;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

enum GameState {
    Menu,
    SpaceDefender,
    AngryBirds,
}

enum MenuSelection {
    None,
    SpaceDefender,
    AngryBirds,
}

impl MenuSelection {
    fn is_selected(&self) -> bool {
        !matches!(self, MenuSelection::None)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Little Games")
        .build();

    rl.set_target_fps(60);

    let mut state = GameState::Menu;
    let mut menu_selection = MenuSelection::None;
    let mut sd_game: Option<SpaceDefenderGame> = None;
    let mut ab_game: Option<AngryBirdsGame> = None;

    while !rl.window_should_close() {
        match state {
            GameState::Menu => {
                if rl.is_key_pressed(KeyboardKey::KEY_ONE) || rl.is_key_pressed(KeyboardKey::KEY_KP_1) {
                    menu_selection = MenuSelection::SpaceDefender;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_TWO) || rl.is_key_pressed(KeyboardKey::KEY_KP_2) {
                    menu_selection = MenuSelection::AngryBirds;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    break;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) && menu_selection.is_selected() {
                    match menu_selection {
                        MenuSelection::SpaceDefender => {
                            sd_game = Some(SpaceDefenderGame::new());
                            state = GameState::SpaceDefender;
                        }
                        MenuSelection::AngryBirds => {
                            ab_game = Some(AngryBirdsGame::new());
                            state = GameState::AngryBirds;
                        }
                        MenuSelection::None => {}
                    }
                }

                let mut d = rl.begin_drawing(&thread);
                draw_menu(&mut d, &menu_selection);
            }
            GameState::SpaceDefender => {
                if let Some(ref mut game) = sd_game {
                    if game.wants_to_quit() {
                        sd_game = None;
                        menu_selection = MenuSelection::None;
                        state = GameState::Menu;
                        continue;
                    }

                    game.update(&rl);

                    let mut d = rl.begin_drawing(&thread);
                    game.draw(&mut d);
                }
            }
            GameState::AngryBirds => {
                if let Some(ref mut game) = ab_game {
                    if game.wants_to_quit() {
                        ab_game = None;
                        menu_selection = MenuSelection::None;
                        state = GameState::Menu;
                        continue;
                    }

                    game.update(&rl);

                    let mut d = rl.begin_drawing(&thread);
                    game.draw(&mut d);
                }
            }
        }
    }
}

fn draw_menu(d: &mut RaylibDrawHandle, selected: &MenuSelection) {
    d.clear_background(Color::new(30, 30, 50, 255));

    d.draw_text("Rust Little Games", SCREEN_WIDTH / 2 - 150, 80, 40, Color::WHITE);

    let sd_selected = matches!(selected, MenuSelection::SpaceDefender);
    let sd_color = if sd_selected { Color::YELLOW } else { Color::WHITE };
    d.draw_text("1. Space Defender", SCREEN_WIDTH / 2 - 120, 200, 30, sd_color);

    let ab_selected = matches!(selected, MenuSelection::AngryBirds);
    let ab_color = if ab_selected { Color::YELLOW } else { Color::WHITE };
    d.draw_text("2. Angry Birds", SCREEN_WIDTH / 2 - 100, 280, 30, ab_color);

    d.draw_text("Press 1 or 2 to select, ENTER to start", SCREEN_WIDTH / 2 - 200, 400, 20, Color::GRAY);
    d.draw_text("Press ESC to quit", SCREEN_WIDTH / 2 - 80, 440, 20, Color::GRAY);
}