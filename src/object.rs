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

    pub fn integrate(&mut self, dt: f32, collisions: &Vec<Object>) {
        let temp = self.old_position;
        self.old_position = self.position;

        self.apply_gravity();
        self.apply_bounds();
        self.apply_collisions(collisions); // this causes explosions ! :(

        self.position += (self.position - temp) + self.acceleration * dt.pow(2f32);
        self.acceleration *= Vec2::ZERO;
    }

    pub fn apply_bounds(&mut self) {
        /* Forcefully keep objects within 300 of origin.
            * See main.rs#view for more information.
            */
        let dist = self.position.distance(Vec2::ZERO) + self.radius;
        if dist > 300f32 {
            self.position = self.position.normalize_or_zero() * (300f32 - self.radius);
        }
    }

    pub fn apply_collisions(&mut self, collisions: &Vec<Object>) {
        for other in collisions {
            // true equality wouldn't work since `other` is a clone, check position instead
            if self.position == other.position {
                continue;
            }
            let dist = self.position.distance(other.position) - self.radius - other.radius;
            if dist <= 0f32 {
                self.position = other.position
                    + (self.position - other.position).normalize_or_zero() * (other.radius + self.radius);
                println!("Updated distance is {}", self.position.distance(other.position) - self.radius - other.radius);
            }
        }
    }

    pub fn apply_gravity(&mut self) {
        // Acceleration due to gravity
        self.accelerate(Vec2::new(0f32, -0.002f32));
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