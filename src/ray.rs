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

    pub fn origin(&self) -> Vec3 {
        return self.origin
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction
    }
    
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use super::Vec3;

    #[test]
    fn point_at_parameter() {
        let r = Ray::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        );

        assert_eq!(r.point_at_parameter(2.0), Vec3::new(9.0, 12.0, 15.0));
    }
}
