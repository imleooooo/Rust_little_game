use raylib::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Player {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        Player {
            x: (screen_width / 2) as f32,
            y: (screen_height - 80) as f32,
            speed: 8.0,
            width: 50.0,
            height: 40.0,
            color: Color::BLUE,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            self.x -= self.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            self.x += self.speed;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            (self.x - self.width / 2.0) as i32,
            (self.y - self.height / 2.0) as i32,
            self.width as i32,
            self.height as i32,
            self.color,
        );
        d.draw_triangle(
            Vector2::new(self.x, self.y - self.height / 2.0),
            Vector2::new(self.x - self.width / 2.0, self.y + self.height / 2.0),
            Vector2::new(self.x + self.width / 2.0, self.y + self.height / 2.0),
            Color::SKYBLUE,
        );
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