use ggez::audio::SoundSource;
use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;
use ggez::timer;

mod config;
mod state;
mod crab;
mod player;
mod claw;
use crate::game::claw::{Directions};
mod assets;
mod snacks;
mod menu;
mod network;

pub use crate::game::state::{State};
pub use crate::game::config::{SCREEN_W, SCREEN_H, TIME, DESIRED_FPS};
pub use crate::game::network::{NetworkEvent};

type Point2 = na::Point2<f32>;

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.connection.update() {
            NetworkEvent::SnackUpdate(id, location, velocity) => {
                self.snacks[id].reset(location, velocity)?;
            },
            NetworkEvent::PlayerUpdate(id, score, anchor) => {
                match id {
                    1 => {
                        self.player1.claw.joint_anchor = anchor;
                        self.player1.score = score;
                    },
                    2 => {
                        self.player2.claw.joint_anchor = anchor;
                        self.player2.score = score;
                    },
                    _ => {},
                }
            },
            NetworkEvent::GameUpdate(address, location, velocity) => {
                self.crab.location = location;
                self.crab.velocity = velocity;
                self.reset_player(ctx)?;
                self.menu.active = false;
                self.active = true;
                self.time = timer::time_since_start(ctx).as_secs() + TIME as u64;
                // connect to receiving address
                self.menu.remote = address;
                self.connection.connect(&self.menu.remote, "4664");
            },
            NetworkEvent::None => {}
        }
        
        while timer::check_update_time(ctx, DESIRED_FPS) {
            
            for s in self.snacks.iter_mut() {
                s.update(&mut self.connection)?;
            }
            self.crab.update(self.screen_width)?;
            self.player1.update(self.crab.location)?;
            self.player2.update(self.crab.location)?;
            self.collision_check();
        }

        if !self.assets.bg_sound.playing() {
            let _  = self.assets.bg_sound.play();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        
        let draw_params = graphics::DrawParam::new().dest(Point2::new(0., 0.));
        graphics::draw(ctx, &self.assets.bg_image, draw_params)?;

        let time = timer::time_since_start(ctx).as_secs_f32()/0.2;
        for (i, s) in self.snacks.iter().enumerate() {
            match ((time * s.velocity[1]) as usize + i) % 4 {
                0 => {s.draw(ctx, &self.assets.snack_image1)?;},
                1 => {s.draw(ctx, &self.assets.snack_image2)?;},
                2 => {s.draw(ctx, &self.assets.snack_image3)?;},
                3 => {s.draw(ctx, &self.assets.snack_image2)?;},
                _ => {s.draw(ctx, &self.assets.snack_image1)?;},
            }
            
        }
        self.crab.draw(ctx, &self.assets.crab_image)?;
        self.player1.draw(ctx, &self.assets.claw_left)?;
        self.player2.draw(ctx, &self.assets.claw_right)?;
        self.menu.draw(ctx, &self.assets)?;

        self.render_ui(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if self.menu.select(x, y) {
            self.reset(ctx).expect("Could not reset game state");
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        // Match Keycode for each player
        // if just one match statement used, Keycode for second player won't be considered
        if self.menu.active == false {
            match keycode {
                KeyCode::W => {
                    self.player1.movedir(Directions::Up);
                    self.player1.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                KeyCode::A => {
                    self.player1.movedir(Directions::Left);
                    self.player1.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                KeyCode::S => {
                    self.player1.movedir(Directions::Down);
                    self.player1.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                KeyCode::D => {
                    self.player1.movedir(Directions::Right);
                    self.player1.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                _ => {}
            }
            match keycode {
                KeyCode::I => {
                    self.player2.movedir(Directions::Up);
                    self.player2.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                KeyCode::J => {
                    self.player2.movedir(Directions::Left);
                    self.player2.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                KeyCode::K => {
                    self.player2.movedir(Directions::Down);
                    self.player2.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                KeyCode::L => {
                    self.player2.movedir(Directions::Right);
                    self.player2.send(&mut self.connection).expect("Could not send keystroke to participant!");
                },
                _ => {}
            }
        } else {
            match keycode {
                KeyCode::Key0 => self.menu.remote.push_str("0"),
                KeyCode::Key1 => self.menu.remote.push_str("1"),
                KeyCode::Key2 => self.menu.remote.push_str("2"),
                KeyCode::Key3 => self.menu.remote.push_str("3"),
                KeyCode::Key4 => self.menu.remote.push_str("4"),
                KeyCode::Key5 => self.menu.remote.push_str("5"),
                KeyCode::Key6 => self.menu.remote.push_str("6"),
                KeyCode::Key7 => self.menu.remote.push_str("7"),
                KeyCode::Key8 => self.menu.remote.push_str("8"),
                KeyCode::Key9 => self.menu.remote.push_str("9"),
                KeyCode::Period => self.menu.remote.push_str("."),
                KeyCode::Back => {self.menu.remote.pop();},
                _ => {}
            }
        }
    }
}
