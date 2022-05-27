mod object;
use std::time::{SystemTime, UNIX_EPOCH};

use object::Object;

use rand::prelude::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
    .update(update)
    .simple_window(view)
    .run();
}

struct Model {
    pub objects: Vec<Object>,
    last_update: u128
}

fn current_time() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get current time!").as_millis()
}

fn model(_app: &App) -> Model {
    _app.set_loop_mode(LoopMode::rate_fps(144.0));
    let mut model = Model { objects: Vec::new(), last_update: current_time() };
    
    for _ in 0..30 {
        let theta = random::<f32>() * 6.28318530718;
        let radius = (300f32 - 20f32) * random::<f32>().sqrt();
        
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        let mut object = Object::new(x, y);
        object.position += Vec2::new((random::<f32>()-0.5)*15f32, (random::<f32>()-0.5)*15f32); 
        model.objects.push(object);
    }
    
    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    /* Nannou is _supposed_ to allow us to easily declare separate
    * rates for view() and update(), but as of now, actually trying
    * to use LoopMode::Rate causes issues, as it sometimes runs with
    * excessively high delta-time (for example when moving the window
        * across virtual desktops).
        * 
        * This issue is solved here by manually implementing a better, consistent
        * rate. It _always_ has a delta time of _exactly_ period_ms, and simply
        * attempts to fast-track the simulation if it runs too slowly.
        * 
        * This way we don't need to mess with variable time-step.
        * 
        * TL;DR we schedule shit ourselves because nannou is stupid
        */
        let rate = 240f32;
        let period_ms: f32 = 1f32 / rate * 1000f32;
        
        
        /* I was concerned integer precision wasn't good enough
        * when fast-tracking, so I used a floating-point accumulator
        * which is then rounded when actually modifying last_update
        * 
        * Perhaps last_update could even just be modified after the loop?
        * I'm not sure.
        */
        let initial = model.last_update;
        let mut add = 0f32;
        
        while (current_time() - model.last_update) as f32 >= period_ms {
            add += period_ms;
            model.last_update = initial + add as u128;
            let len = model.objects.len();
            for i in 0..len {

                let (before, rest) = model.objects.as_mut_slice().split_at_mut(i);
                let (current, after) = rest.split_at_mut(1);
                let object = &mut current[0];

                
                let temp = object.old_position;
                object.old_position = object.position;
                
                object.apply_gravity();
                object.apply_bounds();
                
                for slice in [before, after] {
                    for other in slice {
                        let dist = object.position.distance(other.position) - object.radius - other.radius;
                        if dist <= 0f32 {
                            let midpoint = (object.position + other.position) / 2.0;
                            object.position = midpoint + (object.position - midpoint).normalize_or_zero() * object.radius;
                            other.position = midpoint + (other.position - midpoint).normalize_or_zero() * other.radius;
                        }
                    }
                }
                
                
                object.position += (object.position - temp) + object.acceleration * period_ms.pow(2f32) * 0.5;
                object.acceleration *= Vec2::ZERO;
            }
        }
    }
    
    fn view(_app: &App, model: &Model, frame: Frame) {
        let draw = _app.draw();
        
        draw.background().color(BLACK);
        
        
        /* All objects are hard-coded to stay within a radius of 300 around the origin.
        * This displays that area. I know there should probably be a fancy object type
        * system with a constraint object for this, but I don't want to make that.
        */
        
        draw.ellipse().stroke(WHITE)
        .rgba(0f32, 0f32,0f32, 0f32)
        .stroke_weight(6f32)
        .radius(300f32)
        .finish();
        
        model.objects.iter().for_each(|obj| obj.draw(&draw));
        
        draw.to_frame(_app, &frame)
        .expect("Failed to draw frame! Is the window missing?");
    }