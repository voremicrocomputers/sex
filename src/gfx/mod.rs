use uefi::proto::console::gop::BltPixel;
use crate::{SexInfo, SEXINFO};

pub fn put_pixel(x: i32, y: i32, color: BltPixel) {
    let mut buffer = SEXINFO.page_two.lock(); // buffer for drawing
    let width = SEXINFO.width.lock();
    let index = (y * width.clone() as i32 + x) as usize;
    buffer[index] = color;
}

pub fn draw_box(x: i32, y: i32, width: i32, height: i32, color: BltPixel) {
    let mut buffer = SEXINFO.page_two.lock(); // buffer for drawing
    let pp_scanline = SEXINFO.width.lock();

    for i in 0..width {
        for j in 0..height {
            let index = (y + j) * pp_scanline.clone() as i32 + x + i;
            buffer[index as usize] = color;
        }
    }
}