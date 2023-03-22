/*
    Note: in this code, i use the following unique idiom:
        do_thing() || if_cant_then_do_this() || another_fallback() ...
    to chain together conditional particle behaviours.
    Generally I would use if/elseif/else but this "short-circuiting" pattern is particularly clean in this code.

    TODO:
        - try to add a high viscosity gel
        - wood
    //NOTE: try the behaviour as struct for encapsulating grid and particle
    CONSIDER:
        -
*/

use crate::{element::Element, grid::Grid, particle::Particle};
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

pub fn on_floor(y: i32) -> bool {
    y == Grid::HEIGHT - 1
}

pub fn on_ceiling(y: i32) -> bool {
    y == 0
}

pub fn set_if_empty(grid: &mut Grid, x: i32, y: i32, p: Particle) {
    if let Some(tp) = grid.get(x, y) {
        if tp.element == Element::Air {
            grid.set(x, y, p);
        }
    }
}

pub fn try_move(grid: &mut Grid, x: i32, y: i32, new_x: i32, new_y: i32) -> bool {
    if let (Some(p), Some(tp)) = (grid.get(x, y), grid.get(new_x, new_y)) {
        // if tp is wall, dont move
        if tp.element == Element::Wall {
            return false;
        } else if tp.element == Element::Air {
            grid.swap(x, y, new_x, new_y);
            return true;
        }

        if y == new_y {
            if p.density() >= tp.density() {
                grid.swap(x, y, new_x, new_y);
                return true;
            }
        } else if new_y > y {
            if p.density() > tp.density() {
                grid.swap(x, y, new_x, new_y);
                return true;
            }
        } else if new_y < y {
            if p.density() < tp.density() {
                grid.swap(x, y, new_x, new_y);
                return true;
            }
        }
    }
    false
}

/// Try to move down, if cant, try to move left-down or right-down.
pub fn fall(grid: &mut Grid, x: i32, y: i32) -> bool {
    on_floor(y) || try_move(grid, x, y, x, y + 1) || {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2);
        if direction == 0 {
            try_move(grid, x, y, x - 1, y + 1)
        } else {
            try_move(grid, x, y, x + 1, y + 1)
        }
    }
}

/// Like fall but up instead of down.
pub fn fall_up(grid: &mut Grid, x: i32, y: i32) -> bool {
    on_ceiling(y) || try_move(grid, x, y, x, y - 1) || {
        let mut rng = rand::thread_rng();
        let direction = rng.gen_range(0..2);
        if direction == 0 {
            try_move(grid, x, y, x - 1, y - 1)
        } else {
            try_move(grid, x, y, x + 1, y - 1)
        }
    }
}

/// Like try fall but just for left right.
pub fn jitter_left_right(grid: &mut Grid, x: i32, y: i32) -> bool {
    let mut rng = rand::thread_rng();
    let direction = rng.gen_range(0..2);
    if direction == 0 {
        try_move(grid, x, y, x - 1, y)
    } else {
        try_move(grid, x, y, x + 1, y)
    }
}

/// Same as try_jitter but in all 8 directions.
pub fn expand_jitter(grid: &mut Grid, x: i32, y: i32) -> bool {
    let mut rng = rand::thread_rng();
    let direction = rng.gen_range(0..8);
    let (dx, dy) = DIRECTIONS[direction];
    try_move(grid, x, y, x + dx, y + dy)
}

/// Check for nearby collision.
pub fn check_nearby_for(grid: &mut Grid, x: i32, y: i32, element: Element) -> bool {
    /* this function can only check for presence, but we may want to also modify the found one, or return it to read more properties in
    a different version of the find function */
    for (dx, dy) in DIRECTIONS.iter() {
        let cx = x + dx;
        let cy = y + dy;
        let p = grid.get(cx, cy);
        if let Some(p) = p {
            if p.element == element {
                return true;
            }
        }
    }
    false
}

pub fn step_particles(grid: &mut Grid, frame_clock: u32) {
    for y in 0..Grid::HEIGHT {
        for x in 0..Grid::WIDTH {
            {
                let p = grid.get_mut(x, y);
                if let Some(p) = p {
                    if p.last_ticked == frame_clock {
                        continue;
                    } else {
                        if p.lifetime() > 0 && p.age > p.lifetime() {
                            grid.set(x, y, Particle::new(Element::Air, frame_clock));
                            continue;
                        }
                        p.last_ticked = frame_clock;
                        p.age += 1;
                    }
                }
                if let Some(p) = grid.get(x, y) {
                    match p.element {
                        Element::Air => { /*  do nothing */ }
                        Element::Sand => {
                            fall(grid, x, y);
                        }
                        Element::Oil => {
                            let _ = fall(grid, x, y) || jitter_left_right(grid, x, y);
                        }
                        Element::Water => {
                            if check_nearby_for(grid, x, y, Element::Fire)
                                || check_nearby_for(grid, x, y, Element::Lava)
                            {
                                grid.set(x, y, Particle::new(Element::Steam, frame_clock));
                            }
                            let _ = fall(grid, x, y) || jitter_left_right(grid, x, y);
                        }
                        Element::Fire => {
                            let _ = fall_up(grid, x, y) || jitter_left_right(grid, x, y);
                            let mut rng = rand::thread_rng();
                            let chance = rng.gen_range(0..16);
                            if chance == 0 {
                                grid.set(x, y, Particle::new(Element::Smoke, frame_clock));
                            }
                        }
                        Element::Smoke => {
                            let _ = fall_up(grid, x, y) || jitter_left_right(grid, x, y);
                        }
                        Element::Steam => {
                            if check_nearby_for(grid, x, y, Element::Ice) {
                                grid.set(x, y, Particle::new(Element::Water, frame_clock));
                            }
                            let _ = fall_up(grid, x, y) || jitter_left_right(grid, x, y);
                        }
                        Element::Wood => {
                            // if check_nearby_for(
                            //     Element::Fire,
                            //     p,
                            //     &particles[0..i],
                            //     &particles[i + 1..],
                            //     grid,
                            // ) {
                            //     let p = &mut particles[i];
                            //     p.remove = true;
                            //     try_spawn_particle(
                            //         p.x,
                            //         p.y,
                            //         Element::Fire,
                            //         new_particles,
                            //         grid,
                            //         next_particle_id,
                            //     );
                            // }
                        }
                        Element::Wall => { /* Update wall behavior */ }
                        Element::Ice => {
                            fall(grid, x, y);
                        }
                        Element::Lava => {
                            let _ = fall(grid, x, y) || jitter_left_right(grid, x, y);
                            let mut rng = rand::thread_rng();
                            let chance = rng.gen_range(0..16);
                            if chance == 0 {
                                set_if_empty(
                                    grid,
                                    x,
                                    y - 1,
                                    Particle::new(Element::Fire, frame_clock),
                                );
                            }
                        }
                    }
                    let p = grid.get_mut(x, y);
                    if let Some(p) = p {
                        // p.age += 1;
                        // if p.lifetime() > 0 && p.age >= p.lifetime() {
                        //     p.remove = true;
                        // }
                    }
                }
            }
        }
    }

    //  implement aging generically
    // p.age += 1;
    // if p.lifetime() > 0 && p.age >= p.lifetime() {
    //     p.remove = true;
    // }
}
