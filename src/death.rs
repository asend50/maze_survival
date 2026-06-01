use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;

pub async fn run(junoscary_value: i32, haruscary_value: i32, louisscary_value :i32, score_value: i32, current_streak: i32) -> (String, i32, i32, i32, i32, i32)  {

    let borderred = Color::from_hex(0x4D0000);
    let textred = Color::from_hex(0xD40000);
    let backgroundred = Color::from_hex(0x120000);

    let mut lbl_death = Label::new("", 325.0, 435.0, 90);
    lbl_death.with_colors(textred, Some(borderred));
    lbl_death.with_round(10.0);

    let mut lbl_restart = Label::new("Press space to return to title screen", 280.0, 635.0, 50);
    lbl_restart.with_colors(textred, Some(borderred));
    lbl_restart.with_round(10.0);

    let mut lbl_score = Label::new("", 465.0, 235.0, 55);
    lbl_score.with_colors(textred, Some(borderred));
    lbl_score.with_round(10.0);

    let mut lbl_streak = Label::new("", 705.0, 235.0, 55);
    lbl_streak.with_colors(textred, Some(borderred));
    lbl_streak.with_round(10.0);

    let mut img_junoscary = StillImage::new(
        "",
        530.0,  // width
        610.0,  // height
        400.0,  // x position
        650.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    
    let mut img_haruscary = StillImage::new(
        "",
        510.0,  // width
        610.0,  // height
        512.0,  // x position
        650.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    let mut img_louisscary = StillImage::new(
        "",
        470.0,  // width
        610.0,  // height
        400.0,  // x position
        650.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    

    loop {
        clear_background(backgroundred);

        lbl_score.set_text(&format!("Score: {}", score_value));
        lbl_streak.set_text(&format!("Streak: {}", current_streak));

        if junoscary_value == 1 {
            img_junoscary.set_image("assets/junoscary.png").await;
            lbl_death.set_text("You died to Juno..");
        }

        if haruscary_value == 1 {
            img_haruscary.set_image("assets/haruscary.png").await;
            lbl_death.set_text("You died to Haru..");
        }

        if louisscary_value == 1 {
            img_louisscary.set_image("assets/louisscary.png").await;
            lbl_death.set_text("You died to Louis..");
        }


       
        
        

        if is_key_pressed(KeyCode::Space) {
            let junoscary_value = 0;
            let haruscary_value = 0;
            let louisscary_value = 0;
            return ("menu".to_string(),junoscary_value, haruscary_value, louisscary_value, score_value, current_streak);
        }

        img_junoscary.draw();
        img_haruscary.draw();
        img_louisscary.draw();
        lbl_score.draw();
        lbl_streak.draw();
        lbl_death.draw();
        lbl_restart.draw();
        

        next_frame().await;
    }
}