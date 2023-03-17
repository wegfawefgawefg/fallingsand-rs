use std::fmt;

use enum_iterator::Sequence;
use sdl2::pixels::Color;

#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
pub enum Element {
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
impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Element::Sand => write!(f, "Sand"),
            Element::Water => write!(f, "Water"),
            Element::Gas => write!(f, "Gas"),
            Element::Fire => write!(f, "Fire"),
            Element::Smoke => write!(f, "Smoke"),
            Element::Steam => write!(f, "Steam"),
            Element::Wood => write!(f, "Wood"),
            Element::Wall => write!(f, "Wall"),
            Element::Ice => write!(f, "Ice"),
        }
    }
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Sand => Color::RGBA(255, 255, 0, 255),
            Element::Water => Color::RGBA(0, 0, 255, 255),
            Element::Gas => Color::RGBA(255, 255, 255, 255),
            Element::Fire => Color::RGBA(255, 0, 0, 255),
            Element::Smoke => Color::RGBA(128, 128, 128, 255),
            Element::Steam => Color::RGBA(200, 200, 255, 255),
            Element::Wood => Color::RGBA(128, 64, 0, 255),
            Element::Wall => Color::RGBA(255, 255, 255, 255),
            Element::Ice => Color::RGBA(200, 200, 255, 255),
        }
    }
}
