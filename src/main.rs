use std::fmt;
use std::time::Duration;

// use rand::Rng;
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;

use sdl2::video::Window;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
enum ParticleType {
    Sand,
    Water,
    Gas,
    Fire,
    Smoke,
    Steam,
    Wood,
    Wall,
    Ice,
}
impl fmt::Display for ParticleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParticleType::Sand => write!(f, "Sand"),
            ParticleType::Water => write!(f, "Water"),
            ParticleType::Gas => write!(f, "Gas"),
            ParticleType::Fire => write!(f, "Fire"),
            ParticleType::Smoke => write!(f, "Smoke"),
            ParticleType::Steam => write!(f, "Steam"),
            ParticleType::Wood => write!(f, "Wood"),
            ParticleType::Wall => write!(f, "Wall"),
            ParticleType::Ice => write!(f, "Ice"),
        }
    }
}

impl ParticleType {
    fn color(&self) -> Color {
        match *self {
            ParticleType::Sand => Color::RGBA(255, 255, 0, 255),
            ParticleType::Water => Color::RGBA(0, 0, 255, 255),
            ParticleType::Gas => Color::RGBA(255, 255, 255, 255),
            ParticleType::Fire => Color::RGBA(255, 0, 0, 255),
            ParticleType::Smoke => Color::RGBA(128, 128, 128, 255),
            ParticleType::Steam => Color::RGBA(200, 200, 255, 255),
            ParticleType::Wood => Color::RGBA(128, 64, 0, 255),
            ParticleType::Wall => Color::RGBA(255, 255, 255, 255),
            ParticleType::Ice => Color::RGBA(200, 200, 255, 255),
        }
    }
}

#[derive(Clone, Copy)]
struct Particle {
    id: u32,
    x: i32,
    y: i32,
    ptype: ParticleType,
    age: i32,
}

impl Particle {
    fn new(x: i32, y: i32, ptype: ParticleType, next_particle_id: u32) -> Particle {
        Particle {
            id: next_particle_id,
            x,
            y,
            ptype,
            age: 0,
        }
    }

    fn color(&self) -> Color {
        self.ptype.color()
    }
}

fn spawn_particle(
    x: i32,
    y: i32,
    ptype: ParticleType,
    particles: &mut Vec<Particle>,
    next_particle_id: &mut u32,
    grid: &mut Vec<Vec<Option<usize>>>,
) {
    if let Some(particle_id) = grid[y as usize][x as usize] {
        particles.retain(|p| p.id != particle_id as u32);
    }

    let particle = Particle::new(x, y, ptype, *next_particle_id);
    *next_particle_id += 1;
    particles.push(particle);
    grid[y as usize][x as usize] = Some(particle.id as usize);
}

fn update(p: &mut Particle, _grid: &mut Vec<Vec<Option<usize>>>) {
    p.age += 1;
    match p.ptype {
        ParticleType::Sand => { /* Update sand behavior */ }
        ParticleType::Water => { /* Update water behavior */ }
        ParticleType::Gas => { /* Update gas behavior */ }
        ParticleType::Fire => { /* Update fire behavior */ }
        ParticleType::Smoke => { /* Update smoke behavior */ }
        ParticleType::Steam => { /* Update steam behavior */ }
        ParticleType::Wood => { /* Update wood behavior */ }
        ParticleType::Wall => { /* Update wall behavior */ }
        ParticleType::Ice => { /* Update ice behavior */ }
    }
}

// draw_particle_count
// should draw the total number of particles in the top right
fn draw_particle_count(
    canvas: &mut Canvas<Window>,
    font: &Font,
    particles: &Vec<Particle>,
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
) {
    let text = format!("Particles: {}", particles.len());
    let surface = font
        .render(&text)
        .blended(Color::RGBA(255, 255, 255, 255))
        .map_err(|e| e.to_string())
        .unwrap();
    let texture = surface
        .as_texture(texture_creator)
        .map_err(|e| e.to_string())
        .unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let dst = Rect::new(WINDOW_WIDTH as i32 - width as i32 - 10, 10, width, height);
    canvas.copy(&texture, None, dst).unwrap();
}

fn draw_particle_options(
    canvas: &mut Canvas<Window>,
    small_font: &Font,
    large_font: &Font,
    current_particle_type: &ParticleType,
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
) {
    let mut y = 10;
    let particle_types: Vec<ParticleType> = all::<ParticleType>().collect::<Vec<_>>();
    for particle_type in particle_types {
        let text = format!("{}", particle_type);
        let font = if particle_type == *current_particle_type {
            large_font
        } else {
            small_font
        };
        let surface = font
            .render(&text)
            .blended(particle_type.color())
            .map_err(|e| e.to_string())
            .unwrap();
        let texture = surface
            .as_texture(texture_creator)
            .map_err(|e| e.to_string())
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let dst = Rect::new(10, y, width, height);
        canvas.copy(&texture, None, dst).unwrap();
        y += 40;
    }
}

fn main() {
    // Initialize SDL2 boilerplate
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Falling Sand Simulation", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    //  subtexture stuff for interacting with a lower res buffer which is then scaled up to the window size
    //  //  makes the rendering much faster
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut intermediary_canvas: Texture = texture_creator
        .create_texture_target(None, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    // Asset loading
    //  // Load font
    let ttf_context = sdl2::ttf::init()
        .map_err(|e| format!("Failed to init ttf: {}", e))
        .unwrap();
    let font_path = "assets/Simple-Bold.ttf";
    let small_font = ttf_context
        .load_font(font_path, 16)
        .map_err(|e| format!("Failed to load font: {}", e))
        .unwrap();
    let large_font = ttf_context
        .load_font(font_path, 24)
        .map_err(|e| format!("Failed to load font: {}", e))
        .unwrap();

    //  state
    let mut next_particle_id = 0;
    let mut particles: Vec<Particle> = Vec::new();
    let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; WIDTH as usize]; HEIGHT as usize];
    let mut current_particle_type = first::<ParticleType>().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    let n = previous(&current_particle_type);
                    // if we have reached the begining of the enum, loop to the end
                    if n.is_none() {
                        current_particle_type = last::<ParticleType>().unwrap();
                    } else {
                        current_particle_type = n.unwrap();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let n = next(&current_particle_type);
                    // if we have reached the end of the enum, loop to the begining
                    if n.is_none() {
                        current_particle_type = first::<ParticleType>().unwrap();
                    } else {
                        current_particle_type = n.unwrap();
                    }
                }
                Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate,
                    x,
                    y,
                    xrel: _,
                    yrel: _,
                } => {
                    let x = x * WIDTH as i32 / WINDOW_WIDTH as i32;
                    let y = y * HEIGHT as i32 / WINDOW_HEIGHT as i32;
                    let is_mouse_down = mousestate.left();
                    if is_mouse_down {
                        spawn_particle(
                            x,
                            y,
                            current_particle_type,
                            &mut particles,
                            &mut next_particle_id,
                            &mut grid,
                        );
                    }
                }
                _ => {}
            }
        }

        // Update particles
        for particle in &mut particles {
            update(particle, &mut grid);
        }

        canvas
            .with_texture_canvas(&mut intermediary_canvas, |texture_canvas| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();

                // Draw particles
                for particle in &particles {
                    texture_canvas.set_draw_color(particle.color());
                    let _ = texture_canvas.fill_rect(Rect::new(
                        particle.x as i32,
                        particle.y as i32,
                        1,
                        1,
                    ));
                }
            })
            .unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw the intermediary canvas onto the main canvas, scaled to the window size
        canvas
            .copy(
                &intermediary_canvas,
                None,
                Some(Rect::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)),
            )
            .unwrap();

        draw_particle_count(&mut canvas, &small_font, &particles, &texture_creator);
        draw_particle_options(
            &mut canvas,
            &small_font,
            &large_font,
            &current_particle_type,
            &texture_creator,
        );

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
