use crate::canon::utils::Vec3;

pub struct Camara {
    pub pos:Vec3,
    pub vel:Vec3,
    pub ang:Vec3,
}

impl Camara{
    pub fn new(pos:Vec3) -> Camara{
        Camara{
            pos,
            vel: Vec3::NULO,
            ang: Vec3::NULO
        }
    }
}