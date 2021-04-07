use ggez::{Context, GameResult};
use ggez::graphics;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::network::{Connection, NetworkEvent};
use crate::game::config::{CRAB_W,
                          CRAB_S};

pub struct Crab {
    pub location: Point2,
    pub velocity: Vector2,
    w: f32,
    s: f32
}

impl Crab {
    pub fn new(location: Point2) -> GameResult<Crab> {
        let c = Crab {
            location,
            velocity: Vector2::new(CRAB_S, 0.0),
            w: CRAB_W,
            s: CRAB_S
        };
        Ok(c)
    }

    pub fn update(&mut self, max_screen: f32) -> GameResult<&Self> {
        // change direction of crab
        if self.location.x + (self.w * 2.) >= max_screen {
            self.velocity.x = -self.s;
        } else if self.location.x < self.w {
            self.velocity.x = self.s
        }
        // move crab
        self.location.x = self.location.x + self.velocity.x;

        Ok(self)
    }

    pub fn send(&self, connection: &mut Connection) -> GameResult<&Self> {
        connection.send(NetworkEvent::GameUpdate("127.0.0.1".to_owned() , self.location, self.velocity))?;
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let draw_params = graphics::DrawParam::new().dest(self.location).scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, draw_params)?;
        Ok(self)
    }
}
