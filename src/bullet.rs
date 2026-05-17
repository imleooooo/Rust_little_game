use raylib::prelude::*;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
    pub active: bool,
}

impl Bullet {
    pub fn new(x: f32, y: f32) -> Self {
        Bullet {
            x,
            y,
            speed: 10.0,
            radius: 6.0,
            color: Color::YELLOW,
            active: true,
        }
    }

    pub fn update(&mut self) {
        self.y -= self.speed;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_circle(self.x as i32, self.y as i32, self.radius, self.color);
        }
    }

    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0)
    }
}