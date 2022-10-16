use std::rc::Rc;

use super::material::Scatter;
use super::vec::{Vec3, Point3};
use super::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Scatter>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(r: &Ray, t: f64, outward_normal: Vec3, material: Rc<dyn Scatter>) -> HitRecord {
        let p = r.at(t);
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
        
        HitRecord { p, normal, material, t, front_face }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}
