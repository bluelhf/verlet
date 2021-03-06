use nannou::prelude::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Object {
    pub position: Vec2,
    pub old_position: Vec2,
    pub acceleration: Vec2,
    pub radius: f32,
    pub rgba: (f32, f32, f32, f32),


}

impl Object {
    pub fn new(x: f32, y: f32) -> Self {
        let pos = Vec2::new(x, y);

        Self {
            position: pos, 
            old_position: pos.clone(), 
            acceleration: Vec2::new(0f32, 0f32), 
            radius: 20f32, 
            rgba: (1f32, 1f32, 1f32, 1f32)
        }
    }

    pub fn accelerate(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn draw(&self, draw: &Draw) {
        let (r, g, b, a) = self.rgba;
        let stroke = gray(0.8);
        draw.ellipse()
            .rgba(r, g, b, a)
            .stroke(stroke).stroke_weight(1f32)
            .radius(self.radius)
            .x(self.position.x).y(self.position.y).finish();
    }
}