use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::{rect::Rect, render::Canvas};

use crate::particle::Particle;
use crate::settings::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_particles(
    canvas: &mut Canvas<Window>,
    intermediary_canvas: &mut Texture,
    particles: &Vec<Particle>,
) {
    canvas
        .with_texture_canvas(intermediary_canvas, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            // Draw particles
            for particle in particles {
                texture_canvas.set_draw_color(particle.color());
                let _ =
                    texture_canvas.fill_rect(Rect::new(particle.x as i32, particle.y as i32, 1, 1));
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
}
