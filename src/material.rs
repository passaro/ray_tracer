use super::vec::{Vec3, Color};
use super::ray::Ray;
use super::hit::HitRecord;


pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().reflect(hit.normal).normalized();
        let scattered = Ray::new(hit.p, reflected);

        if scattered.direction().dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
