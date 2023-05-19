use std::time::{Duration, Instant};

const FPS: u128 = 60;

const CANVAS_WIDTH: u32 = 80;
const CANVAS_HEIGHT: u32 = 20;

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

        (x + y * CANVAS_WIDTH) as usize
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

    pub fn render(&self) {
        Self::clear();

        println!("{}", self.c.iter().collect::<String>());
    }
}

fn main() {
    let mut render_duration = Duration::default();
    let mut last_frame_time = Instant::now();
    let minimum_frame_duration = 1000 / FPS;

    let mut canvas = Canvas::new();

    let mut x = 0;
    let mut direction = 1;

    loop {
        let start = Instant::now();
        let elapsed_time = last_frame_time.elapsed() - render_duration;

        if elapsed_time.as_millis() > minimum_frame_duration {
            let start_render = Instant::now();
            canvas.clear_canvas();

            if x > CANVAS_WIDTH as i32 - 1 && direction > 0 {
                direction = -1;
            } else if x <= 0 && direction < 0 {
                direction = 1;
            }

            x += direction;

            let up_x = (x - CANVAS_WIDTH as i32).abs();

            canvas.draw_line((40, 0), (x, 10), '.');
            canvas.draw_line((40, (CANVAS_HEIGHT - 1) as i32), (up_x, 10), '.');

            canvas.draw_line((x, 10), (40, (CANVAS_HEIGHT - 1) as i32), '.');
            canvas.draw_line((up_x, 10), (40, 0), '.');

            canvas.render();

            render_duration = Instant::now().duration_since(start_render);
            last_frame_time = start;
        }
    }
}
