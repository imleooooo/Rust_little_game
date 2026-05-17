use raylib::prelude::*;
use rapier2d::prelude::*;
use crate::angry_birds::physics::{PhysicsWorld, PIXELS_PER_METER};

#[derive(Clone, Debug, PartialEq)]
pub enum PigSize {
    Small,
    Big,
}

pub struct Pig {
    pub pig_size: PigSize,
    pub handle: RigidBodyHandle,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub health: f32,
    pub max_health: f32,
    pub destroyed: bool,
}

impl Pig {
    pub fn new(pig_size: PigSize, handle: RigidBodyHandle, x: f32, y: f32) -> Self {
        let (radius, health, max_health) = match pig_size {
            PigSize::Small => (18.0, 40.0, 40.0),
            PigSize::Big => (28.0, 80.0, 80.0),
        };

        Pig {
            pig_size,
            handle,
            x,
            y,
            radius,
            health,
            max_health,
            destroyed: false,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.destroyed {
            return;
        }

        let screen_pos = physics_to_screen_pos(self.x, self.y, 600.0);

        d.draw_circle(
            screen_pos.0 as i32,
            screen_pos.1 as i32,
            self.radius,
            Color::GREEN,
        );

        d.draw_circle(
            screen_pos.0 as i32,
            screen_pos.1 as i32,
            self.radius,
            Color::BLACK,
        );

        let eye_offset = self.radius * 0.3;
        let eye_y_offset = self.radius * 0.15;
        d.draw_circle(
            (screen_pos.0 - eye_offset) as i32,
            (screen_pos.1 - eye_y_offset) as i32,
            self.radius * 0.2,
            Color::WHITE,
        );
        d.draw_circle(
            (screen_pos.0 + eye_offset) as i32,
            (screen_pos.1 - eye_y_offset) as i32,
            self.radius * 0.2,
            Color::WHITE,
        );

        d.draw_circle(
            (screen_pos.0 - eye_offset) as i32,
            (screen_pos.1 - eye_y_offset) as i32,
            self.radius * 0.1,
            Color::BLACK,
        );
        d.draw_circle(
            (screen_pos.0 + eye_offset) as i32,
            (screen_pos.1 - eye_y_offset) as i32,
            self.radius * 0.1,
            Color::BLACK,
        );

        let mouth_offset = self.radius * 0.4;
        d.draw_circle(
            screen_pos.0 as i32,
            (screen_pos.1 + mouth_offset) as i32,
            self.radius * 0.15,
            Color::BLACK,
        );
    }

    pub fn update_position(&mut self, physics: &PhysicsWorld, screen_height: f32) {
        if let Some(pos) = physics.get_position(self.handle) {
            self.x = pos.x * PIXELS_PER_METER;
            self.y = screen_height - pos.y * PIXELS_PER_METER;
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
        if self.health <= 0.0 {
            self.destroyed = true;
        }
    }

    pub fn apply_force_from_impact(&mut self, impact_velocity: f32) {
        let damage = impact_velocity * 8.0;
        self.take_damage(damage);
    }

    pub fn is_off_screen(&self, screen_height: f32) -> bool {
        self.y > screen_height + 100.0 || self.y < -200.0
    }
}

fn physics_to_screen_pos(x: f32, y: f32, screen_height: f32) -> (f32, f32) {
    (x, screen_height - y)
}

pub struct PigFactory;

impl PigFactory {
    pub fn create_pig(
        pig_size: PigSize,
        physics: &mut PhysicsWorld,
        x: f32,
        y: f32,
        screen_height: f32,
    ) -> Pig {
        let (radius, _) = match pig_size {
            PigSize::Small => (18.0, 40.0),
            PigSize::Big => (28.0, 80.0),
        };

        let handle = physics.create_dynamic_ball(x, y, radius * 2.0, screen_height);

        if let Some(body) = physics.rigid_body_set.get_mut(handle) {
            body.set_linear_damping(0.5);
            body.set_angular_damping(0.5);
        }

        Pig::new(pig_size, handle, x, y)
    }
}