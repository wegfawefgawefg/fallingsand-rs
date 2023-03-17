use enum_iterator::all;
use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator, TextureQuery},
    ttf::Font,
};

use sdl2::video::Window;

use crate::{element::Element, particle::Particle, settings::WINDOW_WIDTH};

// draw_particle_count
// should draw the total number of particles in the top right
pub fn draw_particle_count(
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

pub fn draw_particle_options(
    canvas: &mut Canvas<Window>,
    small_font: &Font,
    large_font: &Font,
    current_element: &Element,
    texture_creator: &TextureCreator<sdl2::video::WindowContext>,
) {
    let mut y = 10;
    let elements: Vec<Element> = all::<Element>().collect::<Vec<_>>();
    for element in elements {
        let text = format!("{}", element);
        let font = if element == *current_element {
            large_font
        } else {
            small_font
        };
        let surface = font
            .render(&text)
            .blended(element.color())
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
