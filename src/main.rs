mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);


    let polygon_1 = vec![
        (165, 380), 
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330),
        (230, 360), 
        (250, 380), 
        (220, 385), 
        (205, 410), 
        (193, 383)
    ];
    framebuffer.set_current_color(0x00FFFF);
    framebuffer.fill_polygon(&polygon_1);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&polygon_1);

    let polygon_2 = vec![
    (321, 335), 
    (288, 286), 
    (339, 251), 
    (374, 302)
];
framebuffer.set_current_color(0x0000FF); // Azul
framebuffer.fill_polygon(&polygon_2);
framebuffer.set_current_color(0xFFFFFF); // Blanco
framebuffer.polygon(&polygon_2);







    let _ = framebuffer.render_buffer("output.bmp");

    println!("Respuesta almacenada en output.bmp");
} 
