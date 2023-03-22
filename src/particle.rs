use sdl2::pixels::Color;

use crate::element::Element;

#[derive(Clone, Copy)]
pub struct Particle {
    pub element: Element,
    pub last_ticked: u32,
    pub age: u32,
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            element: Element::Air,
            last_ticked: 0,
            age: 0,
        }
    }
}

impl Particle {
    pub fn new(element: Element, last_ticked: u32) -> Particle {
        Particle {
            element,
            last_ticked: last_ticked,
            age: 0,
        }
    }

    pub fn color(&self) -> Color {
        self.element.color()
    }

    pub fn lifetime(&self) -> u32 {
        match self.element {
            Element::Air => 0,
            Element::Sand => 0,
            Element::Water => 0,
            Element::Oil => 0,
            Element::Fire => 8,
            Element::Smoke => 120,
            Element::Steam => 0,
            Element::Wood => 0,
            Element::Wall => 0,
            Element::Ice => 0,
            Element::Lava => 0,
        }
    }

    pub fn density(&self) -> i32 {
        match self.element {
            Element::Air => 0,
            Element::Sand => 20,
            Element::Water => 10,
            Element::Oil => 9,
            Element::Fire => 0,
            Element::Smoke => -1,
            Element::Steam => -2,
            Element::Wood => 30,
            Element::Wall => 100,
            Element::Ice => 30,
            Element::Lava => 30,
        }
    }

    pub fn is_static(&self) -> bool {
        match self.element {
            Element::Air => false,
            Element::Sand => false,
            Element::Water => false,
            Element::Oil => false,
            Element::Fire => false,
            Element::Smoke => false,
            Element::Steam => false,
            Element::Wood => false,
            Element::Wall => true,
            Element::Ice => false,
            Element::Lava => false,
        }
    }
}
