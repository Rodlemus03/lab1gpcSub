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
framebuffer.set_current_color(0xFF0000); // Azul
framebuffer.fill_polygon(&polygon_2);
framebuffer.set_current_color(0xFFFFFF); // Blanco
framebuffer.polygon(&polygon_2);

let polygon_3 = vec![
    (377, 249), 
    (411, 197), 
    (436, 249)
];
framebuffer.set_current_color(0x0000FF); // Rojo
framebuffer.fill_polygon(&polygon_3);
framebuffer.set_current_color(0xFFFFFF); // Blanco
framebuffer.polygon(&polygon_3);

let polygon_4 = vec![
    (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), 
    (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), 
    (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), 
    (552, 214), (517, 144), (466, 180)
];

let polygon_5 = vec![
    (682, 175), (708, 120), (735, 148), (739, 170)
];

// Rellenar el polígono 4 con verde
framebuffer.set_current_color(0x00FF00); // Verde
framebuffer.fill_polygon(&polygon_4);

// Rellenar el agujero (polígono 5) con el color de fondo (por ejemplo, negro)
framebuffer.set_current_color(0x000000); // Negro (color de fondo)
framebuffer.fill_polygon(&polygon_5);

// Dibujar el borde del polígono 4
framebuffer.set_current_color(0xFFFFFF); // Blanco
framebuffer.polygon(&polygon_4);

// Dibujar el borde del polígono 5
framebuffer.polygon(&polygon_5);





    let _ = framebuffer.render_buffer("output.bmp");

    println!("Respuesta almacenada en output.bmp");
} 
