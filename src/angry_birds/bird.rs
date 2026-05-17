use raylib::prelude::*;
use rapier2d::prelude::*;
use nalgebra::Vector2;
use crate::angry_birds::physics::{PhysicsWorld, PIXELS_PER_METER};

#[derive(Clone, Debug, PartialEq)]
pub enum BirdType {
    Red,
    Blue,
    Yellow,
    Black,
    White,
}

pub struct Bird {
    pub bird_type: BirdType,
    pub handle: RigidBodyHandle,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub launched: bool,
    pub used: bool,
    pub active: bool,
    pub ability_triggered: bool,
}

impl Bird {
    pub fn new(bird_type: BirdType, handle: RigidBodyHandle, x: f32, y: f32) -> Self {
        Bird {
            bird_type,
            handle,
            x,
            y,
            radius: 15.0,
            launched: false,
            used: false,
            active: true,
            ability_triggered: false,
        }
    }

    pub fn get_color(&self) -> Color {
        match self.bird_type {
            BirdType::Red => Color::RED,
            BirdType::Blue => Color::BLUE,
            BirdType::Yellow => Color::YELLOW,
            BirdType::Black => Color::BLACK,
            BirdType::White => Color::WHITE,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if !self.active || !self.launched {
            return;
        }

        let screen_pos = physics_to_screen_pos(self.x, self.y, 600.0);

        d.draw_circle(screen_pos.0 as i32, screen_pos.1 as i32, self.radius, self.get_color());

        let eye_offset = self.radius * 0.3;
        d.draw_circle(
            (screen_pos.0 - eye_offset) as i32,
            (screen_pos.1 - eye_offset * 0.5) as i32,
            4.0,
            Color::WHITE,
        );
        d.draw_circle(
            (screen_pos.0 + eye_offset) as i32,
            (screen_pos.1 - eye_offset * 0.5) as i32,
            4.0,
            Color::WHITE,
        );

        d.draw_circle(
            (screen_pos.0 - eye_offset - 1.0) as i32,
            (screen_pos.1 - eye_offset * 0.5) as i32,
            2.0,
            Color::BLACK,
        );
        d.draw_circle(
            (screen_pos.0 + eye_offset - 1.0) as i32,
            (screen_pos.1 - eye_offset * 0.5) as i32,
            2.0,
            Color::BLACK,
        );
    }

    pub fn draw_in_sling(&self, d: &mut RaylibDrawHandle, sling_x: f32, sling_y: f32) {
        if self.launched || !self.active {
            return;
        }
        d.draw_circle(sling_x as i32, sling_y as i32, self.radius, self.get_color());

        let eye_offset = self.radius * 0.3;
        d.draw_circle(
            (sling_x - eye_offset) as i32,
            (sling_y - eye_offset * 0.5) as i32,
            4.0,
            Color::WHITE,
        );
        d.draw_circle(
            (sling_x + eye_offset) as i32,
            (sling_y - eye_offset * 0.5) as i32,
            4.0,
            Color::WHITE,
        );
    }

    pub fn update_position(&mut self, physics: &PhysicsWorld, screen_height: f32) {
        if let Some(pos) = physics.get_position(self.handle) {
            self.x = pos.x * PIXELS_PER_METER;
            self.y = screen_height - pos.y * PIXELS_PER_METER;
        }
    }

    pub fn use_ability(&mut self, physics: &mut PhysicsWorld, screen_height: f32) -> Vec<Bird> {
        let mut new_birds = Vec::new();

        if self.ability_triggered || !self.active || !self.launched {
            return new_birds;
        }

        match self.bird_type {
            BirdType::Red => {
                self.ability_triggered = true;
                if let Some(_pos) = physics.get_position(self.handle) {
                    let impulse = Vector2::new(0.0, 5.0);
                    physics.apply_impulse(self.handle, impulse);
                }
            }
            BirdType::Blue => {
                self.ability_triggered = true;
                if let Some(pos) = physics.get_position(self.handle) {
                    for i in 0..3 {
                        let offset = (i as f32 - 1.0) * 30.0;
                        let new_x = pos.x * PIXELS_PER_METER + offset;
                        let new_y = pos.y * PIXELS_PER_METER;
                        let new_handle = physics.create_dynamic_ball(new_x, new_y, self.radius * 0.6, screen_height);
                        let vel = Vector2::new(offset * 0.5, 2.0);
                        physics.set_linear_velocity(new_handle, vel);
                        new_birds.push(Bird::new(BirdType::Blue, new_handle, new_x, new_y));
                    }
                }
            }
            BirdType::Yellow => {
                self.ability_triggered = true;
                if let Some(body) = physics.rigid_body_set.get_mut(self.handle) {
                    let current_vel = body.linvel().clone();
                    let boost = current_vel * 1.8;
                    body.set_linvel(boost, true);
                }
            }
            BirdType::Black => {
                self.ability_triggered = true;
                if let Some(pos) = physics.get_position(self.handle) {
                    for dx in -2..=2 {
                        for dy in -2..=2 {
                            let explosion_impulse = Vector2::new(dx as f32 * 3.0, dy as f32 * 3.0);
                            let target_x = pos.x * PIXELS_PER_METER + dx as f32 * 50.0;
                            let target_y = screen_height - pos.y * PIXELS_PER_METER + dy as f32 * 50.0;

                            let mut closest_body: Option<RigidBodyHandle> = None;
                            let mut closest_dist = f32::MAX;

                            for (handle, body) in physics.rigid_body_set.iter() {
                                let body_pos = body.translation();
                                let dist = ((body_pos.x * PIXELS_PER_METER - target_x).powi(2) +
                                           (screen_height - body_pos.y * PIXELS_PER_METER - target_y).powi(2)).sqrt();
                                if dist < closest_dist && dist < 100.0 {
                                    closest_dist = dist;
                                    closest_body = Some(handle);
                                }
                            }

                            if let Some(target_handle) = closest_body {
                                physics.apply_impulse(target_handle, explosion_impulse);
                            }
                        }
                    }
                }
            }
            BirdType::White => {
                self.ability_triggered = true;
                if let Some(pos) = physics.get_position(self.handle) {
                    for _ in 0..2 {
                        let egg_x = pos.x * PIXELS_PER_METER + (rand_x() - 0.5) * 40.0;
                        let egg_y = pos.y * PIXELS_PER_METER - 30.0;
                        let new_handle = physics.create_dynamic_ball(egg_x, egg_y, 8.0, screen_height);
                        let vel = Vector2::new(0.0, -5.0);
                        physics.set_linear_velocity(new_handle, vel);
                        new_birds.push(Bird::new(BirdType::White, new_handle, egg_x, egg_y));
                    }
                }
            }
        }

        new_birds
    }
}

fn physics_to_screen_pos(x: f32, y: f32, screen_height: f32) -> (f32, f32) {
    (x, screen_height - y)
}

fn rand_x() -> f32 {
    use std::time::SystemTime;
    let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    (seed % 100) as f32 / 100.0
}

pub struct BirdQueue {
    pub birds: Vec<BirdType>,
    pub current_index: usize,
}

impl BirdQueue {
    pub fn new(bird_types: Vec<BirdType>) -> Self {
        BirdQueue {
            birds: bird_types,
            current_index: 0,
        }
    }

    pub fn get_next(&self) -> Option<BirdType> {
        if self.current_index < self.birds.len() {
            Some(self.birds[self.current_index].clone())
        } else {
            None
        }
    }

    pub fn advance(&mut self) {
        if self.current_index < self.birds.len() {
            self.current_index += 1;
        }
    }

    pub fn get_remaining(&self) -> usize {
        self.birds.len().saturating_sub(self.current_index)
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

pub fn draw_bird_preview(bird_type: &BirdType, d: &mut RaylibDrawHandle, x: i32, y: i32, size: f32) {
    let color = match bird_type {
        BirdType::Red => Color::RED,
        BirdType::Blue => Color::BLUE,
        BirdType::Yellow => Color::YELLOW,
        BirdType::Black => Color::BLACK,
        BirdType::White => Color::WHITE,
    };

    d.draw_circle(x, y, size, color);
    d.draw_circle(x - (size * 0.3) as i32, y - (size * 0.2) as i32, size * 0.25, Color::WHITE);
    d.draw_circle(x + (size * 0.3) as i32, y - (size * 0.2) as i32, size * 0.25, Color::WHITE);
}