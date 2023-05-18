use std::time::{Duration, Instant};

const FPS: u128 = 60;

const CANVAS_WIDTH: u32 = 80;
const CANVAS_HEIGHT: u32 = 20;

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

    fn map_coordinate(x: u32, y: u32) -> usize {
        let x = if x as i32 - 1 < 0 { 0 as u32 } else { x - 1 };
        let y = ((CANVAS_HEIGHT - 1) as i32 - y as i32).abs() as u32;

        (x + y * CANVAS_WIDTH) as usize
    }

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> char {
        self.c[Self::map_coordinate(x, y)]
    }

    pub fn pixel(&mut self, x: u32, y: u32, c: char) {
        self.c[Self::map_coordinate(x, y)] = c;
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

    let mut x: (i32, i32) = (0, 1);
    let mut y: (i32, i32) = (0, 1);

    loop {
        let start = Instant::now();
        let elapsed_time = last_frame_time.elapsed() - render_duration;

        if elapsed_time.as_millis() > minimum_frame_duration {
            let start_render = Instant::now();

            if x.1 > 0 && x.0 >= (CANVAS_WIDTH - 1) as i32 {
                x.1 = -1;
            } else if x.1 < 0 && x.0 <= 0 {
                x.1 = 1;
            }

            if y.1 > 0 && y.0 >= (CANVAS_HEIGHT - 1) as i32 {
                y.1 = -1;
            } else if y.1 < 0 && y.0 <= 0 {
                y.1 = 1;
            }

            x.0 += 1 * x.1;
            y.0 += 1 * y.1;

            if canvas.get_pixel(x.0 as u32, y.0 as u32) == '#' {
                canvas.pixel(x.0 as u32, y.0 as u32, ' ');
            } else {
                canvas.pixel(x.0 as u32, y.0 as u32, '#');
            }

            canvas.render();

            render_duration = Instant::now().duration_since(start_render);
            last_frame_time = start;
        }
    }
}
