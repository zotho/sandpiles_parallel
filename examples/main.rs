use macroquad::prelude::*;

use sandpiles_parallel::Field;

const COLORS: [Color; 23] = [
    RED, ORANGE, YELLOW, GOLD, GREEN, BLUE, DARKBLUE, PURPLE, VIOLET, PINK, MAROON, LIME,
    DARKGREEN, SKYBLUE, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, LIGHTGRAY, GRAY, DARKGRAY,
    MAGENTA,
];

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cellular".to_owned(),
        fullscreen: true,
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        sample_count: 64,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let size = 10.0;
    let mut sw;
    let mut sh;

    let mut field: Field<u32> = Field::new(WIDTH, HEIGHT);
    let initial = 100000;
    let n_updates = 1000;
    field[(WIDTH / 2, HEIGHT / 2)] = initial;

    let mut last_x = 0;
    let mut last_y = 0;

    loop {
        sw = screen_width();
        sh = screen_height();

        let x_offset = sw / 2.0 - field.width as f32 * size / 2.0;
        let y_offset = sh / 2.0 - field.height as f32 * size / 2.0;
        let centered = |x, y| (x * size + x_offset, y * size + y_offset);
        let from_centered = |x, y| ((x - x_offset) / size, (y - y_offset) / size);

        let (mx, my) = mouse_position();

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = from_centered(mx, my);
            last_x = x as usize;
            last_y = y as usize;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = from_centered(mx, my);
            let (x, y) = (x as usize, y as usize);
            field.put_line(last_x, last_y, x, y);

            last_x = x;
            last_y = y;
        }

        clear_background(GRAY);

        let (start_x, start_y) = centered(0.0, 0.0);
        draw_rectangle(
            start_x,
            start_y,
            field.width as f32 * size,
            field.height as f32 * size,
            BLACK,
        );

        for _ in 0..n_updates {
            field.update_parallel();
        }

        for y in 0..field.height {
            for x in 0..field.width {
                let (cx, cy) = centered(x as f32, y as f32);
                let count = field[(x, y)];
                if count > 0 {
                    let color = COLORS[(count as usize - 1) % COLORS.len()];
                    draw_rectangle(cx, cy, size, size, color);
                }
            }
        }

        next_frame().await
    }
}
