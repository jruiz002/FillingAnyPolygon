use raylib::prelude::*;

/// Dimensiones de la imagen
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

/// Estructura para representar un polígono
#[derive(Debug, Clone)]
struct Polygon {
    points: Vec<(i32, i32)>,
    fill_color: Color,
    line_color: Color,
    is_hole: bool,
}

impl Polygon {
    fn new(points: Vec<(i32, i32)>, fill_color: Color, line_color: Color, is_hole: bool) -> Self {
        let adjusted_points = points
            .into_iter()
            .map(|(x, y)| (x, HEIGHT - y)) 
            .collect();

        Self {
            points: adjusted_points,
            fill_color,
            line_color,
            is_hole,
        }
    }
}

/// Relleno por scan-line con soporte de agujeros
fn fill_polygons(image: &mut Image, polygons: &[Polygon]) {
    let solids: Vec<&Polygon> = polygons.iter().filter(|p| !p.is_hole).collect();
    let holes: Vec<&Polygon> = polygons.iter().filter(|p| p.is_hole).collect();

    for poly in solids {
        scanline_fill(image, poly, Color::BLACK, &holes);
    }
}

/// Algoritmo de relleno con soporte de agujeros
fn scanline_fill(image: &mut Image, polygon: &Polygon, bg_color: Color, holes: &[&Polygon]) {
    if polygon.points.len() < 3 {
        return;
    }

    let points = &polygon.points;
    let mut edges = Vec::new();

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];
        if y1 != y2 {
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            let (min_x, max_x) = if y1 < y2 { (x1, x2) } else { (x2, x1) };
            edges.push((min_y, max_y, min_x, max_x));
        }
    }

    let min_y = points.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = Vec::new();

        for &(ey1, ey2, ex1, ex2) in &edges {
            if y >= ey1 && y < ey2 {
                let x = ex1 + ((y - ey1) * (ex2 - ex1)) / (ey2 - ey1);
                intersections.push(x);
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x1 = intersections[i];
                let x2 = intersections[i + 1];
                for x in x1..=x2 {
                    if !is_point_in_any_polygon((x, y), holes) {
                        image.draw_pixel(x, y, polygon.fill_color);
                    } else {
                        image.draw_pixel(x, y, bg_color);
                    }
                }
            }
        }
    }

    // Dibujar contorno
    for i in 0..polygon.points.len() {
        let (x1, y1) = polygon.points[i];
        let (x2, y2) = polygon.points[(i + 1) % polygon.points.len()];
        image.draw_line(x1, y1, x2, y2, polygon.line_color);
    }
}

fn is_point_in_polygon(point: (i32, i32), polygon: &Polygon) -> bool {
    let (px, py) = point;
    let mut inside = false;
    let n = polygon.points.len();
    for i in 0..n {
        let (x1, y1) = polygon.points[i];
        let (x2, y2) = polygon.points[(i + 1) % n];
        if ((y1 > py) != (y2 > py)) &&
            (px < (x2 - x1) * (py - y1) / (y2 - y1 + 1) + x1)
        {
            inside = !inside;
        }
    }
    inside
}

fn is_point_in_any_polygon(point: (i32, i32), holes: &[&Polygon]) -> bool {
    for &hole in holes {
        if is_point_in_polygon(point, hole) {
            return true;
        }
    }
    false
}

fn main() {
    let mut image = Image::gen_image_color(WIDTH, HEIGHT, Color::BLACK);

    let polygons = vec![
        Polygon::new(
            vec![
                (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
                (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
            ],
            Color::YELLOW,
            Color::WHITE,
            false,
        ),
        Polygon::new(
            vec![
                (321, 335), (288, 286), (339, 251), (374, 302),
            ],
            Color::BLUE,
            Color::WHITE,
            false,
        ),
        Polygon::new(
            vec![
                (377, 249), (411, 197), (436, 249),
            ],
            Color::RED,
            Color::WHITE,
            false,
        ),
        Polygon::new(
            vec![
                (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
                (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
                (597, 215), (552, 214), (517, 144), (466, 180),
            ],
            Color::GREEN,
            Color::WHITE,
            false,
        ),
        // Agujero
        Polygon::new(
            vec![
                (682, 175), (708, 120), (735, 148), (739, 170),
            ],
            Color::BLACK,
            Color::WHITE,
            true,
        ),
    ];

    fill_polygons(&mut image, &polygons);
    image.export_image("out.png");
    println!("Imagen generada como out.png");
}
