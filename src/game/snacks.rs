use ggez::{Context, GameResult};
use ggez::graphics;
use rand;
use nalgebra as na;
use crate::game::network::{Connection, NetworkEvent};

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{SCREEN_H, SCREEN_W, SNACK_W};

pub struct Snack {
    id: usize,
    location: Point2,
    pub velocity: Vector2,
    w: f32,
    active: bool
}

impl Snack {
    fn new (id: usize) -> GameResult<Snack> {
        let s = Snack {
            id: id,
            location: Point2::new(rand::random::<f32>() * SCREEN_W,
                                  rand::random::<f32>() * SCREEN_H - SCREEN_H),
            velocity: Vector2::new(0.0,
                                   rand::random::<f32>() * 2.0 + 0.1),
            w: SNACK_W,
            active: true
        };
        Ok(s)
    }

    pub fn update(&mut self, connection: &mut Connection) -> GameResult<&Self> {
        // Move the Snack down
        self.location = self.location + self.velocity;

        // Remove the Snack from the Screen when it's outside
        if self.location.y > SCREEN_H {
            self.active = false;
        }

        // Recreate inactive Snack
        if self.active == false {
            self.location = Point2::new(rand::random::<f32>() * SCREEN_W, -SNACK_W);
            self.velocity = Vector2::new(0., rand::random::<f32>() * 2. + 0.1);
            self.active = true;
            connection.send(NetworkEvent::SnackUpdate(self.id, self.location, self.velocity))?;
        }

        Ok(self)
    }

    pub fn send(&self, connection: &mut Connection) -> GameResult<&Self> {
        connection.send(NetworkEvent::SnackUpdate(self.id, self.location, self.velocity))?;
        Ok(self)
    }

    pub fn reset(&mut self, location: Point2, velocity: Vector2) -> GameResult<&Self> {
        self.location = location;
        self.velocity = velocity;
        self.active = true;
        
        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        if self.active == true {
            let draw_params = graphics::DrawParam::new().dest(self.location).scale(Vector2::new(0.3, 0.3));
            graphics::draw(ctx, img, draw_params)?;

        }
        Ok(self)
    }

    pub fn collides_with(&mut self, other: Point2) -> bool {
        if self.active == true {
            let distance = self.location - other;
            if distance.norm() < self.w {
                self.active = false;
                // Here return needs to be explicit
                return true
            }
        }
        false
    }
}

pub fn spawn_snacks(num: usize) -> Vec<Snack> {
    (0..num).map(|v| Snack::new(v).expect("could not create snack")).collect()
}