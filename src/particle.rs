use sdl2::pixels::Color;

use crate::element::Element;

#[derive(Clone, Copy)]
pub struct Particle {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub element: Element,
    pub age: i32,
}

impl Particle {
    pub fn new(x: i32, y: i32, element: Element, next_particle_id: u32) -> Particle {
        Particle {
            id: next_particle_id,
            x,
            y,
            element,
            age: 0,
        }
    }

    pub fn color(&self) -> Color {
        self.element.color()
    }
}
