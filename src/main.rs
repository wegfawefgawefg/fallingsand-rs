use element::Element;
use enum_iterator::{first, last, next, previous};
use particle::Particle;
// use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

use settings::{HEIGHT, WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use ui::{draw_particle_count, draw_particle_options};

mod element;
mod particle;
mod render;
mod settings;
mod ui;

use std::time::Duration;

fn spawn_particle(
    x: i32,
    y: i32,
    element: Element,
    particles: &mut Vec<Particle>,
    next_particle_id: &mut u32,
    grid: &mut Vec<Vec<Option<usize>>>,
) {
    if let Some(particle_id) = grid[y as usize][x as usize] {
        particles.retain(|p| p.id != particle_id as u32);
    }

    let particle = Particle::new(x, y, element, *next_particle_id);
    *next_particle_id += 1;
    particles.push(particle);
    grid[y as usize][x as usize] = Some(particle.id as usize);
}

fn update(p: &mut Particle, _grid: &mut Vec<Vec<Option<usize>>>) {
    p.age += 1;
    match p.element {
        Element::Sand => { /* Update sand behavior */ }
        Element::Water => { /* Update water behavior */ }
        Element::Gas => { /* Update gas behavior */ }
        Element::Fire => { /* Update fire behavior */ }
        Element::Smoke => { /* Update smoke behavior */ }
        Element::Steam => { /* Update steam behavior */ }
        Element::Wood => { /* Update wood behavior */ }
        Element::Wall => { /* Update wall behavior */ }
        Element::Ice => { /* Update ice behavior */ }
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
    let mut current_element = first::<Element>().unwrap();

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
                    let n = previous(&current_element);
                    // if we have reached the begining of the enum, loop to the end
                    if n.is_none() {
                        current_element = last::<Element>().unwrap();
                    } else {
                        current_element = n.unwrap();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    let n = next(&current_element);
                    // if we have reached the end of the enum, loop to the begining
                    if n.is_none() {
                        current_element = first::<Element>().unwrap();
                    } else {
                        current_element = n.unwrap();
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
                            current_element,
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
            &current_element,
            &texture_creator,
        );

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
