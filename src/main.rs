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

fn clamp(low: i32, value: i32, high: i32) -> i32 {
    return min(max(value, low), high);
}

const fn color(r: u8, g: u8, b: u8) -> u16 {
    let mut color: u16 = (b as u16) << 10;
    color += (g as u16) << 5;
    color += r as u16;
    return color;
}

#[agb::entry]
fn main() -> ! {
    // my color func doesn't work
    //const WHITE: u16 = color(1, 1, 1);
    //const BLACK: u16 = color(0, 0, 0);
    const WHITE: u16 = 0x001F;
    const BLACK: u16 = 0x0000;
    const WHITE_PAL: u32 = 1;
    const BLACK_PAL: u32 = 2;

    const PADDLE_WIDTH: i32 = 30;
    const PADDLE_HALF_WIDTH: i32 = PADDLE_WIDTH / 2;
    const PADDLE_HEIGHT: i32 = 2;
    const BALL_RADIUS: i32 = 2;
    const DEFAULT_VX: i32 = -2;
    const DEFAULT_VY: i32 = -2;

    let mut gba = agb::Gba::new();
    let mut bitmap = gba.display.video.bitmap4();
    let vblank = agb::interrupt::VBlank::get();
    let mut mgba = agb::mgba::Mgba::new().unwrap();

    let max_x: i32 = display::WIDTH;
    let max_y: i32 = display::HEIGHT;
    let paddle_y_bottom: i32 = max_y - 10;
    let paddle_y_top: i32 = paddle_y_bottom - PADDLE_HEIGHT;
    let mut paddle_x: i32 = max_x / 2;
    let mut paddle_new_x: i32 = paddle_x;
    let mut ball_x: i32 = max_x/2;
    let mut ball_y: i32 = max_y/2;
    let mut ball_new_x: i32 = ball_x;
    let mut ball_new_y: i32 = ball_y;
    let mut ball_vx: i32 = DEFAULT_VX;
    let mut ball_vy: i32 = DEFAULT_VY;

    let draw_rect = |bmp: &mut display::bitmap4::Bitmap4, mut x_min: i32, mut y_min: i32, mut x_max: i32, mut y_max: i32, color_pal: u8| {
        x_min = clamp(0, x_min, max_x);
        x_max = clamp(0, x_max, max_x);

        y_min = clamp(0, y_min, max_y);
        y_max = clamp(0, y_max, max_y);

        for x_pos in x_min..x_max {
            for y_pos in y_min..y_max {
                (*bmp).draw_point(x_pos, y_pos, color_pal);
            }
        }
    };

    bitmap.set_palette_entry(WHITE_PAL, WHITE);
    bitmap.set_palette_entry(BLACK_PAL, BLACK);

    mgba.set_level(agb::mgba::DebugLevel::Debug);

    loop {
        vblank.wait_for_vblank();
        
        // CLEAR
        //draw_rect(&mut bitmap, paddle_x-PADDLE_HALF_WIDTH,paddle_y_top,paddle_x+PADDLE_HALF_WIDTH,paddle_y_bottom, BLACK_PAL as u8);
        draw_rect(&mut bitmap, ball_x-BALL_RADIUS, ball_y-BALL_RADIUS, ball_x+BALL_RADIUS, ball_y+BALL_RADIUS, BLACK_PAL as u8);

        // COMMIT POSITIONS
        paddle_x = paddle_new_x;
        ball_x = ball_new_x;
        ball_y = ball_new_y;

        //mouse_info(&info);

        // CALC POSITIONS
        paddle_new_x = paddle_x;//min(max(paddle_new_x, PADDLE_HALF_WIDTH), max_x-PADDLE_HALF_WIDTH);

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
    }
}
