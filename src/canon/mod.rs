mod utils;
mod modelo;
mod vista;

use modelo::entity::Entity;
use sdl2::keyboard::Keycode;
use sdl2::{Sdl, event::Event};
use vista::SDLView;
use utils::Vec3;

use self::modelo::{
    entity::camara::Camara,
    gameloop::GameLoop
};
use self::utils::Vec2u;


pub struct Canon {
    sdl_view:SDLView,
    escena:Vec<Entity>,
    camara:Camara,
    tiempo:f32,
}

impl Canon {
    pub fn new() -> Canon{
        Canon {
            tiempo:0.0,
            sdl_view: SDLView::new(Vec2u::new(500, 500)),
            escena: Vec::new(),
            camara: Camara::new(Vec3::new(0.0, 0.0, 6.0))
        }
    }

    pub fn run(&mut self){
        self.escena.push(Entity::new_cubo(Vec3::NULO, Vec3::NULO, 1.0));
        
        GameLoop::startLoop(self);

        /*while self.update(0.0) {
            self.paint();
        }*/
    }

    fn update(&mut self, dt:f32) -> bool{ //retorna si se continua con la ejecucion o no
        println!("tiempo: {}", self.tiempo);
        self.tiempo += dt;
        let mut ret = true;
        for event in self.sdl_view.poll_iter() {
            match event {
                Event::Quit { timestamp } => {
                    println!("quit timestamp: {}",timestamp);
                    ret = false;
                }
                Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod: _, repeat: _ } => {
                    if let Some(key) = keycode {
                        if key== Keycode::W {
                            self.camara.vel.z = -1.0*dt;
                        }
                        if key== Keycode::A {
                            self.camara.vel.x = 1.0*dt;
                        }
                        if key== Keycode::S {
                            self.camara.vel.z = 1.0*dt;
                        }
                        if key== Keycode::D {
                            self.camara.vel.x = -1.0*dt;
                        }
                    }
                }
                _ => {}
            }
        }


        for e in self.escena.iter_mut(){
            e.pos.addv(e.vel._mulk(dt));
            e.vel.addv(e.asc._mulk(dt));
        }

        self.camara.pos.addv(self.camara.vel);

        ret
    }

    fn paint(&mut self){
        self.sdl_view.paint(&self.escena, &self.camara);
    }

    pub fn quit(&mut self){

    }
}