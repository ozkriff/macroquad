use macroquad::experimental::{window, graphics, prelude::*};

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        window::clear_background(RED);

        // TODO
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        graphics::Line::new(vec2(40.0, 40.0), vec2(100.0, 200.0), 15.0, BLUE).draw();

        // TODO
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // TODO
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);
        graphics::Text::new("HELLO")
            .with_color(DARKGRAY)
            .with_font_size(30)
            .draw_at(20.0, 20.0); // TODO: set position with `.with_pos(Vec2(...))`?

        window::next_frame().await
    }
}


/*
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        
        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
*/
