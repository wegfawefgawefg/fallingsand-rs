use crate::{element::Element, particle::Particle};

pub struct Grid {
    pub particles: Vec<Vec<Particle>>,
}

impl Grid {
    // width and height
    pub const WIDTH: i32 = 64;
    pub const HEIGHT: i32 = 64;
    // pub const WIDTH: i32 = 128;
    // pub const HEIGHT: i32 = 128;
    pub fn new() -> Grid {
        Grid {
            particles: vec![vec![Particle::default(); Grid::WIDTH as usize]; Grid::HEIGHT as usize],
        }
    }

    // define an enumerated iterator over the grid with x and y
    pub fn iter(&self) -> impl Iterator<Item = (i32, i32, &Particle)> {
        (0..Grid::HEIGHT)
            .flat_map(move |y| (0..Grid::WIDTH).map(move |x| (x, y)))
            .map(move |(x, y)| (x, y, &self.particles[y as usize][x as usize]))
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Particle> {
        if Grid::pos_in_world(x, y) {
            Some(&self.particles[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Particle> {
        if Grid::pos_in_world(x, y) {
            Some(&mut self.particles[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: i32, y: i32, p: Particle) {
        if !Grid::pos_in_world(x, y) {
            return;
        }
        self.particles[y as usize][x as usize] = p;
    }

    pub fn pos_in_world(x: i32, y: i32) -> bool {
        x >= 0 && x < Grid::WIDTH && y >= 0 && y < Grid::HEIGHT
    }

    /// this is where you would implement bouyancy, for solids vs liquids, and liquids vs liquids
    pub fn swap(&mut self, x: i32, y: i32, new_x: i32, new_y: i32) -> bool {
        if !Grid::pos_in_world(new_x, new_y) || !Grid::pos_in_world(x, y) {
            return false;
        }

        let temp = self.particles[new_y as usize][new_x as usize].clone();
        self.particles[new_y as usize][new_x as usize] =
            self.particles[y as usize][x as usize].clone();
        self.particles[y as usize][x as usize] = temp;
        true
    }

    // pub fn swap(&mut self, x: i32, y: i32, new_x: i32, new_y: i32) -> bool {
    //     if !Grid::pos_in_world(new_x, new_y) || !Grid::pos_in_world(x, y) {
    //         false;
    //     }
    //     let temp = self.particles[new_y as usize][new_x as usize];
    //     self.particles[new_y as usize][new_x as usize] = self.particles[y as usize][x as usize];
    //     self.particles[y as usize][x as usize] = temp;
    //     true
    // }

    // pub fn fall

    // probably some helpers needed
    // get neighborhood, etc
    // check up, down, left, right, etc
}
