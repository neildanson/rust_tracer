extern crate image;
use std::path::Path;
use image::*;
use raytracer::vec3::Vec3;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Vec<Vec3>>,
}

pub fn load_texture(filename: &str) -> Texture {
    let path = Path::new(filename);
    let image = image::open(&path).unwrap();

    let (width, height) = image.dimensions();
    let mut rows = Vec::with_capacity(height as usize);
    for y in 0..height {
        let mut pixels = Vec::with_capacity(width as usize);
        for x in 0..width {
            let pix = image.get_pixel(x, y);
            let r = pix[0] as f64 / 255.0;
            let g = pix[1] as f64 / 255.0;
            let b = pix[2] as f64 / 255.0;
            let v = Vec3 { x: r, y: g, z: b };
            pixels.push(v);
        }
        rows.push(pixels);
    }

    Texture {
        width: width,
        height: height,
        pixels: rows,
    }
}

impl Texture {
    pub fn lookup(&self, x: f64, y: f64) -> Vec3 {
        let width = (self.width - 2u32) as f64;
        let height = (self.height - 2u32) as f64;
        let (tu, tv) = (x * width, y * height);
        let (tui, tvi) = (tu.floor(), tv.floor());
        let (tur, tvr) = (tu - tui, tv - tvi);
        let (tuo, tvo) = (1.0 - tur, 1.0 - tvr);

        let (x1, y1) = (tui as usize, tvi as usize);
        let (x2, y2) = (tui as usize + 1, tvi as usize + 1);

        let pix1 = self.pixels[y1][x1];
        let pix2 = self.pixels[y1][x2];
        let pix3 = self.pixels[y2][x1];
        let pix4 = self.pixels[y2][x2];

        let color = (pix1 * tuo + pix2 * tur) * tvo + (pix3 * tuo + pix4 * tur) * tvr;
        color
    }
}