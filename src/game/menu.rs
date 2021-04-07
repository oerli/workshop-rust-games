use ggez::{Context, GameResult};
use ggez::graphics;
use crate::game::assets::Assets;
use nalgebra as na;
use std::net::{IpAddr};

type Point2 = na::Point2<f32>;

use crate::game::config::{SCREEN_H, SCREEN_W};

pub struct Menu {
    location: Point2,
    width: u32,
    height: u32,
    pub active: bool,
    pub remote: String,
}

impl Menu {
    pub fn new () -> GameResult<Menu> {
        let m = Menu {
            location: Point2::new(SCREEN_W / 2. - 40., SCREEN_H / 2. - 10.),
            width: 0,
            height: 0,
            active: true,
            remote: "127.0.0.1".to_owned(),
        };
        
        Ok(m)
    }


    pub fn draw(&mut self, ctx: &mut Context, assets: &Assets) -> GameResult<&Self> {
        if self.active == true {
            
            let item = graphics::Text::new((format!("Start"), assets.font, 38.0));
            self.width = item.width(ctx);
            self.height = item.height(ctx);
            graphics::draw(ctx, &item, (self.location, 0., graphics::BLACK))?;

            let item = graphics::Text::new((format!("{}", self.remote), assets.font, 38.0));
            let address_width = item.width(ctx) as f32;
            graphics::draw(ctx, &item, (Point2::new(SCREEN_W / 2. - address_width / 2., SCREEN_H / 2. - 40.), 0., graphics::BLACK))?;
        }
        Ok(self)
    }

    pub fn select(&mut self, x: f32, y: f32) -> bool {
        if self.active == true {
            if self.location.x < x && self.location.x + (self.width as f32) > x && 
                self.location.y < y && self.location.y + (self.height as f32) > y {
                // check if ip is valid
                match self.remote.parse() {
                    Ok(IpAddr::V4(_ip)) => {
                        self.active = false;
                        return true
                    },
                    Ok(IpAddr::V6(_ip)) => {
                        self.active = false;
                        return true
                    },
                    Err(_e) => {
                        return false
                    }
                }
            }
        }
        return false
    }
}
