use sdl2::{Sdl, VideoSubsystem, render::Canvas, video::Window, pixels::Color, EventPump, sys::__STDC_IEC_559__, rect::Point, event::EventPollIterator};
use crate::canon::utils::print_matriz4x4;

use super::{utils::{Vec2u, Vec3, mul_vec3matriz4x4, mul_matriz4x4, get_view_transform, get_traslation_transform}, modelo::entity::{Entity, camara::{self, Camara}}};
pub struct SDLView {
    sdl_context:Sdl,
    screen_size:Vec2u,
    canvas:Canvas<Window>,
    event_queue:EventPump,
    alfa:f32,
    n:f32,
    f:f32
}

impl SDLView {
    pub fn new(screen_size:Vec2u) -> SDLView{

        let screen_size = screen_size;
        let sdl_context = sdl2::init().expect(":)");
        let video_subsystem = sdl_context.video().expect("yeyy");
        let window = video_subsystem.window("visuales", screen_size.x, screen_size.y)
            .build()
            .unwrap();
        let canvas = window.into_canvas()
        .build()
        .unwrap();
    

        let mut event_queue = sdl_context.event_pump().unwrap();


        

        SDLView { 
            screen_size, 
            sdl_context, 
            canvas,
            event_queue,
            alfa:std::f32::consts::PI/2.0,
            n:0.25,
            f:20.0
        }
    }

    pub fn poll_iter(&mut self) -> EventPollIterator{
        self.event_queue.poll_iter()
    }


    pub fn paint(&mut self, escena:&Vec<Entity>, camara:&Camara) {
        self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        
        let camera_transform = mul_matriz4x4(get_view_transform(self.alfa, self.n, self.f), get_traslation_transform(camara.pos));
        
        for entity in escena {
            for triangulo in entity.get_mesh().get_triangulos() {
                let mut vertice_1 = *entity.get_mesh().get_vertices().get(triangulo.x as usize).unwrap();
                let mut vertice_2 = *entity.get_mesh().get_vertices().get(triangulo.y as usize).unwrap();
                let mut vertice_3 = *entity.get_mesh().get_vertices().get(triangulo.z as usize).unwrap();

                vertice_1 = SDLView::arr4x1_a_vec3(mul_vec3matriz4x4(&vertice_1, camera_transform));
                vertice_2 = SDLView::arr4x1_a_vec3(mul_vec3matriz4x4(&vertice_2, camera_transform));
                vertice_3 = SDLView::arr4x1_a_vec3(mul_vec3matriz4x4(&vertice_3, camera_transform));
                
                self.canvas.draw_line(self.image_sp_to_screen_point(&vertice_1), self.image_sp_to_screen_point(&vertice_2)).expect("idk");
                self.canvas.draw_line(self.image_sp_to_screen_point(&vertice_2), self.image_sp_to_screen_point(&vertice_3)).expect("idk");
                self.canvas.draw_line(self.image_sp_to_screen_point(&vertice_1), self.image_sp_to_screen_point(&vertice_3)).expect("idk");
                
            }
        }
        
        self.canvas.present();
    }

    fn image_sp_to_screen_point(&self, vec:&Vec3) -> Point {
        let max_size:f32 = if self.screen_size.x > self.screen_size.y {self.screen_size.x as f32} else {self.screen_size.y as f32};

        Point::new(
            (vec.x*max_size/2.0+max_size/2.0) as i32,
            (vec.y*max_size/2.0+max_size/2.0) as i32
        )
    }

    fn arr4x1_a_vec3(arr:[f32;4]) -> Vec3 {
        //println!("[{},{},{},{}]",arr[0], arr[1], arr[2], arr[3]);
        Vec3::new(arr[0]/arr[3], arr[1]/arr[3], arr[2]/arr[3])
    }
}

