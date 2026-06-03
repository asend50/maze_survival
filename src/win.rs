use macroquad::prelude::*;
use crate::modules::label::Label;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;

pub async fn run(junoscary: i32, haruscary: i32, louisscary :i32, score: i32, streak: i32, highscore: i32, tm: TextureManager) -> (String, i32, i32, i32, i32, i32, i32, TextureManager)  {

let borderblue = Color::from_hex(0x28708f);
let backgroundblue = Color::from_hex(0x005275);
let textblue = Color::from_hex(0xd3f2ff);

    let mut lbl_win = Label::new("You Win!", 512.0, 435.0, 90);
    lbl_win.with_colors(textblue, Some(borderblue));
    lbl_win.with_round(10.0);

    let mut lbl_restart = Label::new("Press space to return to title screen", 280.0, 635.0, 50);
    lbl_restart.with_colors(textblue, Some(borderblue));
    lbl_restart.with_round(10.0);

    let mut lbl_score = Label::new("", 465.0, 235.0, 55);
    lbl_score.with_colors(textblue, Some(borderblue));
    lbl_score.with_round(10.0);

    let mut lbl_streak = Label::new("", 705.0, 235.0, 55);
    lbl_streak.with_colors(textblue, Some(borderblue));
    lbl_streak.with_round(10.0);

    let mut lbl_highscore = Label::new("", 560.0, 100.0, 55);
    lbl_highscore.with_colors(textblue, Some(borderblue));
    lbl_highscore.with_round(10.0);

     let mut img_legoshi = StillImage::new(
        "assets/legoshisprite.png",
        530.0, // width
        610.0, // height
        400.0, // x position
        650.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    loop {
        clear_background(backgroundblue);

        lbl_score.set_text(format!("Wins: {}", score));
        lbl_streak.set_text(format!("Streak: {}", streak));

        img_legoshi.set_preload(tm.get_preload("assets/legoshisprite.png").unwrap());
        lbl_highscore.set_text(format!("Highscore: {}", highscore));
        
        

        if is_key_pressed(KeyCode::Space) {
            return ("menu".to_string(),junoscary, haruscary, louisscary, score, streak, highscore, tm);
        }

        lbl_win.draw();
        lbl_restart.draw();
        lbl_score.draw();
        lbl_streak.draw();
        img_legoshi.draw();
        lbl_highscore.draw();

        next_frame().await;
    }
}