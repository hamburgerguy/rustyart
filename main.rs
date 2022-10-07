//import libraries
use nannou::prelude::*;
use nannou::noise::*;
//use image::gif::{Decoder, Encoder};
//use image::{ImageDecoder, AnimationDecoder};
//use std::fs::File;

//main fn for running application
fn main() {
    nannou::app(model).update(update).run();
}

//code for creating particle instances
struct Particle {
    positions: Vec<Vec2>,
}

impl Particle {
    pub fn new(p:Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Particle {
            positions,
        }
    }
}

//struct for instantiating nannou instance with types of noise to combine
struct Model {
    particles1: Vec<Particle>,
    particles2: Vec<Particle>,
    particles3: Vec<Particle>,
    particles4: Vec<Particle>,
    particles5: Vec<Particle>,
    particles6: Vec<Particle>,
    noise: Perlin,
    noise2: Perlin,
}

//number of particles of each type in simulation
const N_THINGS: usize = 300;
const N_THINGS2: usize = 50;
const N_THINGS3: usize = 200;
const N_THINGS4: usize = 300;
const N_THINGS5: usize = 300;
const N_THINGS6: usize = 400;

//open window for displaying nannou graphics
fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024,1024).view(view).build().unwrap();
    let mut particles1 = Vec::new();
    let mut particles2 = Vec::new();
    let mut particles3 = Vec::new();
    let mut particles4 = Vec::new();
    let mut particles5 = Vec::new();
    let mut particles6 = Vec::new();

    for i in 0..N_THINGS {
        let particle = Particle::new(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
            ));
        particles1.push(particle);
    }

    for i in 0..N_THINGS2 {
        let particle = Particle::new(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
            ));
        particles2.push(particle);
    }
    for i in 0..N_THINGS3 {
        let particle = Particle::new(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
            ));
        particles3.push(particle);
    }
    for i in 0..N_THINGS4 {
        let particle = Particle::new(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
            ));
        particles4.push(particle);
    }
    for i in 0..N_THINGS5 {
        let particle = Particle::new(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
            ));
        particles5.push(particle);
    }
    for i in 0..N_THINGS6 {
        let particle = Particle::new(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
            ));
        particles6.push(particle);
    }
    
    //noise parameter (fancy math)
    let noise = Perlin::new();
    let noise2 = Perlin::new();
    Model { 
        particles1,
        particles2,
        particles3,
        particles4,
        particles5,
        particles6,
        noise,
        noise2,
     }
}

//update graphics
fn update(app: &App, model: &mut Model, _update: Update) {
    //have parameters change over time (currently creates cool zoom in/zoom out effect)
    let time = app.elapsed_frames() as f32 /60.0;
    let sn = 0.01 + time.cos() as f64*0.005;
    for particle in model.particles1.iter_mut() {
        //instatiate particles in random positions with constant velocity
        particle.positions.clear();
        particle.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
        ));

        for k in 0..50 {
            let last = particle.positions[0];
            let new = last + vec2(
            
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise2.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            particle.positions.insert(0,new);
        }
    }
    for particle in model.particles2.iter_mut() {
        //instatiate particles in random positions with constant velocity
        particle.positions.clear();
        particle.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
        ));

        for k in 0..50 {
            let last = particle.positions[0];
            let new = last + vec2(
            
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise2.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            particle.positions.insert(0,new);
        }
    }
    for particle in model.particles3.iter_mut() {
        //instatiate particles in random positions with constant velocity
        particle.positions.clear();
        particle.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
        ));

        for k in 0..50 {
            let last = particle.positions[0];
            let new = last + vec2(
            
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise2.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            particle.positions.insert(0,new);
        }
    }
    for particle in model.particles4.iter_mut() {
        //instatiate particles in random positions with constant velocity
        particle.positions.clear();
        particle.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
        ));

        for k in 0..50 {
            let last = particle.positions[0];
            let new = last + vec2(
            
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise2.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            particle.positions.insert(0,new);
        }
    }
    for particle in model.particles5.iter_mut() {
        //instatiate particles in random positions with constant velocity
        particle.positions.clear();
        particle.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
        ));

        for k in 0..50 {
            let last = particle.positions[0];
            let new = last + vec2(
            // Generate Noise vector field (TODO change float values so less dirty)
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise2.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            particle.positions.insert(0,new);
        }
    }
    for particle in model.particles6.iter_mut() {
        //instatiate particles in random positions with constant velocity
        particle.positions.clear();
        particle.positions.push(vec2(
            (random::<f32>()-0.5)*1024.0,
            (random::<f32>()-0.5)*1024.0,
        ));

        for k in 0..50 {
            let last = particle.positions[0];
            let new = last + vec2(
            
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise2.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32,
            );
            particle.positions.insert(0,new);
        }
    }
}

//code for generating the graphical output. 
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    //create time parameter
    let _time = app.elapsed_frames() as f32 /60.0;

    //change background colour
    if app.elapsed_frames()==1 {
        draw.background().color(BLACK);
    }
    //used for creating a faint black background to make it less static over time
    draw.rect().w_h(1024.0,1024.0).color(srgba(0.0,0.0,0.0,0.1));

    //change color of particles
    for particle in model.particles1.iter() {
        draw.polyline().points(particle.positions.iter().cloned()).color(MAROON);
    }
    for particle in model.particles2.iter() {
        draw.polyline().points(particle.positions.iter().cloned()).color(ORANGE);
    }
    for particle in model.particles3.iter() {
        draw.polyline().points(particle.positions.iter().cloned()).color(CRIMSON);
    }
    for particle in model.particles4.iter() {
        draw.polyline().points(particle.positions.iter().cloned()).color(FIREBRICK);
    }
    for particle in model.particles5.iter() {
        draw.polyline().points(particle.positions.iter().cloned()).color(GOLDENROD);
    }
    for particle in model.particles6.iter() {
        draw.polyline().points(particle.positions.iter().cloned()).color(PERU);
    }
    
    draw.to_frame(app, &frame).unwrap();
    // TODO Decode a gif into frames and output as file
}


