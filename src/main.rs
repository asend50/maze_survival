/*
By: <Asen Doiron>
Date:
Program Details: <The purpose of this program is to >
*/

mod modules;

mod menu;
mod game;
mod win;
mod death;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Program Name".to_owned(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;
    let mut junoscary = 0;
    let mut haruscary = 0;
    let mut louisscary = 0;
    let mut score = 0;
    let mut streak = 0;

    loop {
          if get_time() - last_switch > 0.01 {
    (current_screen, junoscary, haruscary, louisscary, score, streak) = match current_screen.as_str() {
        "menu" => menu::run(junoscary, haruscary, louisscary, score, streak).await,
        "game" => game::run(junoscary, haruscary, louisscary, score, streak).await,
        "win" => win::run(junoscary, haruscary, louisscary, score, streak).await,
        "death" => death::run(junoscary, haruscary, louisscary, score, streak).await,
        _ => break,
    };
    last_switch = get_time();
}

    next_frame().await;
}}