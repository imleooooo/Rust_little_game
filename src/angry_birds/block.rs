use raylib::prelude::*;
use rapier2d::prelude::*;
use crate::angry_birds::physics::{PhysicsWorld, PIXELS_PER_METER};

#[derive(Clone, Debug, PartialEq)]
pub enum BlockType {
    Wood,
    Stone,
    Ice,
}

pub struct Block {
    pub block_type: BlockType,
    pub handle: RigidBodyHandle,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub health: f32,
    pub max_health: f32,
    pub destroyed: bool,
}

impl Block {
    pub fn new(block_type: BlockType, handle: RigidBodyHandle, x: f32, y: f32, width: f32, height: f32) -> Self {
        let (health, max_health) = match block_type {
            BlockType::Wood => (50.0, 50.0),
            BlockType::Stone => (150.0, 150.0),
            BlockType::Ice => (30.0, 30.0),
        };

        Block {
            block_type,
            handle,
            x,
            y,
            width,
            height,
            health,
            max_health,
            destroyed: false,
        }
    }

    pub fn get_color(&self) -> Color {
        match self.block_type {
            BlockType::Wood => Color::BROWN,
            BlockType::Stone => Color::GRAY,
            BlockType::Ice => Color::SKYBLUE,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.destroyed {
            return;
        }

        let screen_pos = physics_to_screen_pos(self.x, self.y, 600.0);
        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        d.draw_rectangle(
            (screen_pos.0 - half_w) as i32,
            (screen_pos.1 - half_h) as i32,
            self.width as i32,
            self.height as i32,
            self.get_color(),
        );

        d.draw_rectangle_lines(
            (screen_pos.0 - half_w) as i32,
            (screen_pos.1 - half_h) as i32,
            self.width as i32,
            self.height as i32,
            Color::BLACK,
        );

        let health_ratio = self.health / self.max_health;
        let bar_width = self.width * 0.8;
        let bar_height = 5.0;
        let bar_x = screen_pos.0 - bar_width / 2.0;
        let bar_y = screen_pos.1 - half_h - 10.0;

        d.draw_rectangle(
            bar_x as i32,
            bar_y as i32,
            bar_width as i32,
            bar_height as i32,
            Color::RED,
        );
        d.draw_rectangle(
            bar_x as i32,
            bar_y as i32,
            (bar_width * health_ratio) as i32,
            bar_height as i32,
            Color::GREEN,
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
        let damage = impact_velocity * 5.0;
        self.take_damage(damage);
    }

    pub fn is_off_screen(&self, screen_height: f32) -> bool {
        self.y > screen_height + 100.0 || self.y < -200.0
    }
}

fn physics_to_screen_pos(x: f32, y: f32, screen_height: f32) -> (f32, f32) {
    (x, screen_height - y)
}

pub struct BlockFactory;

impl BlockFactory {
    pub fn create_block(
        block_type: BlockType,
        physics: &mut PhysicsWorld,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        screen_height: f32,
    ) -> Block {
        let handle = physics.create_dynamic_box(x, y, width, height, screen_height);

        if let Some(body) = physics.rigid_body_set.get_mut(handle) {
            let density = match block_type {
                BlockType::Wood => 1.0,
                BlockType::Stone => 3.0,
                BlockType::Ice => 0.8,
            };
        }

        Block::new(block_type, handle, x, y, width, height)
    }
}