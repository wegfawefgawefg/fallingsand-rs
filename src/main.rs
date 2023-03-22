use element::Element;
use enum_iterator::{first, last, next, previous};
use grid::Grid;
use particle::Particle;
use particle_behaviour::step_particles;
use render::render_particles;

// use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureCreator};

use settings::{WINDOW_HEIGHT, WINDOW_WIDTH};
use ui::{draw_particle_count, draw_particle_options};

mod element;
mod grid;
mod particle;
mod particle_behaviour;
mod render;
mod settings;
mod ui;

use std::time::Duration;

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
        .create_texture_target(None, Grid::WIDTH as u32, Grid::HEIGHT as u32)
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
    let mut frame_clock = 0;
    let mut grid = Grid::new();
    let mut current_element = first::<Element>().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
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
                    let x = x * Grid::WIDTH as i32 / WINDOW_WIDTH as i32;
                    let y = y * Grid::HEIGHT as i32 / WINDOW_HEIGHT as i32;
                    let is_mouse_down = mousestate.left();
                    if is_mouse_down {
                        grid.set(
                            x,
                            y,
                            Particle {
                                element: current_element,
                                ..Default::default()
                            },
                        );
                    }
                }
                _ => {}
            }
        }

        // update zone
        step_particles(&mut grid, frame_clock);

        // render zone
        render_particles(&mut canvas, &mut intermediary_canvas, &grid);
        // draw_particle_count(&mut canvas, &small_font, &particles, &texture_creator);
        draw_particle_options(
            &mut canvas,
            &small_font,
            &large_font,
            &current_element,
            &texture_creator,
        );

        canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));

        frame_clock += 1;
    }
}
