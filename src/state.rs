use libm::{ceilf, cosf, fabsf, floorf, sinf, sqrtf, tanf};
use core::f32::consts::{FRAC_PI_2, PI};
use crate::constants::{FRAME_WIDTH, SCREEN_SIZE};
use crate::map::{Orientation, read_map, Terrain};

const STEP_SIZE: f32 = 0.045;
const GRAVITATIONAL_ACCELERATION: f32 = 6.0;
const INITIAL_JUMP_SPEED: f32 = 3.0;

const FOV: f32 = PI / 2.7; // Spelarens synsfelt
const HALF_FOV: f32 = FOV * 0.5; // Halve spelarens synsfelt
const ANGLE_STEP: f32 = FOV / (SCREEN_SIZE as f32); // Vinkelen mellom kvar stråle
const WALL_HEIGHT: f32 = 100.0; // Eit magisk tal?

pub enum View {
    FirstPerson,
    MapView,
}

pub struct State {
    pub view: View,
    pub player_x: f32,
    pub player_y: f32,
    pub player_z: f32,
    pub player_velocity: f32,
    pub player_z_velocity: f32,
    pub player_angle: f32,
    pub player_angular_velocity: f32,
    pub previous_gamepad: u8,
}

fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}

impl State {
    /// Flytter spelaren
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool, jump: bool) {
        // lagre noverandre posisjon i det høvet vi treng han seinare
        let previous_position = (self.player_x, self.player_y);

        if self.player_z == 0.0 && !jump {
            self.player_velocity = STEP_SIZE * up as i32 as f32 - STEP_SIZE * down as i32 as f32;
            self.player_angular_velocity = STEP_SIZE * left as i32 as f32 - STEP_SIZE * right as i32 as f32;
        }

        self.player_x += cosf(self.player_angle) * self.player_velocity;
        self.player_y += -sinf(self.player_angle) * self.player_velocity;
        self.player_angle += self.player_angular_velocity;

        match read_map(self.player_x, self.player_y) {
            Terrain::Open => {},
            Terrain::Wall => {
                if read_map(self.player_x, previous_position.1) == Terrain::Open {
                    self.player_y = previous_position.1;
                } else if read_map(previous_position.0, self.player_y) == Terrain::Open {
                    self.player_x = previous_position.0;
                } else {
                    self.player_x = previous_position.0;
                    self.player_y = previous_position.1;
                }
            },
            Terrain::Doorway => {
                self.player_x = previous_position.0;
                self.player_y = previous_position.1;
            },
        }

        if jump && self.player_z == 0.0 {
            self.player_z_velocity = INITIAL_JUMP_SPEED;
        }

        if self.player_z > 0.0 {
            self.player_velocity *= 0.975;
            self.player_angular_velocity *= 0.975;
        }

        self.player_z += self.player_z_velocity * FRAME_WIDTH;
        self.player_z_velocity -= GRAVITATIONAL_ACCELERATION * FRAME_WIDTH;
        if self.player_z <= 0.0 {
            self.player_z = 0.0;
            self.player_z_velocity = 0.0;
        }
    }

    /// Gjev tilbake næraste vegg som ei stråle skjer langsetter ei horisontal linje
    fn horizontal_intersection(&self, angle: f32) -> Ray {
        // Seier om vinkelen peikar nordover (i det heile)
        let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

        // first_x og first_y er dei første skjeringspunkt mellom stråle og gitter
        let first_y = if up {
            ceilf(self.player_y) - self.player_y
        } else {
            floorf(self.player_y) - self.player_y
        };
        let first_x = -first_y / tanf(angle);

        // dy og dx er «stråleforlenginga»
        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / tanf(angle);

        // next_x og next_y held styr på kor langt strålen er frå spelaren
        let mut next_x = first_x;
        let mut next_y = first_y;
        let mut terrain: Terrain;

        // Lykkje kor strålen forlengast til han når ein vegg
        for _ in 0..256 {
            // current_x og current_y er strålens noverande posisjon
            let current_x = next_x + self.player_x;
            let current_y = if up {
                next_y + self.player_y
            } else {
                next_y + self.player_y - 1.0
            };

            // Lykkja stoggar når strålen kjem til ein vegg
            terrain = read_map(current_x, current_y);
            if terrain == Terrain::Wall {
                return Ray {
                    angle_diff: angle - self.player_angle,
                    distance: distance(next_x, next_y),
                    terrain: Terrain::Wall,
                    orientation: Orientation::Horizontal,
                }
            }
            if terrain != Terrain::Open {
                return Ray {
                    angle_diff: angle - self.player_angle,
                    distance: 100.0,
                    terrain: Terrain::Wall,
                    orientation: Orientation::Horizontal,
                }
            }

            // forleng strålen så lenge me ikkje har nådd ein vegg
            next_x += dx;
            next_y += dy;
        }

        // gje tilbake avstanden fra (next_x, next_y) til spelarens posisjon
        Ray {
            angle_diff: angle - self.player_angle,
            distance: distance(next_x, next_y),
            terrain: Terrain::Wall,
            orientation: Orientation::Horizontal,
        }
    }

    /// Gjev tilbake næraste vegg som ei stråle skjer langsetter ei vertikal linje
    fn vertical_intersection(&self, angle: f32) -> Ray {
        // Seier om vinkelen peikar nordover (i det heile)
        let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

        // first_x og first_y er dei første skjeringspunkt mellom stråle og gitter
        let first_x = if right {
            ceilf(self.player_x) - self.player_x
        } else {
            floorf(self.player_x) - self.player_x
        };
        let first_y = -tanf(angle) * first_x;

        // dy og dx er «stråleforlenginga»
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -tanf(angle);

        // next_x og next_y held styr på kor langt strålen er frå spelaren
        let mut next_x = first_x;
        let mut next_y = first_y;
        let mut terrain: Terrain;

        // Lykkje kor strålen forlengast til han når ein vegg
        for _ in 0..256 {
            // current_x og current_y er strålens noverande posisjon
            let current_x = if right {
                next_x + self.player_x
            } else {
                next_x + self.player_x - 1.0
            };
            let current_y = next_y + self.player_y;

            // Lykkja stoggar når strålen kjem til ein vegg
            terrain = read_map(current_x, current_y);
            if terrain == Terrain::Wall {
                return Ray {
                    angle_diff: angle - self.player_angle,
                    distance: distance(next_x, next_y),
                    terrain: Terrain::Wall,
                    orientation: Orientation::Vertical,
                };
            }
            if terrain != Terrain::Open {
                return Ray {
                    angle_diff: angle - self.player_angle,
                    distance: 100.0,
                    terrain: Terrain::Wall,
                    orientation: Orientation::Vertical,
                };
            }

            // forleng strålen så lenge me ikkje har nådd ein vegg
            next_x += dx;
            next_y += dy;
        }

        // gje tilbake avstanden fra (next_x, next_y) til spelarens posisjon
        Ray {
            angle_diff: angle - self.player_angle,
            distance: distance(next_x, next_y),
            terrain: Terrain::Wall,
            orientation: Orientation::Vertical,
        }
    }

    pub fn get_rays(&self) -> [Option<Ray>; SCREEN_SIZE as usize] {

        let angle_step = FOV / SCREEN_SIZE as f32;
        let initial_angle = self.player_angle + HALF_FOV;
        
        let mut rays = [None; SCREEN_SIZE as usize];

        for (idx, ray) in rays.iter_mut().enumerate() {
            *ray = Some(self.raycast(initial_angle - idx as f32 * angle_step))
        }

        rays
    }

    fn raycast(&self, angle: f32) -> Ray {
        let vertical_intersection = self.vertical_intersection(angle);
        let horizontal_intersection = self.horizontal_intersection(angle);

        if vertical_intersection.distance < horizontal_intersection.distance {
            vertical_intersection
        } else {
            horizontal_intersection
        }
    }
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub angle_diff: f32,
    pub distance: f32,
    pub terrain: Terrain,
    pub orientation: Orientation,
}

impl Ray {
    pub fn wall_height(&self) -> f32 {
        WALL_HEIGHT / (self.distance * cosf(self.angle_diff))
    }
}