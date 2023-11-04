#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x:f32,
    pub y:f32,
    pub z:f32
}

impl Vec3 {
    pub const NULO:Vec3 = Vec3{x:0.0,y:0.0,z:0.0};

    pub fn new(x:f32,y:f32,z:f32) -> Vec3{
        Vec3 {x,y,z}
    }

    pub fn add(&mut self, x:f32, y:f32, z:f32){
        self.x+=x;
        self.y+=y;
        self.z+=z;
    }
    pub fn addv(&mut self, v:Vec3){
        self.x+=v.x;
        self.y+=v.y;
        self.z+=v.z;
    }


    pub fn set(&mut self, x:f32, y:f32, z:f32){
        self.x=x;
        self.y=y;
        self.z=z;
    }
    pub fn setv(&mut self, v:Vec3){
        self.x=v.x;
        self.y=v.y;
        self.z=v.z;
    }

    pub fn _mulk(&self, k:f32)->Vec3{
        Vec3 { x: self.x*k, y: self.y*k, z: self.z*k }
    }

    pub fn modulo(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn distancia(&self, v:Vec3) -> f32{
        ((self.x-v.x).powi(2) + (self.y-v.y).powi(2) + (self.z-v.z).powi(2)).sqrt()
    }

    pub fn inv(&self) -> Vec3{
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }

    pub fn to_string(&self) -> String{
        String::new()+"("+&self.x.to_string()+";"+&self.y.to_string()+";"+&self.z.to_string()+")"
    }
}

use std::fmt;
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
#[derive(Copy, Clone)]
pub struct Vec3i {
    x:i32,
    y:i32,
    z:i32
}

impl Vec3i {
    pub const NULO:Vec3i = Vec3i{x:0,y:0,z:0};

    pub fn new(x:i32,y:i32,z:i32) -> Vec3i{
        Vec3i {x,y,z}
    }

    pub fn add(&mut self, x:i32, y:i32, z:i32){
        self.x+=x;
        self.y+=y;
        self.z+=z;
    }

    pub fn set(&mut self, x:i32, y:i32, z:i32){
        self.x=x;
        self.y=y;
        self.z=z;
    }

    pub fn modulo(&self) -> f32{
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f32).sqrt()
    }

    pub fn distancia(&self, v:Vec3i) -> f32{
        (((self.x-v.x).pow(2) + (self.y-v.y).pow(2) + (self.z-v.z).pow(2)) as f32).sqrt()
    }

    pub fn to_string(&self) -> String{
        String::new()+"("+&self.x.to_string()+";"+&self.y.to_string()+";"+&self.z.to_string()+")"
    }
}

impl fmt::Display for Vec3i {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
#[derive(Copy, Clone)]
pub struct Vec3u {
    pub x:u32,
    pub y:u32,
    pub z:u32
}

impl Vec3u {
    pub const NULO:Vec3u = Vec3u{x:0,y:0,z:0};

    pub fn new(x:u32,y:u32,z:u32) -> Vec3u{
        Vec3u {x,y,z}
    }

    pub fn add(&mut self, x:u32, y:u32, z:u32){
        self.x+=x;
        self.y+=y;
        self.z+=z;
    }

    pub fn set(&mut self, x:u32, y:u32, z:u32){
        self.x=x;
        self.y=y;
        self.z=z;
    }

    pub fn modulo(&self) -> f32{
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f32).sqrt()
    }

    pub fn distancia(&self, v:Vec3u) -> f32{
        (((self.x-v.x).pow(2) + (self.y-v.y).pow(2) + (self.z-v.z).pow(2)) as f32).sqrt()
    }

    pub fn to_string(&self) -> String{
        String::new()+"("+&self.x.to_string()+";"+&self.y.to_string()+";"+&self.z.to_string()+")"
    }
}

impl fmt::Display for Vec3u {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
#[derive(Copy, Clone)]
pub struct Vec2u {
    pub x:u32,
    pub y:u32
}

impl Vec2u {
    pub const NULO:Vec2u = Vec2u{x:0,y:0};

    pub fn new(x:u32,y:u32) -> Vec2u{
        Vec2u {x,y}
    }

    pub fn add(&mut self, x:u32, y:u32){
        self.x+=x;
        self.y+=y;
    }

    pub fn set(&mut self, x:u32, y:u32){
        self.x=x;
        self.y=y;
    }

    pub fn modulo(&self) -> f32{
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn distancia(&self, v:Vec2u) -> f32{
        (((self.x-v.x).pow(2) + (self.y-v.y).pow(2)) as f32).sqrt()
    }

    pub fn to_string(&self) -> String{
        String::new()+"("+&self.x.to_string()+";"+&self.y.to_string()+")"
    }
}

impl fmt::Display for Vec2u {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}









pub fn get_view_transform(alfa:f32, n:f32, f:f32) -> [[f32;4];4] {
    [
        [1.0/(alfa/2.0).tan(), 0.0, 0.0, 0.0],
        [0.0, 1.0/(alfa/2.0).tan(), 0.0, 0.0],
        [0.0, 0.0, (f+n)/(f-n), (2.0*f*n)/(f-n)],
        [0.0, 0.0, -1.0, 0.0],
    ]
}
pub fn get_traslation_transform(mov:Vec3) -> [[f32;4];4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [mov.x, mov.y, mov.z, 1.0],
    ]
}

pub fn mul_matriz4x4(matriz1:[[f32;4];4], matriz2:[[f32;4];4]) -> [[f32;4];4]{
    let mut ret = [[0.0;4];4];
    
    for j in 0..4 {
        for i in 0..4 {
            for k in 0..4 {
                ret[i][j] += matriz1[k][j]*matriz2[i][k];
            }
        }
    }

    ret
}
pub fn mul_vec3matriz4x4(vec:&Vec3, matriz1:[[f32;4];4]) -> [f32;4]{
    let mut ret = [0.0;4];
    let mut matriz2:[f32;4] = [vec.x, vec.y, vec.z, 1.0];

    for i in 0..4 {
        for k in 0..4 {
            ret[i] += matriz1[k][i]*matriz2[k];
        }
    }
    
    ret
}

pub fn print_matriz4x4(matriz:[[f32;4];4]){
    println!("[");
    for j in 0..4{
        for i in 0..4{
            print!(" ,{}",matriz[i][j]);
        }
        print!("\n");
    }
    println!("]");
}