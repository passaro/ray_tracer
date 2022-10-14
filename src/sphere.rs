use std::sync::Arc;

use super::hit::{Hit, HitRecord};
use super::material::Scatter;
use super::ray::Ray;
use super::vec::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Scatter>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hit for Sphere {
    
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        } 
        
        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(r, root, outward_normal, self.material.clone()))
    }
}
