pub mod physics;
pub mod bird;
pub mod block;
pub mod pig;
pub mod slingshot;
pub mod level;

use raylib::prelude::*;
use rapier2d::prelude::*;
use nalgebra::Vector2 as NalgebraVector2;

use physics::PhysicsWorld;
use bird::{Bird, BirdType, BirdQueue};
use block::{Block, BlockFactory};
use pig::{Pig, PigFactory};
use slingshot::{Slingshot, launch_bird, calculate_trajectory};
use level::LevelManager;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const SLINGSHOT_X: f32 = 150.0;
const SLINGSHOT_Y: f32 = 450.0;

pub struct AngryBirdsGame {
    physics: PhysicsWorld,
    level_manager: LevelManager,
    bird_queue: Option<BirdQueue>,
    birds: Vec<Bird>,
    blocks: Vec<Block>,
    pigs: Vec<Pig>,
    slingshot: Slingshot,
    current_bird_handle: Option<RigidBodyHandle>,
    game_state: InternalGameState,
    score: i32,
    total_birds_used: i32,
    waiting_for_settle: bool,
    settle_timer: f32,
    all_pigs_destroyed: bool,
    level_complete: bool,
    game_over: bool,
    trajectory_points: Vec<(f32, f32)>,
    wants_to_quit: bool,
}

#[derive(PartialEq)]
enum InternalGameState {
    Aiming,
    Flying,
}

impl AngryBirdsGame {
    pub fn new() -> Self {
        let mut physics = PhysicsWorld::new();
        physics.create_ground(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

        let mut game = AngryBirdsGame {
            physics,
            level_manager: LevelManager::new(),
            bird_queue: None,
            birds: Vec::new(),
            blocks: Vec::new(),
            pigs: Vec::new(),
            slingshot: Slingshot::new(SLINGSHOT_X, SLINGSHOT_Y),
            current_bird_handle: None,
            game_state: InternalGameState::Aiming,
            score: 0,
            total_birds_used: 0,
            waiting_for_settle: false,
            settle_timer: 0.0,
            all_pigs_destroyed: false,
            level_complete: false,
            game_over: false,
            trajectory_points: Vec::new(),
            wants_to_quit: false,
        };

        game.load_current_level();
        game
    }

    fn load_current_level(&mut self) {
        self.birds.clear();
        self.blocks.clear();
        self.pigs.clear();

        self.bird_queue = Some(BirdQueue::new(self.level_manager.get_current_level().birds.clone()));

        for block_data in &self.level_manager.get_current_level().blocks {
            let block = BlockFactory::create_block(
                block_data.block_type.clone(),
                &mut self.physics,
                block_data.x,
                block_data.y,
                block_data.width,
                block_data.height,
                SCREEN_HEIGHT as f32,
            );
            self.blocks.push(block);
        }

        for pig_data in &self.level_manager.get_current_level().pigs {
            let pig = PigFactory::create_pig(
                pig_data.pig_size.clone(),
                &mut self.physics,
                pig_data.x,
                pig_data.y,
                SCREEN_HEIGHT as f32,
            );
            self.pigs.push(pig);
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.wants_to_quit = true;
            return;
        }

        let should_restart = rl.is_key_pressed(KeyboardKey::KEY_R);
        let should_next_level = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let mouse_released = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
        let mouse_x = rl.get_mouse_x() as f32;
        let mouse_y = rl.get_mouse_y() as f32;

        if should_restart && (self.game_over || self.level_complete) {
            self.restart();
            return;
        }

        if self.game_over {
            return;
        }

        if self.level_complete {
            if should_next_level && self.level_manager.get_level_number() < self.level_manager.levels.len() {
                self.level_manager.next_level();
                self.load_current_level();
                self.slingshot = Slingshot::new(SLINGSHOT_X, SLINGSHOT_Y);
                self.current_bird_handle = None;
                self.game_state = InternalGameState::Aiming;
                self.waiting_for_settle = false;
                self.settle_timer = 0.0;
                self.all_pigs_destroyed = false;
                self.level_complete = false;
                self.total_birds_used = 0;
            }
            return;
        }

        match self.game_state {
            InternalGameState::Aiming => {
                if mouse_down {
                    let dx = self.slingshot.base_x - mouse_x;
                    let dy = self.slingshot.base_y - mouse_y;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < 150.0 {
                        self.slingshot.update_pull(mouse_x, mouse_y);

                        if let Some(bird_type) = self.bird_queue.as_ref().and_then(|q| q.get_next()) {
                            if self.current_bird_handle.is_none() {
                                let handle = self.physics.create_dynamic_ball(
                                    self.slingshot.pull_x,
                                    self.slingshot.pull_y,
                                    15.0 * 2.0,
                                    SCREEN_HEIGHT as f32,
                                );
                                if let Some(body) = self.physics.rigid_body_set.get_mut(handle) {
                                    body.set_body_type(RigidBodyType::Fixed, true);
                                }
                                self.current_bird_handle = Some(handle);
                                self.birds.push(Bird::new(bird_type.clone(), handle, self.slingshot.pull_x, self.slingshot.pull_y));
                            } else if let Some(handle) = self.current_bird_handle {
                                let pos = self.physics.to_physics_pos(self.slingshot.pull_x, self.slingshot.pull_y, SCREEN_HEIGHT as f32);
                                if let Some(body) = self.physics.rigid_body_set.get_mut(handle) {
                                    body.set_translation(pos, true);
                                }
                                if let Some(bird) = self.birds.iter_mut().last() {
                                    bird.x = self.slingshot.pull_x;
                                    bird.y = self.slingshot.pull_y;
                                }
                            }

                            let dx = self.slingshot.base_x - self.slingshot.pull_x;
                            let dy = self.slingshot.base_y - self.slingshot.pull_y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            if dist > 5.0 {
                                let power = dist / self.slingshot.max_pull_distance * 30.0;
                                let vel = NalgebraVector2::new(
                                    dx / dist * power,
                                    -dy / dist * power,
                                );
                                self.trajectory_points = calculate_trajectory(self.slingshot.base_x, self.slingshot.base_y, vel, 30);
                            } else {
                                self.trajectory_points.clear();
                            }
                        }
                    }
                }

                if mouse_released {
                    if let Some(handle) = self.current_bird_handle {
                        if let Some(velocity) = self.slingshot.release() {
                            self.physics.wake_up(handle);
                            launch_bird(&mut self.physics, handle, velocity, SCREEN_HEIGHT as f32);

                            if let Some(bird) = self.birds.iter_mut().last() {
                                bird.launched = true;
                            }
                            if let Some(queue) = self.bird_queue.as_mut() {
                                queue.advance();
                            }
                            self.total_birds_used += 1;
                            self.current_bird_handle = None;
                            self.game_state = InternalGameState::Flying;
                            self.waiting_for_settle = false;
                            self.trajectory_points.clear();
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    if let Some(idx) = self.birds.iter().position(|b| b.launched && !b.ability_triggered && b.active) {
                        if let Some(bird) = self.birds.get_mut(idx) {
                            let new_birds = bird.use_ability(&mut self.physics, SCREEN_HEIGHT as f32);
                            self.birds.extend(new_birds);
                        }
                    }
                }
            }
            InternalGameState::Flying => {
                if self.waiting_for_settle {
                    self.settle_timer += 1.0 / 60.0;
                    if self.settle_timer > 2.0 {
                        self.check_game_state();

                        if self.level_complete || self.game_over {
                            return;
                        }

                        self.spawn_next_bird();

                        if self.current_bird_handle.is_some() {
                            self.game_state = InternalGameState::Aiming;
                            self.waiting_for_settle = false;
                            self.settle_timer = 0.0;
                        } else {
                            self.game_over = true;
                        }
                    }
                } else {
                    let all_settled = self.birds.iter()
                        .filter(|b| b.launched && b.active)
                        .all(|b| self.physics.is_sleeping(b.handle));

                    if all_settled {
                        let all_birds_done = self.birds.iter()
                            .filter(|b| b.launched && b.active)
                            .all(|b| {
                                let pos = self.physics.get_position(b.handle);
                                pos.map(|p| {
                                    let screen_y = SCREEN_HEIGHT as f32 - p.y * 50.0;
                                    let screen_x = p.x * 50.0;
                                    let vel = self.physics.rigid_body_set.get(b.handle)
                                        .map(|body| *body.linvel())
                                        .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                                    let speed = (vel.x * vel.x + vel.y * vel.y).sqrt();
                                    !(50.0..=550.0).contains(&screen_y) || !(50.0..=750.0).contains(&screen_x) || speed < 0.5
                                }).unwrap_or(false)
                            });

                        if all_birds_done || self.birds.iter().filter(|b| b.launched && b.active).count() == 0 {
                            self.waiting_for_settle = true;
                        }
                    }

                    let all_out_of_bounds = self.birds.iter()
                        .filter(|b| b.launched && b.active)
                        .all(|b| {
                            let pos = self.physics.get_position(b.handle);
                            pos.map(|p| {
                                let screen_y = SCREEN_HEIGHT as f32 - p.y * 50.0;
                                let screen_x = p.x * 50.0;
                                !(-50.0..=580.0).contains(&screen_y) || !(-50.0..=850.0).contains(&screen_x)
                            }).unwrap_or(false)
                        });

                    if all_out_of_bounds && self.birds.iter().any(|b| b.launched && b.active) {
                        self.waiting_for_settle = true;
                    }
                }
            }
        }

        self.physics.step();

        for bird in &mut self.birds {
            if bird.launched && bird.active {
                bird.update_position(&self.physics, SCREEN_HEIGHT as f32);
            }
        }

        for block in &mut self.blocks {
            block.update_position(&self.physics, SCREEN_HEIGHT as f32);
        }

        for pig in &mut self.pigs {
            pig.update_position(&self.physics, SCREEN_HEIGHT as f32);
        }

        self.handle_collisions();

        self.blocks.retain(|b| !b.destroyed);
        self.pigs.retain(|p| !p.destroyed);
    }

    fn handle_collisions(&mut self) {
        for bird in self.birds.iter_mut() {
            if !bird.launched || !bird.active {
                continue;
            }
            let bird_handle = bird.handle;

            if let Some(_bpos) = self.physics.get_position(bird_handle) {
                for block in self.blocks.iter_mut() {
                    if block.destroyed {
                        continue;
                    }
                    let block_handle = block.handle;

                    if Self::check_collision(&self.physics, bird_handle, block_handle) {
                        let vel = self.physics.rigid_body_set.get(bird_handle)
                            .map(|b| *b.linvel())
                            .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                        let impact_speed = (vel.x * vel.x + vel.y * vel.y).sqrt() * 50.0;
                        block.apply_force_from_impact(impact_speed);

                        if block.destroyed {
                            self.physics.remove_body(block.handle);
                            self.score += 10;
                        }
                    }
                }

                for pig in self.pigs.iter_mut() {
                    if pig.destroyed {
                        continue;
                    }
                    let pig_handle = pig.handle;

                    if Self::check_collision(&self.physics, bird_handle, pig_handle) {
                        let vel = self.physics.rigid_body_set.get(bird_handle)
                            .map(|b| *b.linvel())
                            .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                        let impact_speed = (vel.x * vel.x + vel.y * vel.y).sqrt() * 50.0;
                        pig.apply_force_from_impact(impact_speed);

                        if pig.destroyed {
                            self.physics.remove_body(pig.handle);
                            self.score += 50;
                        }
                    }
                }
            }
        }

        for block in self.blocks.iter_mut() {
            if block.destroyed {
                continue;
            }
            let block_handle = block.handle;

            if let Some(_bpos) = self.physics.get_position(block_handle) {
                for pig in self.pigs.iter_mut() {
                    if pig.destroyed {
                        continue;
                    }
                    let pig_handle = pig.handle;

                    if Self::check_collision(&self.physics, block_handle, pig_handle) {
                        let vel = self.physics.rigid_body_set.get(block_handle)
                            .map(|b| *b.linvel())
                            .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                        let impact_speed = (vel.x * vel.x + vel.y * vel.y).sqrt() * 50.0;

                        if impact_speed > 3.0 {
                            pig.apply_force_from_impact(impact_speed * 0.5);

                            if pig.destroyed {
                                self.physics.remove_body(pig.handle);
                                self.score += 50;
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_collision(physics: &PhysicsWorld, handle1: RigidBodyHandle, handle2: RigidBodyHandle) -> bool {
        let pos1 = physics.get_position(handle1);
        let pos2 = physics.get_position(handle2);

        if let (Some(p1), Some(p2)) = (pos1, pos2) {
            let dist = ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt();
            dist < 1.0
        } else {
            false
        }
    }

    fn check_game_state(&mut self) {
        self.all_pigs_destroyed = self.pigs.iter().all(|p| p.destroyed);

        if self.all_pigs_destroyed {
            self.level_complete = true;
        } else {
            let has_active_bird = self.birds.iter().any(|b| b.launched && b.active && !b.ability_triggered);
            if !has_active_bird {
                let remaining_pigs = self.pigs.iter().filter(|p| !p.destroyed).count();
                if remaining_pigs > 0 {
                    self.game_over = true;
                }
            }
        }
    }

    fn spawn_next_bird(&mut self) {
        if let Some(queue) = &self.bird_queue {
            if queue.get_next().is_some() {
                self.slingshot.pull_x = self.slingshot.base_x;
                self.slingshot.pull_y = self.slingshot.base_y;
                return;
            }
        }
        self.current_bird_handle = None;
    }

    fn restart(&mut self) {
        self.physics = PhysicsWorld::new();
        self.physics.create_ground(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

        self.level_manager.reset();
        self.bird_queue = None;
        self.birds.clear();
        self.blocks.clear();
        self.pigs.clear();
        self.slingshot = Slingshot::new(SLINGSHOT_X, SLINGSHOT_Y);
        self.current_bird_handle = None;
        self.score = 0;
        self.total_birds_used = 0;
        self.waiting_for_settle = false;
        self.settle_timer = 0.0;
        self.all_pigs_destroyed = false;
        self.level_complete = false;
        self.game_over = false;
        self.wants_to_quit = false;
        self.game_state = InternalGameState::Aiming;

        self.load_current_level();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.game_over {
            d.clear_background(Color::BLACK);
            d.draw_text("GAME OVER", SCREEN_WIDTH / 2 - 100, SCREEN_HEIGHT / 2 - 50, 40, Color::RED);
            d.draw_text(&format!("Final Score: {}", self.score), SCREEN_WIDTH / 2 - 80, SCREEN_HEIGHT / 2, 30, Color::WHITE);
            d.draw_text("Press R to Restart", SCREEN_WIDTH / 2 - 100, SCREEN_HEIGHT / 2 + 50, 20, Color::GRAY);
            d.draw_text("Press ESC to return to Menu", SCREEN_WIDTH / 2 - 150, SCREEN_HEIGHT / 2 + 90, 20, Color::GRAY);
            return;
        }

        if self.level_complete {
            d.clear_background(Color::new(50, 100, 50, 255));

            let stars = calculate_stars(self.total_birds_used, self.level_manager.get_current_level().birds.len() as i32);
            d.draw_text(&format!("Level {} Complete!", self.level_manager.get_level_number()), SCREEN_WIDTH / 2 - 120, 150, 40, Color::WHITE);
            d.draw_text(&format!("Score: {}", self.score), SCREEN_WIDTH / 2 - 60, 220, 30, Color::WHITE);

            let star_color = Color::YELLOW;
            for i in 0..3 {
                let x = SCREEN_WIDTH / 2 - 60 + i * 50;
                if i < stars {
                    d.draw_text("★", x, 280, 50, star_color);
                } else {
                    d.draw_text("☆", x, 280, 50, Color::GRAY);
                }
            }

            if self.level_manager.get_level_number() >= self.level_manager.levels.len() {
                d.draw_text("You Win! All Levels Complete!", SCREEN_WIDTH / 2 - 180, 380, 25, Color::GOLD);
            } else {
                d.draw_text("Press ENTER for Next Level", SCREEN_WIDTH / 2 - 150, 380, 25, Color::WHITE);
            }
            d.draw_text("Press R to Restart", SCREEN_WIDTH / 2 - 100, 430, 20, Color::GRAY);
            d.draw_text("Press ESC to return to Menu", SCREEN_WIDTH / 2 - 150, 470, 20, Color::GRAY);
            return;
        }

        d.clear_background(Color::new(135, 206, 235, 255));

        d.draw_rectangle(0, SCREEN_HEIGHT - 50, SCREEN_WIDTH, 50, Color::new(139, 69, 19, 255));

        d.draw_text(&format!("Level {}", self.level_manager.get_level_number()), 10, 10, 20, Color::BLACK);
        d.draw_text(&format!("Score: {}", self.score), 10, 35, 20, Color::BLACK);

        if let Some(queue) = &self.bird_queue {
            let remaining = queue.get_remaining();
            d.draw_text(&format!("Birds: {}", remaining), 10, 60, 20, Color::BLACK);

            for i in 0..remaining {
                let bird_type = &queue.birds[queue.current_index + i];
                let color = match bird_type {
                    BirdType::Red => Color::RED,
                    BirdType::Blue => Color::BLUE,
                    BirdType::Yellow => Color::YELLOW,
                    BirdType::Black => Color::BLACK,
                    BirdType::White => Color::WHITE,
                };
                d.draw_circle(100 + i as i32 * 30, 70, 10.0, color);
            }
        }

        self.slingshot.draw(d);

        for point in &self.trajectory_points {
            let screen_y = 600.0 - point.1;
            if screen_y > 0.0 && screen_y < 600.0 && point.0 > 0.0 && point.0 < 800.0 {
                d.draw_circle(point.0 as i32, screen_y as i32, 3.0, Color::new(255, 255, 0, 128));
            }
        }

        for bird in &self.birds {
            if !bird.launched && bird.active {
                bird.draw_in_sling(d, self.slingshot.pull_x, self.slingshot.pull_y);
            } else {
                bird.draw(d);
            }
        }

        for block in &self.blocks {
            block.draw(d);
        }

        for pig in &self.pigs {
            pig.draw(d);
        }

        if self.game_state == InternalGameState::Aiming && self.current_bird_handle.is_some() {
            if let Some(bird) = self.birds.iter().find(|b| b.handle == self.current_bird_handle.unwrap()) {
                let hint = match bird.bird_type {
                    BirdType::Red => "SPACE: Boost",
                    BirdType::Blue => "SPACE: Split",
                    BirdType::Yellow => "SPACE: Speed",
                    BirdType::Black => "SPACE: Bomb",
                    BirdType::White => "SPACE: Drop Egg",
                };
                d.draw_text(hint, SCREEN_WIDTH - 150, 10, 15, Color::GRAY);
            }
        }

        let remaining_pigs = self.pigs.iter().filter(|p| !p.destroyed).count();
        d.draw_text(&format!("Pigs: {}", remaining_pigs), SCREEN_WIDTH - 80, 10, 20, Color::RED);
    }

    #[allow(dead_code)]
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn wants_to_quit(&self) -> bool {
        self.wants_to_quit
    }
}

fn calculate_stars(birds_used: i32, total_birds: i32) -> i32 {
    let birds_saved = total_birds - birds_used;
    if birds_saved >= total_birds - 2 {
        3
    } else if birds_saved >= total_birds - 3 {
        2
    } else {
        1
    }
}