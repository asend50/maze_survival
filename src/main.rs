/*
By: <Asen Doiron>
Date:
Program Details: <The purpose of this program is to survive for one minute in a maze while avoiding three different enemies. This game keeps track of your total wins, your current streak, and your highscore.>
*/

mod modules;
mod menu;
mod game;
mod win;
mod death;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions; // If you want to customize the loading screen

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


let backgroundblue = Color::from_hex(0x005275);
let textblue = Color::from_hex(0xd3f2ff);

    let mut tm = TextureManager::new();

    let loading_options = LoadingScreenOptions {
       title: Some("Maze Survival".to_string()),
       background_color: backgroundblue,
       bar_fill_color: textblue,
       // Use default values for other options
       ..Default::default()
   };
   tm.preload_with_loading_screen(&["assets/mazebackground.png", "assets/legoshispriteleft.png", "assets/legoshispriteright.png", "assets/legoshisprite.png", "assets/junospriteleft.png", "assets/junospriteright.png", "assets/junoscary.png", "assets/haruspriteleft.png", "assets/haruspriteright.png", "assets/haruscary.png", "assets/louisspriteleft.png", "assets/louisspriteright.png", "assets/louisscary.png"], Some(loading_options)).await;

    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;
    let mut junoscary = 0;
    let mut haruscary = 0;
    let mut louisscary = 0;
    let mut score = 0;
    let mut streak = 0;
    let mut highscore = 0;

    loop {
          if get_time() - last_switch > 0.01 {
    (current_screen, junoscary, haruscary, louisscary, score, streak, highscore, tm) = match current_screen.as_str() {
        "menu" => menu::run(junoscary, haruscary, louisscary, score, streak, highscore, tm).await,
        "game" => game::run(junoscary, haruscary, louisscary, score, streak, highscore, tm).await,
        "win" => win::run(junoscary, haruscary, louisscary, score, streak, highscore, tm).await,
        "death" => death::run(junoscary, haruscary, louisscary, score, streak, highscore, tm).await,
        _ => break,
    };
    last_switch = get_time();
}

    next_frame().await;
}}