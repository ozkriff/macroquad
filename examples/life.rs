use macroquad::prelude::*;

#[macroquad::main("Fire")]
async fn main() {
    let w = 50usize;
    let h = 30usize;

    let mut image = Image::gen_image_color(w as u16, (h - 2) as u16, BLACK);

    let texture = load_texture_from_image(&image);
    set_texture_filter(texture, FilterMode::Nearest);

    let mut fire = vec![0u8; w * h];
    let mut prev_fire = vec![0u8; w * h];

    let mut acc = 0.0;
    loop {
        acc += get_frame_time();

        clear_background(BLACK);

        set_camera(Camera2D::from_display_rect(Rect::new(
            0.0,
            0.0,
            w as _,
            (h - 5) as _,
        )));

        let image_data = image.get_image_data_mut();

        if acc > 0.1 {
            for i in w + 1..(h - 1) * w - 1 {
                let sum = prev_fire[i - w - 1] as i32
                    + prev_fire[i - w] as i32
                    + prev_fire[i - w + 1] as i32
                    + prev_fire[i - 1] as i32
                    + prev_fire[i + 1] as i32
                    + prev_fire[i + w - 1] as i32
                    + prev_fire[i + w] as i32
                    + prev_fire[i + w + 1] as i32;
                let mut avg = (sum / 8) as u8;

                if rand::gen_range(0, 5) == 0 && (avg > 0 || i >= (h - 4) * w) {
                    if avg >= 1 {
                        avg -= 1;
                    } else {
                        avg = 255;
                    }
                }
                fire[i] = avg;
            }

            for x in 0..(h - 1) * w {
                prev_fire[x as usize] = fire[x + w];
            }

            for i in (h - 7) * w..(h - 1) * w {
                if fire[i] < 15 {
                    fire[i] = 22 - fire[i];
                }
            }

            for i in 0..(h - 2) * w {
                image_data[i] = PALETTE[fire[i] as usize];
            }

            update_texture(texture, &image);

            acc = 0.0;
        }
        draw_texture(texture, 0., 0., WHITE);

        next_frame().await
    }
}

const PALETTE: [Color; 256] = [
    Color::new_const(0, 0, 0, 255),
    Color::new_const(0, 1, 1, 255),
    Color::new_const(0, 4, 5, 255),
    Color::new_const(0, 7, 9, 255),
    Color::new_const(0, 8, 11, 255),
    Color::new_const(0, 9, 12, 255),
    Color::new_const(15, 6, 8, 255),
    Color::new_const(25, 4, 4, 255),
    Color::new_const(33, 3, 3, 255),
    Color::new_const(40, 2, 2, 255),
    Color::new_const(48, 2, 2, 255),
    Color::new_const(55, 1, 1, 255),
    Color::new_const(63, 0, 0, 255),
    Color::new_const(63, 0, 0, 255),
    Color::new_const(63, 3, 0, 255),
    Color::new_const(63, 7, 0, 255),
    Color::new_const(63, 10, 0, 255),
    Color::new_const(63, 13, 0, 255),
    Color::new_const(63, 16, 0, 255),
    Color::new_const(63, 20, 0, 255),
    Color::new_const(63, 23, 0, 255),
    Color::new_const(63, 26, 0, 255),
    Color::new_const(63, 29, 0, 255),
    Color::new_const(63, 33, 0, 255),
    Color::new_const(63, 36, 0, 255),
    Color::new_const(63, 39, 0, 255),
    Color::new_const(63, 39, 0, 255),
    Color::new_const(63, 40, 0, 255),
    Color::new_const(63, 40, 0, 255),
    Color::new_const(63, 41, 0, 255),
    Color::new_const(63, 42, 0, 255),
    Color::new_const(63, 42, 0, 255),
    Color::new_const(63, 43, 0, 255),
    Color::new_const(63, 44, 0, 255),
    Color::new_const(63, 44, 0, 255),
    Color::new_const(63, 45, 0, 255),
    Color::new_const(63, 45, 0, 255),
    Color::new_const(63, 46, 0, 255),
    Color::new_const(63, 47, 0, 255),
    Color::new_const(63, 47, 0, 255),
    Color::new_const(63, 48, 0, 255),
    Color::new_const(63, 49, 0, 255),
    Color::new_const(63, 49, 0, 255),
    Color::new_const(63, 50, 0, 255),
    Color::new_const(63, 51, 0, 255),
    Color::new_const(63, 51, 0, 255),
    Color::new_const(63, 52, 0, 255),
    Color::new_const(63, 53, 0, 255),
    Color::new_const(63, 53, 0, 255),
    Color::new_const(63, 54, 0, 255),
    Color::new_const(63, 55, 0, 255),
    Color::new_const(63, 55, 0, 255),
    Color::new_const(63, 56, 0, 255),
    Color::new_const(63, 57, 0, 255),
    Color::new_const(63, 57, 0, 255),
    Color::new_const(63, 58, 0, 255),
    Color::new_const(63, 58, 0, 255),
    Color::new_const(63, 59, 0, 255),
    Color::new_const(63, 60, 0, 255),
    Color::new_const(63, 60, 0, 255),
    Color::new_const(63, 61, 0, 255),
    Color::new_const(63, 62, 0, 255),
    Color::new_const(63, 62, 0, 255),
    Color::new_const(63, 63, 0, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
    Color::new_const(63, 63, 63, 255),
];
