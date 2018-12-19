use crate::vec3::Vec3;

pub struct Image {
    width: usize,
    height: usize,
    content: Vec<Vec3>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            content: vec![Vec3::zeros(); width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Vec3) {
        self.content[self.width * y + x] = pixel;
    }
    fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        self.content[self.width * y + x]
    }

    pub fn dump(&self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let col = self.get_pixel(x, y);
                let ir = (255.99 * col.r()) as i32;
                let ig = (255.99 * col.g()) as i32;
                let ib = (255.99 * col.b()) as i32;
                println!("{} {} {}", ir, ig, ib);
            }
        }
    }
}