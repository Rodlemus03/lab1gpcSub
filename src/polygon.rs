use crate::framebuffer::Framebuffer;
use crate::line::Line;

pub trait Polygon {
    fn polygon(&mut self, points: &[(usize, usize)]);
    fn fill_polygon(&mut self, points: &[(usize, usize)]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()];

            self.line(x0, y0, x1, y1);
        }
    }

    fn fill_polygon(&mut self, vertices: &[(usize, usize)]) {
        if vertices.len() < 3 {
            return;
        }
    
        let (mut min_y, mut max_y) = (vertices[0].1, vertices[0].1);
        for &(_, y) in vertices.iter() {
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
    
        for scanline_y in min_y..=max_y {
            let mut intersection_points = Vec::new();
            let mut previous_index = vertices.len() - 1;
            
            for current_index in 0..vertices.len() {
                let (current_x, current_y) = vertices[current_index];
                let (previous_x, previous_y) = vertices[previous_index];
                
                if (current_y < scanline_y && previous_y >= scanline_y) || (previous_y < scanline_y && current_y >= scanline_y) {
                    let intersect_x = current_x as f32 + (scanline_y as f32 - current_y as f32) / (previous_y as f32 - current_y as f32) * (previous_x as f32 - current_x as f32);
                    intersection_points.push(intersect_x as usize);
                }
                previous_index = current_index;
            }
    
            intersection_points.sort_unstable();
            let mut iter = intersection_points.iter().peekable();
    
            while let Some(&start) = iter.next() {
                if let Some(&&end) = iter.peek() {
                    for x in start..=end {
                        self.point(x, scanline_y);
                    }
                    iter.next();
                }
            }
        }
    }
    
}