use macroquad::{prelude::*};
//Goofy ahh fullscreen setup
fn window_conf() -> Conf {
    Conf {
        fullscreen: true,
        window_title: "Epic planes knockoff".to_string(),
        ..Default::default()
    }
}
//Func to calculate player movement
fn calculate_player(mut x: f32, mut y: f32, mut xm: f32, mut ym: f32) -> (f32, f32, f32, f32) {
    let friction: f32 = 25.0;
    let dims: f32 = 50.0;
    let mut input_x = 0.0;
    let mut input_y = 0.0;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) { input_y -= 1.0; } else { input_y += 1.0; }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { input_x -= 1.0; } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { input_x += 1.0; }
    let mag = ((input_x * input_x + input_y * input_y) as f32).sqrt();
    if mag > 1.0 {
        input_x /= mag;
        input_y /= mag;
    }
    let dt = get_frame_time();
    ym += input_y * 55.0 * dt;
    if input_x == 0.0 {
        xm -= friction * xm.signum() * dt;
        if xm.abs() < 1.0 { xm = 0.0; }
    } else {
        xm += input_x * 30.0 * dt;
    }
    xm = xm.clamp(-45.0, 45.0);
    ym = ym.clamp(-70.0, 40.0);
    if x + xm < 0.0 {
        x = 0.0;
        xm = 0.0;
    } else if x + xm + dims > screen_width() {
        x = screen_width() - dims;
        xm = 0.0;
    } else {
        x += xm;
    }
    if y + ym < 0.0 {
        y = 0.0;
        ym = 0.0;
    } else if y + ym + dims > screen_height() {
        y = screen_height() - dims;
        ym = 0.0;
    } else {
        y += ym;
    }
    return (x, y, xm, ym)
}
fn player_colliding(px: f32, py: f32, ph: f32, pw: f32, ox: f32, oy: f32, oh: f32, ow: f32) -> bool {
    let player_right: f32 = px + pw;
    let player_bottom: f32 = py + ph;
    let obstacle_right: f32 = ox + ow;
    let obstacle_bottom: f32 = oy + oh;
    let horizontal_overlap: bool = px < obstacle_right && player_right > ox;
    let vertical_overlap: bool = py < obstacle_bottom && player_bottom > oy;
    horizontal_overlap && vertical_overlap
}
#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;
    loop {
        let pdims: f32 = 50.0;
        let ow: f32 = 50.0;
        let oh: f32 = screen_height() - 2.0 * pdims;
        let mut ox: f32 = screen_width();
        let mut oy: f32 = rand::gen_range(0.0, screen_height() + oh);
        let mut px: f32 = (screen_width() / 2.0) - 100.0;
        let mut py: f32 = (screen_height() / 2.0) - (pdims / 2.0);
        let mut pxm: f32 = 0.0;
        let mut pym: f32 = 0.0;
        let mut score: u32 = 0;
        loop {
            if player_colliding(px, py, pdims, pdims, ox, oy, oh, ow) { println!("Your score was {}!", score); break }
            if ox + ow <= 0.0 { ox = screen_width(); oy = rand::gen_range(0.0, screen_height() - oh); score += 1; }
            (px, py, pxm, pym) = calculate_player(px, py, pxm, pym);
            ox -= 10.0 + (2 * (score/5)) as f32;
            clear_background(BLUE);
            //Obstacle WIP
            draw_rectangle(ox, oy, ow, oh, RED);
            //Draw the player
            draw_rectangle(px, py, pdims, pdims, YELLOW);
            let your_score: String = ("Score: ".to_string() + &score.to_string()).to_string();
            draw_text(&your_score, 0.0, 30.0, 50.0, ORANGE);
            next_frame().await;
        }
        loop {

        }
    }
}
