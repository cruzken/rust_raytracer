use crate::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin, direction
        }
    }

    pub fn direction(self) -> Vec3 {
        return self.direction
    }
    
    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
