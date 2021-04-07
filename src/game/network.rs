use ggez::{GameResult};
use nalgebra as na;
use std::net::{UdpSocket};
use std::io::{Error, ErrorKind};


type Point2 = na::Point2<f32>;
type Vector2 = na::Vector2<f32>;

pub enum NetworkEvent {
    SnackUpdate(usize, Point2, Vector2),
    PlayerUpdate(usize, usize, Vector2),
    GameUpdate(String, Point2, Vector2),
    None,
}

pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    pub fn new() -> GameResult<Connection> {
        let c = Connection {
            socket: UdpSocket::bind("0.0.0.0:4664").expect("could not bind to address"),
        };
        c.socket.set_nonblocking(true).expect("Faild to configure socket property!");
        
        Ok(c)
    }

    pub fn connect(&mut self, remote: &str, port: &str) {
        self.socket.connect(format!("{}:{}", remote, port)).expect("connect function failed");
    } 

    pub fn send(&mut self, event: NetworkEvent) -> Result<usize, std::io::Error> {
        match event {
            NetworkEvent::SnackUpdate(id, location, velocity) => {
                // let data = ["F".as_bytes(), &id.to_ne_bytes(), &location[0].to_ne_bytes(), &location[1].to_ne_bytes(), &velocity[0].to_ne_bytes(), &velocity[1].to_ne_bytes()].concat();
                // self.socket.send(&data[..])
                self.socket.send(format!("FOOD {} {} {} {} {}\n", id, location[0], location[1], velocity[0], velocity[1]).as_bytes())
            },
            NetworkEvent::PlayerUpdate(id, score, anchor) => {
                // let data = ["P".as_bytes(), &id.to_ne_bytes(), &score.to_ne_bytes(), &anchor[0].to_ne_bytes(), &anchor[1].to_ne_bytes()].concat();
                // self.socket.send(&data[..])
                self.socket.send(format!("PLAY {} {} {} {}\n", id, score, anchor[0], anchor[1]).as_bytes())
            },
            NetworkEvent::GameUpdate(_address, location, velocity) => {
                // let data = ["G".as_bytes(), &location[0].to_ne_bytes(), &location[1].to_ne_bytes(), &velocity[0].to_ne_bytes(), &velocity[1].to_ne_bytes()].concat();
                // self.socket.send(&data[..])
                self.socket.send(format!("GAME {} {} {} {}\n", location[0], location[1], velocity[0], velocity[1]).as_bytes())
            },
            NetworkEvent::None => Err(Error::new(ErrorKind::InvalidInput, "no valid package to send"))
        }
    }

    pub fn update(&mut self) -> NetworkEvent {
        let mut buffer = vec![0u8; 64];

        match self.socket.recv_from(&mut buffer) {
            Ok((_size, address)) => {
                let data = String::from_utf8_lossy(&buffer).into_owned();
                let (event, data) = data.as_str().split_at(4);

                let mut positions = [0f32; 4];
                let mut id = 0usize;
                let mut score = 0usize;

                match event {
                    "FOOD" => {
                        for (i, token) in data.split_whitespace().enumerate() {
                            match i {
                                0 => id = token.parse::<usize>().unwrap(),
                                1 | 2 | 3 | 4 => positions[i-1] = token.parse::<f32>().unwrap(),
                                5 => {},
                                _ => println!("package error")
                            }
                        }
                        return NetworkEvent::SnackUpdate(id, Point2::new(positions[0], positions[1]), Vector2::new(positions[2], positions[3]))
                    },
                    "PLAY" => {
                        for (i, token) in data.split_whitespace().enumerate() {
                            match i {
                                0 => id = token.parse::<usize>().unwrap(),
                                1 => score = token.parse::<usize>().unwrap(),
                                2 | 3 => positions[i-1] = token.parse::<f32>().unwrap(),
                                4 => {},
                                _ => println!("package error")
                            }
                        }
                        return NetworkEvent::PlayerUpdate(id, score, Vector2::new(positions[1], positions[2]))
                    },
                    "GAME" => {
                        for (i, token) in data.split_whitespace().enumerate() {
                            match i {
                                0 | 1 | 2 | 3 => positions[i] = token.parse::<f32>().unwrap(),
                                4 => {},
                                _ => println!("package error")
                            }
                        }
                        println!("{}", address.ip());
                        return NetworkEvent::GameUpdate(format!("{}", address.ip()), Point2::new(positions[0], positions[1]), Vector2::new(positions[2], positions[3]))
                    },
                    _ => return NetworkEvent::None
                } 
            },
            Err(_e) => return NetworkEvent::None
        }
    }
}
