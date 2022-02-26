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

const fn color(r: u16, g: u16, b: u16) -> u16 {
    let (r, g, b) = (r as u16, g as u16, b as u16);
    let mut color: u16 = (b & 0x1f) << 10;
    color |= (g & 0x1f) << 5;
    color |= r & 0x1f;
    return color;
}

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    const WHITE: u16 = color(255, 255, 255);
    const BLACK: u16 = color(1,1,1);
    const RED: u16 = color(255, 1, 1);
    const GREEN: u16 = color(1, 255, 1);
    const BLUE: u16 = color(1, 1, 255);
    const WHITE_PAL: u32 = 1;
    const BLACK_PAL: u32 = 3;
    const RED_PAL: u32 = 5;
    const GREEN_PAL: u32 = 7;
    const BLUE_PAL: u32 = 9;

    const PADDLE_WIDTH: i32 = 30;
    const PADDLE_HALF_WIDTH: i32 = PADDLE_WIDTH / 2;
    const PADDLE_HEIGHT: i32 = 4;
    const BALL_RADIUS: i32 = 2;
    const DEFAULT_VX: i32 = -2;
    const DEFAULT_VY: i32 = -2;
    const PADDLE_VX: i32 = 2;

    //let mut gba = agb::Gba::new();
    let mut bitmap = gba.display.video.bitmap4();
    let mut input = agb::input::ButtonController::new();
    let vblank = agb::interrupt::VBlank::get();
    /*
    let mut mgba = agb::mgba::Mgba::new().unwrap();
    */

    let max_x: i32 = display::WIDTH;
    let max_y: i32 = display::HEIGHT;
    let paddle_y_bottom: i32 = max_y - 30;
    let paddle_y_top: i32 = paddle_y_bottom - PADDLE_HEIGHT;
    let mut paddle_new_x: i32 = max_x / 2;
    let mut paddle_x: i32 = paddle_new_x;
    let mut ball_x: i32 = max_x/2;
    let mut ball_y: i32 = max_y/2;
    let mut ball_new_x: i32 = ball_x;
    let mut ball_new_y: i32 = ball_y;
    let mut ball_vx: i32 = DEFAULT_VX;
    let mut ball_vy: i32 = DEFAULT_VY;

    let mut paddle_offscreen_x: i32 = paddle_x;
    let mut ball_offscreen_x: i32 = ball_x;
    let mut ball_offscreen_y: i32 = ball_y;

    let draw_rect = |bmp: &mut display::bitmap4::Bitmap4, mut x_min: i32, mut y_min: i32, mut x_max: i32, mut y_max: i32, color_pal: u32| {
        x_min = clamp(0, x_min, max_x);
        x_max = clamp(0, x_max, max_x);

        y_min = clamp(0, y_min, max_y);
        y_max = clamp(0, y_max, max_y);

        for x_pos in x_min..x_max {
            for y_pos in y_min..y_max {
                (*bmp).draw_point(x_pos, y_pos, color_pal as u8);
            }
        }
    };

    let draw_rect_page = |bmp: &mut display::bitmap4::Bitmap4, mut x_min: i32, mut y_min: i32, mut x_max: i32, mut y_max: i32, color_pal: u32, front: bool| {
        x_min = clamp(0, x_min, max_x);
        x_max = clamp(0, x_max, max_x);

        y_min = clamp(0, y_min, max_y);
        y_max = clamp(0, y_max, max_y);

        for x_pos in x_min..x_max {
            for y_pos in y_min..y_max {
                if front {
                    (*bmp).draw_point_page(x_pos, y_pos, color_pal as u8, display::bitmap4::Page::Front);
                } else {
                    (*bmp).draw_point_page(x_pos, y_pos, color_pal as u8, display::bitmap4::Page::Back);
                }
            }
        }
    };


    bitmap.set_palette_entry(WHITE_PAL, WHITE);
    bitmap.set_palette_entry(BLACK_PAL, BLACK);
    bitmap.set_palette_entry(RED_PAL, RED);
    bitmap.set_palette_entry(GREEN_PAL, GREEN);
    bitmap.set_palette_entry(BLUE_PAL, BLUE);

    //clear
    draw_rect_page(&mut bitmap, 0, 0, max_x, max_y, WHITE_PAL, false);
    draw_rect_page(&mut bitmap, 0, 0, max_x, max_y, WHITE_PAL, true);

    loop {
        vblank.wait_for_vblank();

        input.update();

        // CALC POSITIONS
        match input.x_tri() {
            agb::input::Tri::Negative => {
                paddle_new_x = clamp(PADDLE_HALF_WIDTH, paddle_new_x - PADDLE_VX, max_x-PADDLE_HALF_WIDTH);
            }
            agb::input::Tri::Positive => {
                paddle_new_x = clamp(PADDLE_HALF_WIDTH, paddle_new_x + PADDLE_VX, max_x-PADDLE_HALF_WIDTH);
            }
            agb::input::Tri::Zero => {}
        };

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

        draw_rect(&mut bitmap, paddle_new_x-PADDLE_HALF_WIDTH,paddle_y_top,paddle_new_x+PADDLE_HALF_WIDTH,paddle_y_bottom, BLACK_PAL);
        // CLEAR
        //draw_rect(&mut bitmap, ball_offscreen_x-BALL_RADIUS, ball_offscreen_y-BALL_RADIUS, ball_offscreen_x+BALL_RADIUS, ball_offscreen_y+BALL_RADIUS, WHITE_PAL);
        //draw_rect(&mut bitmap, paddle_offscreen_x-PADDLE_HALF_WIDTH,paddle_y_top,paddle_offscreen_x+PADDLE_HALF_WIDTH,paddle_y_bottom, WHITE_PAL);
        draw_rect(&mut bitmap, 0,paddle_y_top,max_x,paddle_y_bottom, WHITE_PAL);
        
        //draw_rect(&mut bitmap, ball_new_x-BALL_RADIUS, ball_new_y-BALL_RADIUS, ball_new_x+BALL_RADIUS, ball_new_y+BALL_RADIUS, BLACK_PAL);
        //draw_rect(&mut bitmap, paddle_new_x-PADDLE_HALF_WIDTH,paddle_y_top,paddle_new_x+PADDLE_HALF_WIDTH,paddle_y_bottom, BLACK_PAL);

        // CYCLE GRAPHICS PAGES
        bitmap.flip_page();

        // COMMIT POSITIONS
        paddle_offscreen_x = paddle_x;
        ball_offscreen_x = ball_x;
        ball_offscreen_y = ball_y;

        paddle_x = paddle_new_x;
        ball_x = ball_new_x;
        ball_y = ball_new_y;
    }
}
