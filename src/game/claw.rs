use ggez::graphics;
use ggez::{Context, GameResult};
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{CLAW_W, CLAW_H, CLAW_S, SCREEN_W, SCREEN_H};

pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub struct Claw {
    pub location: Point2,
    body_anchor: Vector2,
    pub joint_anchor: Vector2,
    w: f32,
    h: f32,
    s: f32
}

impl Claw {
    pub fn new(location: Point2,
               body_anchor: Vector2,
               joint_anchor: Vector2) -> GameResult<Claw> {
        let c = Claw {
            location,
            body_anchor,
            joint_anchor,
            w: CLAW_W,
            h: CLAW_H,
            s: CLAW_S
        };
        Ok(c)
    }

    pub fn update(&mut self, parent_loc: Point2) -> GameResult<&Self> {
        self.location = parent_loc;
        
        // update joint_anchor relative to the location of crab
        if self.joint_anchor.y + self.location.y - self.h < 0. {
            self.joint_anchor.y = - self.location.y + self.h;
        } else if self.joint_anchor.y + self.location.y + self.h / 2. > SCREEN_H {
            self.joint_anchor.y = SCREEN_H - self.location.y - self.h / 2.;
        }

        if self.joint_anchor.x + self.location.x - self.w < 0. {
            self.joint_anchor.x = - self.location.x + self.w;
        } else if self.joint_anchor.x + self.location.x + self.w / 2. > SCREEN_W {
            self.joint_anchor.x = SCREEN_W - self.location.x - self.w / 2.;
        }

        Ok(self)
    }

    pub fn draw(&self, ctx: &mut Context, img: &graphics::Image) -> GameResult<&Self> {
        let red_color = graphics::Color::new(0.969, 0.298, 0., 1.);
        
        let body_location = self.location + self.body_anchor;
        let joint_location = self.location + self.joint_anchor;

        let arm = graphics::Mesh::new_line(ctx, &[body_location, joint_location], 10., red_color)?;
        graphics::draw(ctx, &arm, graphics::DrawParam::default())?;

        let draw_params = graphics::DrawParam::new().dest(self.get_origin()).scale(Vector2::new(0.2, 0.2));
        graphics::draw(ctx, img, draw_params)?;
        Ok(self)
    }

    pub fn get_origin(&self) -> Point2 {
        let joint_position = self.location + self.joint_anchor;
        let x = joint_position.x - self.w / 2.;
        let y = joint_position.y - self.h;
        Point2::new(x, y)
    }

    pub fn movedir(&mut self, dir:Directions) -> Vector2 {
        match dir {
            Directions::Up => {
                self.joint_anchor.y = self.joint_anchor.y - self.s;
            },
            Directions::Down => {
                self.joint_anchor.y = self.joint_anchor.y + self.s;
            },
            Directions::Left => {
                self.joint_anchor.x = self.joint_anchor.x - self.s;
            },
            Directions::Right => {
                self.joint_anchor.x = self.joint_anchor.x + self.s;
            }
        }
        self.joint_anchor
    }
}
