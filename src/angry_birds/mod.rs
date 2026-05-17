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
use level::{LevelManager, LevelData};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const SLINGSHOT_X: f32 = 150.0;
const SLINGSHOT_Y: f32 = 450.0;

pub fn run() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Angry Birds")
        .build();

    rl.set_target_fps(60);

    let mut physics = PhysicsWorld::new();
    physics.create_ground(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let mut level_manager = LevelManager::new();
    let mut bird_queue: Option<BirdQueue> = None;
    let mut birds: Vec<Bird> = Vec::new();
    let mut blocks: Vec<Block> = Vec::new();
    let mut pigs: Vec<Pig> = Vec::new();
    let mut slingshot = Slingshot::new(SLINGSHOT_X, SLINGSHOT_Y);
    let mut current_bird_handle: Option<RigidBodyHandle> = None;
    let mut game_state = GameState::Aiming;
    let mut score: i32 = 0;
    let mut total_birds_used: i32 = 0;
    let mut waiting_for_settle = false;
    let mut settle_timer: f32 = 0.0;
    let mut all_pigs_destroyed = false;
    let mut level_complete = false;
    let mut game_over = false;
    let mut trajectory_points: Vec<(f32, f32)> = Vec::new();

    load_level(
        &mut physics,
        level_manager.get_current_level(),
        &mut bird_queue,
        &mut birds,
        &mut blocks,
        &mut pigs,
    );

    while !rl.window_should_close() {
        let should_restart = rl.is_key_pressed(KeyboardKey::KEY_R);
        let should_next_level = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
        let mouse_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        let mouse_released = rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
        let mouse_x = rl.get_mouse_x() as f32;
        let mouse_y = rl.get_mouse_y() as f32;

        if should_restart && (game_over || level_complete) {
            restart_game(
                &mut physics,
                &mut level_manager,
                &mut bird_queue,
                &mut birds,
                &mut blocks,
                &mut pigs,
                &mut slingshot,
                &mut current_bird_handle,
                &mut score,
                &mut total_birds_used,
                &mut waiting_for_settle,
                &mut settle_timer,
                &mut all_pigs_destroyed,
                &mut level_complete,
                &mut game_over,
            );
        }

        if game_over {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_text("GAME OVER", SCREEN_WIDTH / 2 - 100, SCREEN_HEIGHT / 2 - 50, 40, Color::RED);
            d.draw_text(&format!("Final Score: {}", score), SCREEN_WIDTH / 2 - 80, SCREEN_HEIGHT / 2, 30, Color::WHITE);
            d.draw_text("Press R to Restart", SCREEN_WIDTH / 2 - 100, SCREEN_HEIGHT / 2 + 50, 20, Color::GRAY);
            continue;
        }

        if level_complete {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(50, 100, 50, 255));

            let stars = calculate_stars(total_birds_used, level_manager.get_current_level().birds.len() as i32);
            d.draw_text(&format!("Level {} Complete!", level_manager.get_level_number()), SCREEN_WIDTH / 2 - 120, 150, 40, Color::WHITE);
            d.draw_text(&format!("Score: {}", score), SCREEN_WIDTH / 2 - 60, 220, 30, Color::WHITE);

            let star_color = Color::YELLOW;
            for i in 0..3 {
                let x = SCREEN_WIDTH / 2 - 60 + i * 50;
                if i < stars {
                    d.draw_text("★", x, 280, 50, star_color);
                } else {
                    d.draw_text("☆", x, 280, 50, Color::GRAY);
                }
            }

            if level_manager.get_level_number() >= level_manager.levels.len() {
                d.draw_text("You Win! All Levels Complete!", SCREEN_WIDTH / 2 - 180, 380, 25, Color::GOLD);
            } else {
                d.draw_text("Press ENTER for Next Level", SCREEN_WIDTH / 2 - 150, 380, 25, Color::WHITE);
            }
            d.draw_text("Press R to Restart", SCREEN_WIDTH / 2 - 100, 430, 20, Color::GRAY);

            if should_next_level && level_manager.get_level_number() < level_manager.levels.len() {
                level_manager.next_level();
                load_level(
                    &mut physics,
                    level_manager.get_current_level(),
                    &mut bird_queue,
                    &mut birds,
                    &mut blocks,
                    &mut pigs,
                );
                slingshot = Slingshot::new(SLINGSHOT_X, SLINGSHOT_Y);
                current_bird_handle = None;
                game_state = GameState::Aiming;
                waiting_for_settle = false;
                settle_timer = 0.0;
                all_pigs_destroyed = false;
                level_complete = false;
                total_birds_used = 0;
            }
            continue;
        }

        match game_state {
            GameState::Aiming => {
                if mouse_down {
                    let dx = slingshot.base_x - mouse_x;
                    let dy = slingshot.base_y - mouse_y;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < 150.0 {
                        slingshot.update_pull(mouse_x, mouse_y);

                        if let Some(bird_type) = bird_queue.as_ref().and_then(|q| q.get_next()) {
                            if current_bird_handle.is_none() {
                                let handle = physics.create_dynamic_ball(
                                    slingshot.pull_x,
                                    slingshot.pull_y,
                                    15.0 * 2.0,
                                    SCREEN_HEIGHT as f32,
                                );
                                if let Some(body) = physics.rigid_body_set.get_mut(handle) {
                                    body.set_body_type(RigidBodyType::Fixed, true);
                                }
                                current_bird_handle = Some(handle);
                                birds.push(Bird::new(bird_type.clone(), handle, slingshot.pull_x, slingshot.pull_y));
                            } else if let Some(handle) = current_bird_handle {
                                let pos = physics.to_physics_pos(slingshot.pull_x, slingshot.pull_y, SCREEN_HEIGHT as f32);
                                if let Some(body) = physics.rigid_body_set.get_mut(handle) {
                                    body.set_translation(pos, true);
                                }
                                if let Some(bird) = birds.iter_mut().last() {
                                    bird.x = slingshot.pull_x;
                                    bird.y = slingshot.pull_y;
                                }
                            }

                            let dx = slingshot.base_x - slingshot.pull_x;
                            let dy = slingshot.base_y - slingshot.pull_y;
                            let dist = (dx * dx + dy * dy).sqrt();
                            if dist > 5.0 {
                                let power = dist / slingshot.max_pull_distance * 30.0;
                                let vel = NalgebraVector2::new(
                                    dx / dist * power,
                                    -dy / dist * power,
                                );
                                trajectory_points = calculate_trajectory(slingshot.base_x, slingshot.base_y, vel, 30);
                            } else {
                                trajectory_points.clear();
                            }
                        }
                    }
                }

                if mouse_released {
                    if let Some(handle) = current_bird_handle {
                        if let Some(velocity) = slingshot.release() {
                            physics.wake_up(handle);
                            launch_bird(&mut physics, handle, velocity, SCREEN_HEIGHT as f32);

                            if let Some(bird) = birds.iter_mut().last() {
                                bird.launched = true;
                            }
                            if let Some(queue) = bird_queue.as_mut() {
                                queue.advance();
                            }
                            total_birds_used += 1;
                            current_bird_handle = None;
                            game_state = GameState::Flying;
                            waiting_for_settle = false;
                            trajectory_points.clear();
                        }
                    }
                }

                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    if let Some(idx) = birds.iter().position(|b| b.launched && !b.ability_triggered && b.active) {
                        if let Some(bird) = birds.get_mut(idx) {
                            let new_birds = bird.use_ability(&mut physics, SCREEN_HEIGHT as f32);
                            birds.extend(new_birds);
                        }
                    }
                }
            }
            GameState::Flying => {
                if waiting_for_settle {
                    settle_timer += 1.0 / 60.0;
                    if settle_timer > 2.0 {
                        check_game_state(
                            &birds,
                            &blocks,
                            &pigs,
                            &mut all_pigs_destroyed,
                            &mut level_complete,
                            &mut game_over,
                            &mut score,
                        );

                        if level_complete || game_over {
                            continue;
                        }

                        spawn_next_bird(
                            &mut bird_queue,
                            &mut slingshot,
                            &mut current_bird_handle,
                        );

                        if current_bird_handle.is_some() {
                            game_state = GameState::Aiming;
                            waiting_for_settle = false;
                            settle_timer = 0.0;
                        } else {
                            game_over = true;
                        }
                    }
                } else {
                    let all_settled = birds.iter()
                        .filter(|b| b.launched && b.active)
                        .all(|b| physics.is_sleeping(b.handle));

                    if all_settled {
                        let all_birds_done = birds.iter()
                            .filter(|b| b.launched && b.active)
                            .all(|b| {
                                let pos = physics.get_position(b.handle);
                                pos.map(|p| {
                                    let screen_y = SCREEN_HEIGHT as f32 - p.y * 50.0;
                                    let screen_x = p.x * 50.0;
                                    let vel = physics.rigid_body_set.get(b.handle)
                                        .map(|body| body.linvel().clone())
                                        .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                                    let speed = (vel.x * vel.x + vel.y * vel.y).sqrt();
                                    screen_y > 550.0 || screen_y < 50.0 || screen_x < 50.0 || screen_x > 750.0 || speed < 0.5
                                }).unwrap_or(false)
                            });

                        if all_birds_done || birds.iter().filter(|b| b.launched && b.active).count() == 0 {
                            waiting_for_settle = true;
                        }
                    }

                    let all_out_of_bounds = birds.iter()
                        .filter(|b| b.launched && b.active)
                        .all(|b| {
                            let pos = physics.get_position(b.handle);
                            pos.map(|p| {
                                let screen_y = SCREEN_HEIGHT as f32 - p.y * 50.0;
                                let screen_x = p.x * 50.0;
                                screen_y > 580.0 || screen_y < -50.0 || screen_x < -50.0 || screen_x > 850.0
                            }).unwrap_or(false)
                        });

                    if all_out_of_bounds && birds.iter().any(|b| b.launched && b.active) {
                        waiting_for_settle = true;
                    }
                }
            }
        }

        physics.step();

        for bird in &mut birds {
            if bird.launched && bird.active {
                bird.update_position(&physics, SCREEN_HEIGHT as f32);
            }
        }

        for block in &mut blocks {
            block.update_position(&physics, SCREEN_HEIGHT as f32);
        }

        for pig in &mut pigs {
            pig.update_position(&physics, SCREEN_HEIGHT as f32);
        }

        for i in 0..birds.len() {
            if !birds[i].launched || !birds[i].active {
                continue;
            }
            let bird_handle = birds[i].handle;

            if let Some(bpos) = physics.get_position(bird_handle) {
                for j in 0..blocks.len() {
                    if blocks[j].destroyed {
                        continue;
                    }
                    let block_handle = blocks[j].handle;

                    if check_collision(&physics, bird_handle, block_handle) {
                        let vel = physics.rigid_body_set.get(bird_handle)
                            .map(|b| b.linvel().clone())
                            .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                        let impact_speed = (vel.x * vel.x + vel.y * vel.y).sqrt() * 50.0;
                        blocks[j].apply_force_from_impact(impact_speed);

                        if blocks[j].destroyed {
                            physics.remove_body(blocks[j].handle);
                            score += 10;
                        }
                    }
                }

                for j in 0..pigs.len() {
                    if pigs[j].destroyed {
                        continue;
                    }
                    let pig_handle = pigs[j].handle;

                    if check_collision(&physics, bird_handle, pig_handle) {
                        let vel = physics.rigid_body_set.get(bird_handle)
                            .map(|b| b.linvel().clone())
                            .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                        let impact_speed = (vel.x * vel.x + vel.y * vel.y).sqrt() * 50.0;
                        pigs[j].apply_force_from_impact(impact_speed);

                        if pigs[j].destroyed {
                            physics.remove_body(pigs[j].handle);
                            score += 50;
                        }
                    }
                }
            }
        }

        for i in 0..blocks.len() {
            if blocks[i].destroyed {
                continue;
            }
            let block_handle = blocks[i].handle;

            if let Some(bpos) = physics.get_position(block_handle) {
                for j in 0..pigs.len() {
                    if pigs[j].destroyed {
                        continue;
                    }
                    let pig_handle = pigs[j].handle;

                    if check_collision(&physics, block_handle, pig_handle) {
                        let vel = physics.rigid_body_set.get(block_handle)
                            .map(|b| b.linvel().clone())
                            .unwrap_or(NalgebraVector2::new(0.0, 0.0));
                        let impact_speed = (vel.x * vel.x + vel.y * vel.y).sqrt() * 50.0;

                        if impact_speed > 3.0 {
                            pigs[j].apply_force_from_impact(impact_speed * 0.5);

                            if pigs[j].destroyed {
                                physics.remove_body(pigs[j].handle);
                                score += 50;
                            }
                        }
                    }
                }
            }
        }

        blocks.retain(|b| !b.destroyed);
        pigs.retain(|p| !p.destroyed);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(135, 206, 235, 255));

        d.draw_rectangle(0, SCREEN_HEIGHT - 50, SCREEN_WIDTH, 50, Color::new(139, 69, 19, 255));

        d.draw_text(&format!("Level {}", level_manager.get_level_number()), 10, 10, 20, Color::BLACK);
        d.draw_text(&format!("Score: {}", score), 10, 35, 20, Color::BLACK);

        if let Some(queue) = &bird_queue {
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

        slingshot.draw(&mut d);

        for point in &trajectory_points {
            let screen_y = 600.0 - point.1;
            if screen_y > 0.0 && screen_y < 600.0 && point.0 > 0.0 && point.0 < 800.0 {
                d.draw_circle(point.0 as i32, screen_y as i32, 3.0, Color::new(255, 255, 0, 128));
            }
        }

        for bird in &birds {
            if !bird.launched && bird.active {
                bird.draw_in_sling(&mut d, slingshot.pull_x, slingshot.pull_y);
            } else {
                bird.draw(&mut d);
            }
        }

        for block in &blocks {
            block.draw(&mut d);
        }

        for pig in &pigs {
            pig.draw(&mut d);
        }

        if game_state == GameState::Aiming && current_bird_handle.is_some() {
            if let Some(bird) = birds.iter().find(|b| b.handle == current_bird_handle.unwrap()) {
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

        let remaining_pigs = pigs.iter().filter(|p| !p.destroyed).count();
        d.draw_text(&format!("Pigs: {}", remaining_pigs), SCREEN_WIDTH - 80, 10, 20, Color::RED);
    }
}

#[derive(PartialEq)]
enum GameState {
    Aiming,
    Flying,
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

fn check_game_state(
    birds: &[Bird],
    blocks: &[Block],
    pigs: &[Pig],
    all_pigs_destroyed: &mut bool,
    level_complete: &mut bool,
    game_over: &mut bool,
    score: &mut i32,
) {
    *all_pigs_destroyed = pigs.iter().all(|p| p.destroyed);

    if *all_pigs_destroyed {
        *level_complete = true;
    } else {
        let has_active_bird = birds.iter().any(|b| b.launched && b.active && !b.ability_triggered);
        if !has_active_bird {
            let remaining_pigs = pigs.iter().filter(|p| !p.destroyed).count();
            if remaining_pigs > 0 {
                *game_over = true;
            }
        }
    }
}

fn spawn_next_bird(
    bird_queue: &mut Option<BirdQueue>,
    slingshot: &mut Slingshot,
    current_bird_handle: &mut Option<RigidBodyHandle>,
) {
    if let Some(queue) = bird_queue {
        if queue.get_next().is_some() {
            slingshot.pull_x = slingshot.base_x;
            slingshot.pull_y = slingshot.base_y;
            return;
        }
    }
    *current_bird_handle = None;
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

fn load_level(
    physics: &mut PhysicsWorld,
    level_data: &LevelData,
    bird_queue: &mut Option<BirdQueue>,
    birds: &mut Vec<Bird>,
    blocks: &mut Vec<Block>,
    pigs: &mut Vec<Pig>,
) {
    birds.clear();
    blocks.clear();
    pigs.clear();

    *bird_queue = Some(BirdQueue::new(level_data.birds.clone()));

    for block_data in &level_data.blocks {
        let block = BlockFactory::create_block(
            block_data.block_type.clone(),
            physics,
            block_data.x,
            block_data.y,
            block_data.width,
            block_data.height,
            SCREEN_HEIGHT as f32,
        );
        blocks.push(block);
    }

    for pig_data in &level_data.pigs {
        let pig = PigFactory::create_pig(
            pig_data.pig_size.clone(),
            physics,
            pig_data.x,
            pig_data.y,
            SCREEN_HEIGHT as f32,
        );
        pigs.push(pig);
    }
}

fn restart_game(
    physics: &mut PhysicsWorld,
    level_manager: &mut LevelManager,
    bird_queue: &mut Option<BirdQueue>,
    birds: &mut Vec<Bird>,
    blocks: &mut Vec<Block>,
    pigs: &mut Vec<Pig>,
    slingshot: &mut Slingshot,
    current_bird_handle: &mut Option<RigidBodyHandle>,
    score: &mut i32,
    total_birds_used: &mut i32,
    waiting_for_settle: &mut bool,
    settle_timer: &mut f32,
    all_pigs_destroyed: &mut bool,
    level_complete: &mut bool,
    game_over: &mut bool,
) {
    *physics = PhysicsWorld::new();
    physics.create_ground(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    level_manager.reset();
    *bird_queue = None;
    birds.clear();
    blocks.clear();
    pigs.clear();
    *slingshot = Slingshot::new(SLINGSHOT_X, SLINGSHOT_Y);
    *current_bird_handle = None;
    *score = 0;
    *total_birds_used = 0;
    *waiting_for_settle = false;
    *settle_timer = 0.0;
    *all_pigs_destroyed = false;
    *level_complete = false;
    *game_over = false;

    load_level(
        physics,
        level_manager.get_current_level(),
        bird_queue,
        birds,
        blocks,
        pigs,
    );
}