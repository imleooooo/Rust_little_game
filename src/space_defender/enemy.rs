use raylib::prelude::*;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub active: bool,
}

impl Enemy {
    pub fn new(x: f32, speed: f32) -> Self {
        Enemy {
            x,
            y: -40.0,
            speed,
            width: 45.0,
            height: 35.0,
            color: Color::RED,
            active: true,
        }
    }

    pub fn update(&mut self) {
        self.y += self.speed;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_rectangle(
                (self.x - self.width / 2.0) as i32,
                (self.y - self.height / 2.0) as i32,
                self.width as i32,
                self.height as i32,
                self.color,
            );
            d.draw_circle(
                (self.x - self.width / 4.0) as i32,
                (self.y - self.height / 4.0) as i32,
                6.0,
                Color::WHITE,
            );
            d.draw_circle(
                (self.x + self.width / 4.0) as i32,
                (self.y - self.height / 4.0) as i32,
                6.0,
                Color::WHITE,
            );
        }
    }

    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
        )
    }
}