use macroquad::prelude::*;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;

pub async fn run(junoscary: i32, haruscary: i32, louisscary :i32, score: i32, streak: i32, highscore: i32, tm: TextureManager) -> (String, i32, i32, i32, i32, i32, i32, TextureManager) {

let borderblue = Color::from_hex(0x8FDFFF);
let textblue = Color::from_hex(0x005275);
let backgroundblue = Color::from_hex(0xCCF4FF);



    let mut lbl_menu = Label::new("Maze Survival", 395.0, 435.0, 110);
    lbl_menu.with_colors(textblue, Some(borderblue));
    lbl_menu.with_round(10.0);

    let mut lbl_help = Label::new("Try your best to survive for one\nminute while avoiding the enemies", 420.0, 635.0, 40);
    lbl_help.with_colors(textblue, Some(borderblue));
    lbl_help.with_round(10.0);

    let mut lbl_start = Label::new("Press space to start", 470.0, 835.0, 55);
    lbl_start.with_colors(textblue, Some(borderblue));
    lbl_start.with_round(10.0);

    let mut lbl_score = Label::new("", 480.0, 235.0, 55);
    lbl_score.with_colors(textblue, Some(borderblue));
    lbl_score.with_round(10.0);

    let mut lbl_streak = Label::new("", 730.0, 235.0, 55);
    lbl_streak.with_colors(textblue, Some(borderblue));
    lbl_streak.with_round(10.0);

    let mut lbl_highscore = Label::new("", 560.0, 100.0, 55);
    lbl_highscore.with_colors(textblue, Some(borderblue));
    lbl_highscore.with_round(10.0);

    loop {
        clear_background(backgroundblue);
        lbl_score.set_text(format!("Wins: {}", score));
        lbl_streak.set_text(format!("Streak: {}", streak));
        lbl_highscore.set_text(format!("Highscore: {}", highscore));

         if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), junoscary, haruscary, louisscary, score, streak, highscore, tm);
        }

        lbl_menu.draw();
        lbl_help.draw();
        lbl_start.draw();
        lbl_score.draw();
        lbl_streak.draw();
        lbl_highscore.draw();
        next_frame().await;
    }
}