use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Texture;
use sdl2::video::Window;
use sdl2::{rect::Rect, render::Canvas};

use crate::grid::Grid;
use crate::particle::Particle;
use crate::settings::{WINDOW_HEIGHT, WINDOW_WIDTH};

/*
    Particle renduring should be done by operating on the intermediary canvas directly, likely
    likely the data should be treating the 2d buffer as the grid itself, with perhaps one full copy per frame.
    // how can i do a single draw call for all particles?

*/

pub fn render_particles(
    canvas: &mut Canvas<Window>,
    intermediary_canvas: &mut Texture,
    grid: &Grid,
) {
    canvas
        .with_texture_canvas(intermediary_canvas, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();

            for (x, y, p) in grid.iter() {
                texture_canvas.set_draw_color(p.color());
                let _ = texture_canvas.draw_point(Point::new(x as i32, y as i32));
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
