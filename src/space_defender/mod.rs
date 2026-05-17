pub mod player;
pub mod enemy;
pub mod bullet;

pub use player::Player;
pub use enemy::Enemy;
pub use bullet::Bullet;

use raylib::prelude::*;
use std::time::SystemTime;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const MAX_BULLETS: usize = 20;
const MAX_ENEMIES: usize = 10;

fn get_difficulty(score: i32) -> (f32, f32) {
    let base_speed = 1.5;
    let base_interval = 1.5;
    let speed_increase = (score / 50) as f32 * 0.2;
    let interval_decrease = (score / 50) as f32 * 0.1;
    let speed = base_speed + speed_increase;
    let interval = (base_interval - interval_decrease).max(0.5);
    (speed, interval)
}

pub struct SpaceDefenderGame {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: i32,
    game_over: bool,
    spawn_timer: f32,
    last_spawn_time: f32,
    wants_to_quit: bool,
}

impl SpaceDefenderGame {
    pub fn new() -> Self {
        SpaceDefenderGame {
            player: Player::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            game_over: false,
            spawn_timer: 0.0,
            last_spawn_time: 0.0,
            wants_to_quit: false,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.wants_to_quit = true;
            return;
        }

        if self.game_over {
            if rl.is_key_pressed(KeyboardKey::KEY_R) {
                self.reset();
            }
            return;
        }

        self.player.update(rl);

        let half_width = self.player.width / 2.0;
        if self.player.x - half_width < 0.0 {
            self.player.x = half_width;
        }
        if self.player.x + half_width > SCREEN_WIDTH as f32 {
            self.player.x = SCREEN_WIDTH as f32 - half_width;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.bullets.len() < MAX_BULLETS {
            self.bullets.push(Bullet::new(self.player.x, self.player.y - self.player.height / 2.0));
        }

        for bullet in &mut self.bullets {
            bullet.update();
        }
        self.bullets.retain(|b| b.y > -20.0 && b.active);

        self.spawn_timer += 1.0 / 60.0;
        let current_time = self.spawn_timer;
        let (enemy_speed, spawn_interval) = get_difficulty(self.score);

        if current_time - self.last_spawn_time >= spawn_interval && self.enemies.len() < MAX_ENEMIES {
            let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u32;
            let x = ((seed % (SCREEN_WIDTH as u32 - 100)) + 50) as f32;
            self.enemies.push(Enemy::new(x, enemy_speed));
            self.last_spawn_time = current_time;
        }

        for enemy in &mut self.enemies {
            enemy.update();
        }

        for bullet in &mut self.bullets {
            for enemy in &mut self.enemies {
                if bullet.active && enemy.active {
                    let b_bounds = bullet.get_bounds();
                    let e_bounds = enemy.get_bounds();
                    if b_bounds.check_collision_recs(&e_bounds) {
                        bullet.active = false;
                        enemy.active = false;
                        self.score += 10;
                    }
                }
            }
        }

        for enemy in &self.enemies {
            if enemy.active {
                let e_bounds = enemy.get_bounds();
                let p_bounds = self.player.get_bounds();
                if e_bounds.check_collision_recs(&p_bounds) {
                    self.game_over = true;
                }
                if enemy.y > SCREEN_HEIGHT as f32 + 20.0 {
                    self.game_over = true;
                }
            }
        }

        self.enemies.retain(|e| e.y < SCREEN_HEIGHT as f32 + 50.0 && e.active);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text(&format!("Score: {}", self.score), 10, 10, 20, Color::WHITE);
        self.player.draw(d);
        for bullet in &self.bullets {
            bullet.draw(d);
        }
        for enemy in &self.enemies {
            enemy.draw(d);
        }
        if self.game_over {
            d.draw_text("GAME OVER", SCREEN_WIDTH / 2 - 100, SCREEN_HEIGHT / 2 - 30, 40, Color::RED);
            d.draw_text("Press R to Restart", SCREEN_WIDTH / 2 - 120, SCREEN_HEIGHT / 2 + 20, 20, Color::WHITE);
            d.draw_text("Press ESC to return to Menu", SCREEN_WIDTH / 2 - 150, SCREEN_HEIGHT / 2 + 60, 20, Color::GRAY);
        }
    }

    #[allow(dead_code)]
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn wants_to_quit(&self) -> bool {
        self.wants_to_quit
    }

    pub fn reset(&mut self) {
        self.player = Player::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        self.bullets.clear();
        self.enemies.clear();
        self.score = 0;
        self.game_over = false;
        self.spawn_timer = 0.0;
        self.last_spawn_time = 0.0;
        self.wants_to_quit = false;
    }
}