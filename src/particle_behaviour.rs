/*
    Note: in this code, i use the following unique idiom:
        do_thing() || if_cant_then_do_this() || another_fallback() ...
    to chain together conditional particle behaviours.
    Generally I would use if/elseif/else but this "short-circuiting" pattern is particularly clean in this code.

    add gel:
*/

use crate::{
    element::Element,
    particle::Particle,
    settings::{HEIGHT, WIDTH},
};
use rand::Rng;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn spawn_particle(
    x: i32,
    y: i32,
    element: Element,
    particles: &mut Vec<Particle>,
    next_particle_id: &mut u32,
    grid: &mut Vec<Vec<Option<usize>>>,
) {
    if !pos_in_world(x, y) || grid[y as usize][x as usize].is_some() {
        return;
    }

    let particle = Particle::new(x, y, element, *next_particle_id);
    *next_particle_id += 1;
    particles.push(particle);
    grid[y as usize][x as usize] = Some(particle.id as usize);
}

pub fn pos_in_world(x: i32, y: i32) -> bool {
    x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT
}

pub fn on_floor(p: &Particle) -> bool {
    p.y == HEIGHT - 1
}

pub fn pmove(p: &mut Particle, grid: &mut Vec<Vec<Option<usize>>>, x: i32, y: i32) {
    grid[p.y as usize][p.x as usize] = None;
    p.x = x;
    p.y = y;
    grid[p.y as usize][p.x as usize] = Some(p.id as usize);
}

pub fn ptry_move(p: &mut Particle, grid: &mut Vec<Vec<Option<usize>>>, x: i32, y: i32) -> bool {
    if pos_in_world(x, y) && grid[y as usize][x as usize].is_none() {
        pmove(p, grid, x, y);
        true
    } else {
        false
    }
}

/// try to move down, if cant, try to move left or right
pub fn ptry_fall(p: &mut Particle, grid: &mut Vec<Vec<Option<usize>>>) -> bool {
    on_floor(p) || ptry_move(p, grid, p.x, p.y + 1) || {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2);
        if direction == 0 {
            ptry_move(p, grid, p.x - 1, p.y + 1)
        } else {
            ptry_move(p, grid, p.x + 1, p.y + 1)
        }
    }
}

/// ptry_fall but up
pub fn ptry_fall_up(p: &mut Particle, grid: &mut Vec<Vec<Option<usize>>>) -> bool {
    on_floor(p) || ptry_move(p, grid, p.x, p.y - 1) || {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2);
        if direction == 0 {
            ptry_move(p, grid, p.x - 1, p.y + 1)
        } else {
            ptry_move(p, grid, p.x + 1, p.y + 1)
        }
    }
}

/// like try fall but just for left right
pub fn ptry_jitter_lr(p: &mut Particle, grid: &mut Vec<Vec<Option<usize>>>) -> bool {
    let mut rng = rand::thread_rng();
    let direction = rng.gen_range(0..2);
    if direction == 0 {
        ptry_move(p, grid, p.x - 1, p.y)
    } else {
        ptry_move(p, grid, p.x + 1, p.y)
    }
}

/// same as try_jitter but in all 8 directions
pub fn ptry_expandjitter(p: &mut Particle, grid: &mut Vec<Vec<Option<usize>>>) -> bool {
    let mut rng = rand::thread_rng();
    let direction = rng.gen_range(0..8);
    let (x, y) = DIRECTIONS[direction];
    ptry_move(p, grid, p.x + x, p.y + y)
}

pub fn step_particles(
    particles: &mut Vec<Particle>,
    grid: &mut Vec<Vec<Option<usize>>>,
    next_particle_id: &mut u32,
    new_particles: &mut Vec<Particle>,
) {
    for p in particles.iter_mut() {
        p.age += 1;
        match p.element {
            Element::Sand => {
                /* sand tries to go down, if it cant, it randomly goes left or right */
                ptry_fall(p, grid);
            }
            Element::Water => {
                /* water tries to fall, if it cant, randomly try left and right */
                let _ = ptry_fall(p, grid) || ptry_jitter_lr(p, grid);
            }
            Element::Gas => {
                /* Update gas behavior */
                let _ = ptry_expandjitter(p, grid);
            }
            Element::Fire => {
                /* Update fire behavior */
                let _ = ptry_move(p, grid, p.x, p.y - 1);
                if p.age > 8 {
                    p.age = -1;
                    p.remove = true;
                }
                let mut rng = rand::thread_rng();
                let chance = rng.gen_range(0..4);
                if chance == 0 {
                    // ptry_spawn(grid, p.x, p.y - 1, Element::Smoke, &mut next_particle_id);
                    spawn_particle(
                        p.x,
                        p.y - 1,
                        Element::Smoke,
                        new_particles,
                        next_particle_id,
                        grid,
                    );
                }
            }
            Element::Smoke => {
                /* Update smoke behavior */
                let _ = ptry_fall_up(p, grid);
                if p.age > 120 {
                    p.age = -1;
                    p.remove = true;
                }
            }
            Element::Steam => { /* Update steam behavior */ }
            Element::Wood => { /* Update wood behavior */ }
            Element::Wall => { /* Update wall behavior */ }
            Element::Ice => { /* Update ice behavior */ }
        }
    }
}
