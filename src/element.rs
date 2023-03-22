use std::fmt;

use enum_iterator::Sequence;
use sdl2::pixels::Color;

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum Element {
    Air,
    Sand,
    Oil,
    Water,
    Fire,
    Smoke,
    Steam,
    Wood,
    Wall,
    Ice,
    Lava,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::Air => write!(f, "Air"),
            Element::Sand => write!(f, "Sand"),
            Element::Oil => write!(f, "Oil"),
            Element::Water => write!(f, "Water"),
            Element::Fire => write!(f, "Fire"),
            Element::Smoke => write!(f, "Smoke"),
            Element::Steam => write!(f, "Steam"),
            Element::Wood => write!(f, "Wood"),
            Element::Wall => write!(f, "Wall"),
            Element::Ice => write!(f, "Ice"),
            Element::Lava => write!(f, "Lava"),
        }
    }
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Air => Color::RGBA(0, 0, 0, 0),
            Element::Sand => Color::RGBA(255, 255, 0, 255),
            Element::Oil => Color::RGBA(255, 0, 255, 255),
            Element::Water => Color::RGBA(0, 0, 255, 255),
            Element::Fire => Color::RGBA(255, 0, 0, 255),
            Element::Smoke => Color::RGBA(128, 128, 128, 255),
            Element::Steam => Color::RGBA(200, 200, 255, 255),
            Element::Wood => Color::RGBA(128, 64, 0, 255),
            Element::Wall => Color::RGBA(255, 255, 255, 255),
            Element::Ice => Color::RGBA(200, 200, 255, 255),
            Element::Lava => Color::RGBA(255, 128, 0, 255),
        }
    }
}
