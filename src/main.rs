use std::time::{Duration, Instant};

const FPS: u128 = 60;

const CANVAS_WIDTH: u32 = 80;
const CANVAS_HEIGHT: u32 = 22;

type Vec2d = (i32, i32);

pub struct Canvas {
    pub c: Vec<char>,
}

impl Canvas {
    fn new() -> Self {
        let mut c = vec![' '; CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize];

        for i in 0..CANVAS_HEIGHT {
            c[Self::map_coordinate(CANVAS_WIDTH, i as u32)] = '\n';
        }

        Self { c }
    }

    /// Coordinate Systems:
    /// +y
    ///
    /// -y
    /// -x     +x
    fn map_coordinate(x: u32, y: u32) -> usize {
        let x = if x as i32 - 1 < 0 { 0 as u32 } else { x - 1 };
        let y = ((CANVAS_HEIGHT - 1) as i32 - y as i32).abs() as u32;
        let idx = (x + y * CANVAS_WIDTH) as usize;

        if idx as u32 >= CANVAS_HEIGHT * CANVAS_WIDTH {
            0
        } else {
            idx
        }
    }

    pub fn clear_canvas(&mut self) {
        self.c = vec![' '; CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize];

        for i in 0..CANVAS_HEIGHT {
            self.c[Self::map_coordinate(CANVAS_WIDTH, i as u32)] = '\n';
        }
    }

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> char {
        self.c[Self::map_coordinate(x, y)]
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, c: char) {
        self.c[Self::map_coordinate(x, y)] = c;
    }

    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    pub fn draw_line(&mut self, a: Vec2d, b: Vec2d, c: char) {
        let mut a = a;

        let dx = (b.0 - a.0).abs();
        let sx = if a.0 < b.0 { 1 } else { -1 };

        let dy = (b.1 - a.1).abs() * -1;
        let sy = if a.1 < b.1 { 1 } else { -1 };

        let mut error = dx + dy;

        loop {
            self.draw_pixel(a.0 as u32, a.1 as u32, c);

            if a.0 == b.0 && a.1 == b.1 {
                break;
            }

            let e2 = 2 * error;
            if e2 >= dy {
                if a.0 == b.0 {
                    break;
                }
                error += dy;
                a.0 += sx;
            }
            if e2 <= dx {
                if a.1 == b.1 {
                    break;
                }
                error += dx;
                a.1 += sy;
            }
        }
    }

    fn render_circle(&mut self, c: Vec2d, v: Vec2d, ch: char) {
        self.draw_pixel((c.0 + v.0) as u32, (c.1 + v.1) as u32, ch);
        self.draw_pixel((c.0 - v.0) as u32, (c.1 + v.1) as u32, ch);
        self.draw_pixel((c.0 + v.0) as u32, (c.1 - v.1) as u32, ch);
        self.draw_pixel((c.0 - v.0) as u32, (c.1 - v.1) as u32, ch);
        self.draw_pixel((c.0 + v.1) as u32, (c.1 + v.0) as u32, ch);
        self.draw_pixel((c.0 - v.1) as u32, (c.1 + v.0) as u32, ch);
        self.draw_pixel((c.0 + v.1) as u32, (c.1 - v.0) as u32, ch);
        self.draw_pixel((c.0 - v.1) as u32, (c.1 - v.0) as u32, ch);
    }

    pub fn draw_circle(&mut self, c: Vec2d, r: i32, ch: char) {
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        self.render_circle(c, (x, y), ch);

        while y >= x {
            x += 1;

            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            self.render_circle(c, (x, y), ch);
        }
    }

    pub fn render(&self) {
        Self::clear();

        println!("{}", self.c.iter().collect::<String>());
    }
}

fn main() {
    let mut render_duration = Duration::default();
    let mut last_frame_time = Instant::now();
    let mut delta_time = Instant::now();
    let minimum_frame_duration = 1000 / FPS;

    let mut canvas = Canvas::new();
    let mut degree_a: f32 = 0.0;
    let mut degree_b: f32 = 0.0;

    // Make clock time go faster
    let a = 120.0;
    let degree_a_off =
        (2.0 * std::f32::consts::PI / Duration::from_secs(3600).as_millis() as u32 as f32) * a;
    let degree_b_off =
        (2.0 * std::f32::consts::PI / Duration::from_secs(12 * 3600).as_millis() as u32 as f32) * a;

    let initial_a: (f32, f32) = (25.0, 20.0);
    let initial_b: (f32, f32) = (25.0, 15.0);
    let medium: (f32, f32) = (25.0, 11.0);

    loop {
        let start = Instant::now();
        let elapsed_time = last_frame_time.elapsed() - render_duration;

        if elapsed_time.as_millis() > minimum_frame_duration {
            let start_render = Instant::now();
            canvas.clear_canvas();

            let cos = degree_a.cos();
            let sin = degree_a.sin();

            let a = ((initial_a.0 - medium.0), (initial_a.1 - medium.1));
            let a = (
                ((a.0 * cos) + (a.1 * sin)) as i32,
                (-(a.0 * sin) + (a.1 * cos)) as i32,
            );
            let a = ((a.0 + medium.0 as i32), (a.1 + medium.1 as i32));

            let cos = degree_b.cos();
            let sin = degree_b.sin();

            let b = ((initial_b.0 - medium.0), (initial_b.1 - medium.1));
            let b = (
                ((b.0 * cos) + (b.1 * sin)) as i32,
                (-(b.0 * sin) + (b.1 * cos)) as i32,
            );
            let b = ((b.0 + medium.0 as i32), (b.1 + medium.1 as i32));

            let medium = (medium.0 as i32, medium.1 as i32);

            canvas.draw_circle(medium, 10, '#');
            canvas.draw_line(a, medium, '*');
            canvas.draw_line(medium, b, '.');

            canvas.render();

            degree_a += degree_a_off * delta_time.elapsed().as_millis() as u32 as f32;
            degree_b += degree_b_off * delta_time.elapsed().as_millis() as u32 as f32;

            delta_time = Instant::now();
            render_duration = delta_time.duration_since(start_render);
            last_frame_time = start;
        }
    }
}
