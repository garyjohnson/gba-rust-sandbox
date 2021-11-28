#![no_std]
#![no_main]

extern crate agb;
use agb::{display};

fn min(a: i32, b: i32) -> i32 {
    if a < b { 
        return a;
    } else { 
        return b;
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { 
        return a;
    } else { 
        return b;
    }
}

#[agb::entry]
fn main() -> ! {
    const WHITE: u16 = 0x0F00;
    const BLACK: u16 = 0xFF0F;
    const WHITE_PAL: u32 = 1;
    const BLACK_PAL: u32 = 2;

    const PADDLE_WIDTH: i32 = 30;
    const PADDLE_HALF_WIDTH: i32 = PADDLE_WIDTH / 2;
    const PADDLE_HEIGHT: i32 = 2;
    const BALL_RADIUS: i32 = 2;
    const DEFAULT_VX: i32 = -2;
    const DEFAULT_VY: i32 = -2;

    let mut max_x: i32 = display::HEIGHT;
    let mut max_y: i32 = display::WIDTH;
    let paddle_y_bottom: i32 = max_y - 10;
    let paddle_y_top: i32 = paddle_y_bottom - PADDLE_HEIGHT;
    let mut paddle_x: i32 = -1;
    let paddle_new_x: i32 = max_x / 2;
    let mut ball_x: i32 = max_x/2;
    let mut ball_y: i32 = max_y/2;
    let mut ball_new_x: i32 = max_x/2;
    let mut ball_new_y: i32 = max_y/2;
    let mut ball_vx: i32 = DEFAULT_VX;
    let mut ball_vy: i32 = DEFAULT_VY;

    let mut gba = agb::Gba::new();
    let mut bitmap = gba.display.video.bitmap4();

    let draw_rect = |bmp: &mut display::bitmap4::Bitmap4, x: i32, y: i32, height: i32, width: i32, color_pal: u8| {
        for x_pos in x..width {
            for y_pos in y..height {
                (*bmp).draw_point(x_pos, y_pos, color_pal);
            }
        }
    };

    max_y = display::HEIGHT;
    max_x = display::WIDTH;

    bitmap.set_palette_entry(WHITE_PAL, WHITE);
    bitmap.set_palette_entry(BLACK_PAL, BLACK);

    loop {
        //mouse_info(&info);

        // CALC POSITIONS
        //paddle_new_x = min(max(info.pos.x, PADDLE_HALF_WIDTH), max_x-PADDLE_HALF_WIDTH);

        ball_new_x += ball_vx;
        ball_new_y += ball_vy;

        if ball_new_x <= 0 || ball_new_x >= max_x {
            ball_vx = -ball_vx;
        }

        if ball_new_y <= 0 {
            ball_vy = -ball_vy;
        }

        if ball_new_y >= max_y {
            ball_new_x = max_x/2;
            ball_new_y = max_y/2;
            ball_vx = DEFAULT_VX;
            ball_vy = DEFAULT_VY;
        }

        if ball_new_y >= paddle_y_top && 
           ball_new_y <= paddle_y_bottom && 
           ball_new_x+2 >= paddle_x-PADDLE_HALF_WIDTH &&
           ball_new_x+2 <= paddle_x+PADDLE_HALF_WIDTH
        {
            ball_vy = -ball_vy;
        }

        // DRAW 
        draw_rect(&mut bitmap, paddle_new_x-PADDLE_HALF_WIDTH,paddle_y_top,paddle_new_x+PADDLE_HALF_WIDTH,paddle_y_bottom, WHITE_PAL as u8);
        draw_rect(&mut bitmap, ball_new_x-BALL_RADIUS, ball_new_y-BALL_RADIUS, ball_new_x+BALL_RADIUS, ball_new_y+BALL_RADIUS, WHITE_PAL as u8);

        // CYCLE GRAPHICS PAGES
        bitmap.flip_page();

        // CLEAR
        //draw_rect(&mut bitmap, paddle_x-PADDLE_HALF_WIDTH,paddle_y_top,paddle_x+PADDLE_HALF_WIDTH,paddle_y_bottom, BLACK_PAL as u8);
        //draw_rect(&mut bitmap, ball_x-BALL_RADIUS, ball_y-BALL_RADIUS, ball_x+BALL_RADIUS, ball_y+BALL_RADIUS, BLACK_PAL as u8);

        // COMMIT POSITIONS
        paddle_x = paddle_new_x;
        ball_x = ball_new_x;
        ball_y = ball_new_y;

    }
}
