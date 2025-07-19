use macroquad::{prelude::*};
use std::process::exit;
use std::fs;
fn load_score() -> u32 {
    fs::read_to_string("score.txt")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
fn save_score(score: u32) {
    fs::write("score.txt", score.to_string()).unwrap();
}
fn window_conf() -> Conf {
    Conf {
        fullscreen: true,
        window_title: "Epic planes knockoff".to_string(),
        ..Default::default()
    }
}
fn calculate_player(mut x: f32, mut y: f32, mut xm: f32, mut ym: f32) -> (f32, f32, f32, f32) {
    let friction: f32 = 40.0;
    let dims: f32 = 50.0;
    let mut input_x = 0.0;
    let mut input_y = 0.0;
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Space) { input_y -= 1.0; } else { input_y += 1.0; }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) { input_x -= 1.0; } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) { input_x += 1.0; }
    let mag = ((input_x * input_x + input_y * input_y) as f32).sqrt();
    if mag > 1.0 { input_x /= mag; input_y /= mag; }
    let dt = get_frame_time();
    ym += input_y * 40.0 * dt;
    if input_x == 0.0 { xm -= friction * xm.signum() * dt; if xm.abs() < 1.0 { xm = 0.0; } }
    else { xm += input_x * 30.0 * dt; }
    xm = xm.clamp(-45.0, 45.0);
    ym = ym.clamp(-70.0, 40.0);
    if x + xm < 0.0 { x = 0.0; xm = 0.0; }
    else if x + xm + dims > screen_width() { x = screen_width() - dims; xm = 0.0; }
    else { x += xm; }
    if y + ym < 0.0 { y = 0.0; ym = 0.0; }
    else if y + ym + dims > screen_height() { y = screen_height() - dims; ym = 0.0; }
    else { y += ym; }
    return (x, y, xm, ym)
}
fn collision(px: f32, py: f32, ph: f32, pw: f32, ox: f32, oy: f32, oh: f32, ow: f32) -> bool {
    let player_right: f32 = px + pw;
    let player_bottom: f32 = py + ph;
    let obstacle_right: f32 = ox + ow;
    let obstacle_bottom: f32 = oy + oh;
    let horizontal_overlap: bool = px < obstacle_right && player_right > ox;
    let vertical_overlap: bool = py < obstacle_bottom && player_bottom > oy;
    horizontal_overlap && vertical_overlap
}
fn spawn_obstacle() -> (f32, f32) {
    let midpoint: f32 = rand::gen_range(125.0, screen_height() - 125.0);
    let ox1: f32 = midpoint + 75.0;
    let ox2: f32 = (midpoint - 75.0) - screen_height();
    (ox1, ox2)
}
#[macroquad::main(window_conf)]
async fn main() {
    next_frame().await;
    let mut playdims: f32 = 100.0;
    let mut quitdims: f32 = 100.0;
    let mut first_play: bool = true;
    loop {
        let top_score: u32 = load_score();
        loop {
        if is_key_pressed(KeyCode::P) { first_play = false; break; }
        if is_key_pressed(KeyCode::Q) { exit(0) }
        let mut playx: f32 = (screen_width() / 2.0) - (playdims / 2.0);
        let mut playy: f32 = (screen_height() / 2.0) - (playdims / 2.0) + 100.0;
        let mut quitx: f32 = (screen_width() / 2.0) - (quitdims / 2.0);
        let mut quity: f32 = (screen_height() / 2.0) - (quitdims / 2.0) + 250.0;
        let textx: f32 = screen_width() / 2.0;
        let texty: f32 = (screen_height() / 2.0) - 50.0;
        if collision(mouse_position().0, mouse_position().1, 0.1, 0.1, playx, playy, playdims, playdims) {
            if is_mouse_button_pressed(MouseButton::Left) {
                first_play = false;
                break;
            }
            playdims = 120.0;
        } else {
            playdims = 100.0;
        }
        if collision(mouse_position().0, mouse_position().1, 0.1, 0.1, quitx, quity, quitdims, quitdims) {
            if is_mouse_button_pressed(MouseButton::Left) {
                exit(0);
            }
            quitdims = 120.0;
        } else {
            quitdims = 100.0;
        }
        playx = (screen_width() / 2.0) - (playdims / 2.0);
        playy = (screen_height() / 2.0) - (playdims / 2.0) + 100.0;
        quitx = (screen_width() / 2.0) - (quitdims / 2.0);
        quity = (screen_height() / 2.0) - (quitdims / 2.0) + 250.0;
        clear_background(BLUE);
        if first_play {
            draw_text("Play", textx - 150.0, texty, 200.0, ORANGE);
        } else {
            draw_text("Play Again?", textx - 450.0, texty, 200.0, ORANGE);
        }
        draw_text(&format!("Top score: {}", top_score.to_string()), textx - 175.0, texty + 80.0, 75.0, ORANGE);
        draw_rectangle(playx, playy, playdims, playdims, ORANGE);
        draw_text("P", playx + 30.0, playy + 60.0, 80.0, RED);
        draw_rectangle(quitx, quity, quitdims, quitdims, ORANGE);
        draw_text("Q", quitx + 30.0, quity + 60.0, 80.0, RED);
        next_frame().await;
        }
        let pdims: f32 = 50.0;
        let ow: f32 = 50.0;
        let oh: f32 = screen_height() - 75.0;
        let mut oy1: f32 = 0.0;
        let mut oy2: f32 = 0.0;
        let mut ox: f32 = screen_width();
        let mut px: f32 = (screen_width() / 2.0) - 100.0;
        let mut py: f32 = (screen_height() / 2.0) - (pdims / 2.0);
        let mut pxm: f32 = 0.0;
        let mut pym: f32 = 0.0;
        let mut score: u32 = 0;
        loop {
            if collision(px, py, pdims, pdims, ox, oy1, oh, ow) { break }
            if collision(px, py, pdims, pdims, ox, oy2, oh, ow) { break }
            if ox + ow <= 0.0 { ox = screen_width(); (oy1, oy2) = spawn_obstacle(); score += 1; }
            (px, py, pxm, pym) = calculate_player(px, py, pxm, pym);
            ox -= 10.0 + (2 * (score/5)) as f32;
            clear_background(BLUE);
            draw_rectangle(ox, oy1, ow, oh, RED);
            draw_rectangle(ox, oy2, ow, oh, RED);
            draw_rectangle(px, py, pdims, pdims, YELLOW);
            let your_score: String = ("Score: ".to_string() + &score.to_string()).to_string();
            draw_text(&your_score, 0.0, 30.0, 50.0, ORANGE);
            next_frame().await;
        }
        if score > load_score() { save_score(score) }
    }
}
