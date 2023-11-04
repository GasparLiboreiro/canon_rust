pub mod camara;

use crate::canon::utils::Vec3u;

use super::super::utils::{
        Vec3,
        Vec3i
};

pub struct Entity{
    pub pos:Vec3,
    pub vel:Vec3,
    pub asc:Vec3,
    pub ang:Vec3,
    mesh:Mesh
}


impl Entity{
    pub fn new_cubo(pos:Vec3<>, ang:Vec3, l_arista:f32) -> Entity{
        Entity { 
            pos, 
            vel: Vec3::NULO,
            asc: Vec3::NULO,
            ang, 
            mesh: Mesh::cubo(l_arista)
        }
    }
    
    pub fn get_mesh(&self)->&Mesh{
        &self.mesh
    }

    pub fn print(&self){
        println!("pos:{}",self.pos.to_string());
        println!("ang:{}",self.ang.to_string());
    }
}

pub struct Mesh {
    vertices:Vec<Vec3>,   //lista de vertices
    triangulos:Vec<Vec3u> //lista de relaciones de vertices, un Vec3i representa las 3 ids de las vertices en el Vec<> vertices que hacen 1 triangulo
}




impl Mesh {
    pub fn new(vertices:Vec<Vec3>, triangulos:Vec<Vec3u>) -> Mesh {
        Mesh {
            vertices, triangulos
        }
    }
    pub fn cubo(l_arista:f32) -> Mesh{

        let mut vertices:Vec<Vec3> = Vec::new();
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    vertices.push(Vec3::new(l_arista*i as f32 - l_arista/2.0, l_arista*j as f32 - l_arista/2.0, l_arista*k as f32 - l_arista/2.0))
                }
            }
        }
        
        let mut triangulos:Vec<Vec3u> = Vec::new();

        triangulos.push(Vec3u::new(0,1,4));
        triangulos.push(Vec3u::new(5,1,4));
        triangulos.push(Vec3u::new(1,5,3));
        triangulos.push(Vec3u::new(7,5,3));
        triangulos.push(Vec3u::new(2,0,3));
        triangulos.push(Vec3u::new(1,0,3));
        triangulos.push(Vec3u::new(2,0,6));
        triangulos.push(Vec3u::new(4,0,6));
        triangulos.push(Vec3u::new(4,5,6));
        triangulos.push(Vec3u::new(7,5,6));
        triangulos.push(Vec3u::new(6,2,7));
        triangulos.push(Vec3u::new(3,2,7));
        Mesh { vertices, triangulos }
    }
    pub fn get_vertices(&self)->&Vec<Vec3>{
        &self.vertices
    }
    pub fn get_triangulos(&self)->&Vec<Vec3u>{
        &self.triangulos
    }
}