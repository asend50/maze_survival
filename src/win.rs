use macroquad::prelude::*;
use crate::modules::label::Label;

pub async fn run(junoscary: i32, haruscary: i32, louisscary :i32, score: i32, streak: i32) -> (String, i32, i32, i32, i32, i32)  {

    let borderblue = Color::from_hex(0x8FDFFF);
    let textblue = Color::from_hex(0x005275);
    let backgroundblue = Color::from_hex(0xCCF4FF);

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

    loop {
        clear_background(backgroundblue);

        lbl_score.set_text(format!("Wins: {}", score));
        lbl_streak.set_text(format!("Streak: {}", streak));
        
        

        if is_key_pressed(KeyCode::Space) {
            return ("menu".to_string(),junoscary, haruscary, louisscary, score, streak);
        }

        lbl_win.draw();
        lbl_restart.draw();
        lbl_score.draw();
        lbl_streak.draw();

        next_frame().await;
    }
}