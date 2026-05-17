use raylib::prelude::*;
use rapier2d::prelude::*;
use nalgebra::Vector2 as NalgebraVector2;
use crate::angry_birds::physics::{PhysicsWorld, PIXELS_PER_METER};

pub struct Slingshot {
    pub base_x: f32,
    pub base_y: f32,
    pub pull_x: f32,
    pub pull_y: f32,
    pub is_pulling: bool,
    pub max_pull_distance: f32,
}

impl Slingshot {
    pub fn new(x: f32, y: f32) -> Self {
        Slingshot {
            base_x: x,
            base_y: y,
            pull_x: x,
            pull_y: y,
            is_pulling: false,
            max_pull_distance: 100.0,
        }
    }

    pub fn update_pull(&mut self, mouse_x: f32, mouse_y: f32) {
        if !self.is_pulling {
            return;
        }

        self.pull_x = mouse_x;
        self.pull_y = mouse_y;

        let dx = self.base_x - self.pull_x;
        let dy = self.base_y - self.pull_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > self.max_pull_distance {
            let angle = dy.atan2(dx);
            self.pull_x = self.base_x - angle.cos() * self.max_pull_distance;
            self.pull_y = self.base_y - angle.sin() * self.max_pull_distance;
        }
    }

    pub fn release(&mut self) -> Option<NalgebraVector2<f32>> {
        if !self.is_pulling {
            return None;
        }

        self.is_pulling = false;

        let dx = self.base_x - self.pull_x;
        let dy = self.base_y - self.pull_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 10.0 {
            self.pull_x = self.base_x;
            self.pull_y = self.base_y;
            return None;
        }

        let power = distance / self.max_pull_distance * 30.0;
        let velocity = NalgebraVector2::new(
            dx / distance * power,
            -dy / distance * power,
        );

        self.pull_x = self.base_x;
        self.pull_y = self.base_y;

        Some(velocity)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            (self.base_x - 8.0) as i32,
            (self.base_y - 60.0) as i32,
            16,
            80,
            Color::BROWN,
        );

        let v1 = Vector2::new(self.base_x - 30.0, self.base_y - 50.0);
        let v2 = Vector2::new(self.base_x + 30.0, self.base_y - 50.0);
        let v3 = Vector2::new(self.base_x, self.base_y);
        d.draw_triangle(v1, v2, v3, Color::DARKBROWN);

        let left_elastic_top = Vector2::new(self.base_x - 30.0, self.base_y - 50.0);
        let right_elastic_top = Vector2::new(self.base_x + 30.0, self.base_y - 50.0);
        let pull_point = Vector2::new(self.pull_x, self.pull_y);

        d.draw_line_ex(left_elastic_top, pull_point, 4.0, Color::RED);
        d.draw_line_ex(right_elastic_top, pull_point, 4.0, Color::RED);

        d.draw_circle(self.pull_x as i32, self.pull_y as i32, 10.0, Color::YELLOW);
    }
}

pub fn launch_bird(
    physics: &mut PhysicsWorld,
    handle: RigidBodyHandle,
    velocity: NalgebraVector2<f32>,
    _screen_height: f32,
) {
    let phys_velocity = NalgebraVector2::new(
        velocity.x / PIXELS_PER_METER * 60.0,
        -velocity.y / PIXELS_PER_METER * 60.0,
    );

    if let Some(body) = physics.rigid_body_set.get_mut(handle) {
        body.set_linvel(phys_velocity, true);
    }

    physics.wake_up(handle);
}

pub fn calculate_trajectory(start_x: f32, start_y: f32, velocity: NalgebraVector2<f32>, steps: usize) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    let dt = 1.0 / 60.0;
    let gravity = 25.0;

    let mut x = start_x;
    let mut y = start_y;
    let vx = velocity.x;
    let mut vy = -velocity.y;

    for _ in 0..steps {
        x += vx * dt * 60.0;
        y += vy * dt * 60.0;
        vy -= gravity * dt;

        points.push((x, y));

        if y > 600.0 || x < 0.0 || x > 800.0 {
            break;
        }
    }

    points
}