// save-the-ship - simple game with ai
//     Copyright (C) 2026  bl1nd

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as
//     published by the Free Software Foundation, either version 3 of the
//     License, or (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use raylib::prelude::*;
use rand::{RngExt, rng};
use std::f32::consts::PI;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

const SHIP_RADIUS: f32 = 18.0;
const SHIP_SPEED: f32 = 300.0;

const ASTEROID_RADIUS: f32 = 12.0;
const BASE_SPAWN_RATE: f32 = 1.0;

const RAY_LEN: f32 = 180.0;
const NUM_RAYS: usize = 1024;

const SAFE_RADIUS: f32 = 250.0;

#[derive(Clone)]
struct Asteroid {
    pos: Vector2,
    vel: Vector2,
}

#[derive(PartialEq)]
enum Mode {
    Manual,
    AI,
}

struct GameState {
    ship_pos: Vector2,
    asteroids: Vec<Asteroid>,
    time_alive: f32,
    spawn_timer: f32,
    game_over: bool,
    mode: Mode,
    show_rays: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            ship_pos: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
            asteroids: vec![],
            time_alive: 0.0,
            spawn_timer: 0.0,
            game_over: false,
            mode: Mode::Manual,
            show_rays: false,
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Save The Ship")
        .build();

    rl.set_target_fps(60);

    let mut state = GameState::new();
    let mut rng = rng();

    let center = Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            state = GameState::new();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            state.mode = if state.mode == Mode::Manual { Mode::AI } else { Mode::Manual };
        }

        if rl.is_key_pressed(KeyboardKey::KEY_H) {
            state.show_rays = !state.show_rays;
        }

        if !state.game_over {
            state.time_alive += dt;

            match state.mode {
                Mode::Manual => {
                    let mut movement = Vector2::zero();

                    if rl.is_key_down(KeyboardKey::KEY_W) { movement.y -= 1.0; }
                    if rl.is_key_down(KeyboardKey::KEY_S) { movement.y += 1.0; }
                    if rl.is_key_down(KeyboardKey::KEY_A) { movement.x -= 1.0; }
                    if rl.is_key_down(KeyboardKey::KEY_D) { movement.x += 1.0; }

                    if movement.length() > 0.0 {
                        state.ship_pos += movement.normalized() * SHIP_SPEED * dt;
                    }
                }

                Mode::AI => {
                    let mut avoidance = Vector2::zero();
                    let mut strongest = Vector2::zero();
                    let mut closest_dist = f32::MAX;

                    const DANGER_RADIUS: f32 = RAY_LEN;

                    for asteroid in &state.asteroids {
                        let to_ship = state.ship_pos - asteroid.pos;
                        let dist = to_ship.length();

                        if dist < DANGER_RADIUS && dist > 1.0 {
                            let dir = to_ship.normalized();
                            let strength = (DANGER_RADIUS - dist) / DANGER_RADIUS;

                            avoidance += dir * strength;

                            if dist < closest_dist {
                                closest_dist = dist;
                                strongest = dir * strength;
                            }
                        }
                    }

                    let mut velocity;

                    if closest_dist < DANGER_RADIUS {
                        velocity = strongest * SHIP_SPEED * 1.5 + avoidance * SHIP_SPEED;
                    } else {
                        velocity = Vector2::new(60.0, -40.0);
                    }

                    let to_center = center - state.ship_pos;
                    let center_bias = to_center * 0.5;

                    velocity += center_bias;

                    if velocity.length() > SHIP_SPEED {
                        velocity = velocity.normalized() * SHIP_SPEED;
                    }

                    state.ship_pos += velocity * dt;
                }
            }

            let offset = state.ship_pos - center;
            let dist = offset.length();

            if dist > SAFE_RADIUS {
                state.ship_pos = center + offset.normalized() * SAFE_RADIUS;
            }

            let difficulty = 1.0 + state.time_alive * 0.25;

            state.spawn_timer -= dt;
            if state.spawn_timer <= 0.0 {
                state.spawn_timer = BASE_SPAWN_RATE / difficulty;

                let side = rng.random_range(0..4);

                let (pos, dir) = match side {
                    0 => (Vector2::new(0.0, rng.random_range(0.0..SCREEN_HEIGHT as f32)), Vector2::new(1.0, rng.random_range(-0.5..0.5))),
                    1 => (Vector2::new(SCREEN_WIDTH as f32, rng.random_range(0.0..SCREEN_HEIGHT as f32)), Vector2::new(-1.0, rng.random_range(-0.5..0.5))),
                    2 => (Vector2::new(rng.random_range(0.0..SCREEN_WIDTH as f32), 0.0), Vector2::new(rng.random_range(-0.5..0.5), 1.0)),
                    _ => (Vector2::new(rng.random_range(0.0..SCREEN_WIDTH as f32), SCREEN_HEIGHT as f32), Vector2::new(rng.random_range(-0.5..0.5), -1.0)),
                };

                let vel = dir.normalized() * (120.0 + difficulty * 60.0);
                state.asteroids.push(Asteroid { pos, vel });
            }

            for asteroid in &mut state.asteroids {
                asteroid.pos += asteroid.vel * dt;
            }

            for asteroid in &state.asteroids {
                if state.ship_pos.distance_to(asteroid.pos) < ASTEROID_RADIUS + SHIP_RADIUS {
                    state.game_over = true;
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        d.draw_circle_lines(center.x as i32, center.y as i32, SAFE_RADIUS, Color::BLUE);

        for asteroid in &state.asteroids {
            d.draw_circle_v(asteroid.pos, ASTEROID_RADIUS, Color::RED);
        }

        if state.show_rays {
            for i in 0..NUM_RAYS {
                let angle = (i as f32 / NUM_RAYS as f32) * 2.0 * PI;
                let dir = Vector2::new(angle.cos(), angle.sin());
                let end = state.ship_pos + dir * RAY_LEN;

                d.draw_line_v(state.ship_pos, end, Color::YELLOW);
            }
        }

        d.draw_circle_v(state.ship_pos, SHIP_RADIUS, Color::GREEN);

        d.draw_text(&format!("Time: {:.2}", state.time_alive), 10, 10, 20, Color::WHITE);

        let mode_text = match state.mode {
            Mode::Manual => "MODE: MANUAL",
            Mode::AI => "MODE: AI",
        };

        d.draw_text(mode_text, 10, 40, 20, Color::YELLOW);
        d.draw_text("SPACE: Toggle AI | H: Toggle Rays | R: Restart", 10, 70, 18, Color::GRAY);

        if state.game_over {
            d.draw_text("GAME OVER", 400, 400, 30, Color::YELLOW);
        }
    }
}
