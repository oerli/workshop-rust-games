use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::audio::SoundSource;
use ggez::timer;
use nalgebra as na;

type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

use crate::game::config::{NUM_SNACKS, CRAB_W, CRAB_H, CLAW_W, TIME};
use crate::game::assets::Assets;
use crate::game::crab::{Crab};
use crate::game::snacks::{Snack, spawn_snacks};
use crate::game::player::{Player};
use crate::game::menu::{Menu};
use crate::game::network::{Connection};

pub struct State {
    pub player1: Player,
    pub player2: Player,
    pub crab: Crab,
    pub snacks: Vec<Snack>,
    pub screen_width: f32,
    pub screen_height: f32,
    pub assets: Assets,
    pub time: u64,
    pub menu: Menu,
    pub active: bool,
    pub connection: Connection,
}

impl State {
    pub fn new(ctx: &mut Context) -> ggez::GameResult<State> {
        println!("Play Crab!");
        println!("Player 1, use WASD!");
        println!("Player 2, use IJKL!");
        println!("Have fun!");

        let assets = Assets::new(ctx)?;
        let (width, height) = ggez::graphics::drawable_size(ctx);
        let crab_origin = Point2::new(width / 2.0 - (CRAB_W / 2.0),
                                      height - CRAB_H);

        let mut s = State {
            player1: Player::new(1, crab_origin,
                                 Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                                 Vector2::new(-30., -20.))?,
            player2: Player::new(2, crab_origin,
                                 Vector2::new(CRAB_W + 30.0, CRAB_H / 2.),
                                 Vector2::new(170.0, -20.0))?,
            crab: Crab::new(crab_origin)?,
            snacks: spawn_snacks(NUM_SNACKS),
            screen_width: width,
            screen_height: height,
            assets: assets,
            // time: timer::time_since_start(ctx).as_secs() + TIME as u64,
            time: 0,
            menu: Menu::new()?,
            active: true,
            // outgoing connection
            connection: Connection::new()?,
        };
        
        s.connection.connect(&s.menu.remote, "4664");
        Ok(s)
    }

    pub fn render_ui(&mut self, ctx: &mut Context) -> GameResult<&Self> {
        // recreate all from scratch
        if timer::time_since_start(ctx).as_secs() >= self.time {
            self.menu.active = true;
        }

        let score_1 = graphics::Text::new((format!("Player 1: #{}",
                                                   self.player1.score),
                                           self.assets.font, 38.0));
        let score_2 = graphics::Text::new((format!("Player 2: #{}",
                                                   self.player2.score),
                                           self.assets.font, 38.0));
        let time = graphics::Text::new((format!("Timer: #{}",
            self.time.checked_sub(timer::time_since_start(ctx).as_secs()).unwrap_or(0)),
                                            self.assets.font, 38.0));

        let frames = graphics::Text::new((format!("FPS: #{:.0}",
                                                timer::fps(ctx)),
                                            self.assets.font, 18.0));      
        
        let red_color = graphics::Color::new(0.969, 0.292, 0., 1.);
        let green_color = graphics::Color::new(0., 0.992, 0., 1.);
        graphics::draw(ctx, &score_1, (Point2::new(10., 10.), 0., red_color))?;
        graphics::draw(ctx, &score_2, (Point2::new(self.screen_width - 180., 10.), 0., green_color))?;
        graphics::draw(ctx, &time, (Point2::new(self.screen_width / 2. - 60., 10.), 0., graphics::BLACK))?;
        graphics::draw(ctx, &frames, (Point2::new(10., self.screen_height - 40.), 0., graphics::BLACK))?;

        Ok(self)
    }

    pub fn reset_player(&mut self, ctx: &mut Context) -> GameResult<&Self> {
        let (width, height) = ggez::graphics::drawable_size(ctx);

        let crab_origin = Point2::new(width / 2. - (CRAB_W / 2.),
        height - CRAB_H);

        self.player1 = Player::new(1, crab_origin,
                                Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                                Vector2::new(-30., -20.))?;
        self.player2 = Player::new(2, crab_origin,
                                Vector2::new(CRAB_W + 30., CRAB_H / 2.),
                                Vector2::new(170., -20.))?;
        Ok(self)
    }

    pub fn reset(&mut self, ctx: &mut Context) -> GameResult<&Self> {
        self.menu.active = false;

        let (width, height) = ggez::graphics::drawable_size(ctx);

        let crab_origin = Point2::new(width / 2. - (CRAB_W / 2.),
        height - CRAB_H);

        self.player1 = Player::new(1, crab_origin,
                                Vector2::new(CLAW_W - 20., CRAB_H / 2.),
                                Vector2::new(-30., -20.))?;
        self.player2 = Player::new(2, crab_origin,
                                Vector2::new(CRAB_W + 30., CRAB_H / 2.),
                                Vector2::new(170., -20.))?;
        self.crab = Crab::new(crab_origin)?;
        self.snacks = spawn_snacks(NUM_SNACKS);

        // connect to receiving address
        self.connection.connect(&self.menu.remote, "4664");

        for snack in &self.snacks {
            snack.send(&mut self.connection)?;
        }
        self.time = timer::time_since_start(ctx).as_secs() + TIME as u64;

        self.crab.send(&mut self.connection)?;
        
        Ok(self)
    }

    pub fn collision_check(&mut self) {
        if self.menu.active == true {
            return
        }

        let c1 = self.player1.claw.get_origin();
        let c2 = self.player2.claw.get_origin();

        for s in self.snacks.iter_mut() {
            if s.collides_with(c1) {
                let _ = self.assets.snap_sound.play();
                self.player1.increase_score().expect("Could not update score!");
                self.player1.send(&mut self.connection).expect("Could not send data!");
            }
        }

        for s in self.snacks.iter_mut() {
            if s.collides_with(c2) {
                let _ = self.assets.snap_sound.play();
                self.player2.increase_score().expect("Could not update score!");
                self.player2.send(&mut self.connection).expect("Could not send data!");
            }
        }
        
    }
}
