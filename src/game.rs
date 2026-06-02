use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
use crate::modules::label::Label;
use std::time::Duration;
use crate::modules::preload_image::TextureManager;

pub async fn run(junoscary: i32, haruscary: i32, louisscary: i32, score: i32, streak: i32, highscore: i32, tm: TextureManager) -> (String, i32, i32, i32, i32, i32, i32, TextureManager) {
    let mut junoscary_value = junoscary;
    let mut haruscary_value = haruscary;
    let mut louisscary_value = louisscary;
    let mut score_value = score;
    let mut current_streak = streak;
    let mut highscore_value = highscore;

        let img_background = StillImage::new(
        "assets/mazebackground.png",
        1270.0, // width
        1270.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;

    let mut img_legoshi = StillImage::new(
        "assets/legoshispriteright.png",
        110.0, // width
        130.0, // height
        550.0, // x position
        500.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let img_juno = StillImage::new(
        "assets/junosprite.png",
        110.0, // width
        130.0, // height
        490.0, // x position
        60.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let img_haru= StillImage::new(
        "assets/harusprite.png",
        110.0, // width
        130.0, // height
        850.0, // x position
        800.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let img_louis= StillImage::new(
        "assets/louissprite.png",
        110.0, // width
        130.0, // height
        1080.0, // x position
        1050.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let mut lbl_timer = Label::new("", 635.0, 100.0, 60);
    lbl_timer.with_font(load_ttf_font("assets/Technology.ttf").await.unwrap());

    let mut legoshi = img_legoshi;
    let mut juno = img_juno;
    let mut haru = img_haru;
    let mut louis = img_louis;
    let background = img_background;

    const MOVE_SPEED: f32 = 200.0;

    let mut junox = 0.0;
    let mut junoy = 0.0;

    let mut harux = 0.0;
    let mut haruy = 0.0;

    let mut louisx = 0.0;
    let mut louisy = 0.0;

    let start_time = std::time::Instant::now();
    let duration = Duration::from_secs(60); // 60 seconds

    loop {
        clear_background(WHITE);
        draw_text("Game", 20.0, 40.0, 30.0, WHITE);

        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 4.0;

        legoshi.set_preload(tm.get_preload("assets/legoshispriteright.png").unwrap());
        
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 4.0;

        legoshi.set_preload(tm.get_preload("assets/legoshispriteleft.png").unwrap());
        
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.y += 4.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.y -= 4.0;
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
        let movement = move_dir * MOVE_SPEED * get_frame_time();

        // Save old position in case of collision
        let old_pos = legoshi.pos();

        // Move X first
        if movement.x != 0.0 {
            legoshi.set_x(legoshi.get_x() + movement.x);
            if check_collision(&legoshi, &background, 1) {
                legoshi.set_x(old_pos.x); // Undo if collision happens
            }
        }

        // Move Y next
        if movement.y != 0.0 {
            legoshi.set_y(legoshi.get_y() + movement.y);
            if check_collision(&legoshi, &background, 1) {
                legoshi.set_y(old_pos.y); // Undo if collision happens
            }
        }

// Juno AI

        let mut juno_pos = juno.pos();

        if juno_pos.x == 950.0 && juno_pos.y == 60.0 {
            junox = 0.0;
            junoy = 5.0;
        }

        if juno_pos.x == 950.0 && juno_pos.y == 500.0 {
            junox = 5.0;
            junoy = 0.0;
        }

        if juno_pos.x == 1080.0 && juno_pos.y == 500.0 {
            junox = 0.0;
            junoy = 5.0;
        }

        if juno_pos.x == 1080.0 && juno_pos.y == 1050.0 {
            junox = -5.0;
            junoy = 0.0;
        }

        if juno_pos.x == 250.0 && juno_pos.y == 1050.0 {
            junox = -5.0;
            junoy = -5.0;
        }

        if juno_pos.x == 25.0 && juno_pos.y <= 1000.0 {
            junox = 0.0;
            junoy = -5.0;
        }

        if juno_pos.x == 25.0 && juno_pos.y == 20.0 {
            junox = 5.0;
            junoy = 0.0;
        }

        if juno_pos.x == 450.0 && juno_pos.y == 20.0 {
            junox = 0.0;
            junoy = 5.0;
        }

        if juno_pos.x == 450.0 && juno_pos.y == 250.0 {
            junox = -5.0;
            junoy = 0.0;
        }

        if juno_pos.x == 250.0 && juno_pos.y == 250.0 {
            junox = 0.0;
            junoy = 5.0;
        }

        if juno_pos.x == 250.0 && juno_pos.y == 725.0 {
            junox = 5.0;
            junoy = 5.0;
        }

        if juno_pos.x == 490.0 && juno_pos.y <= 1000.0 && juno_pos.y >= 725.0 {
            junox = 0.0;
            junoy = -5.0;
        }

        if juno_pos.x == 490.0 && juno_pos.y == 60.0 {
            junox = 5.0;
            junoy = 0.0;
        }

        juno_pos.y += junoy;
        juno_pos.x += junox;
        juno.set_position(juno_pos);


// Haru AI
        let mut haru_pos = haru.pos();

        if haru_pos.x == 850.0 && haru_pos.y == 800.0 {
            harux = -5.0;
            haruy = 0.0;
        }


        if haru_pos.x == 525.0 && haru_pos.y == 800.0 {
            harux = 0.0;
            haruy = -5.0;
        }

        if haru_pos.x == 525.0 && haru_pos.y == 460.0 {
            harux = 5.0;
            haruy = 0.0;
        }

        if haru_pos.x == 850.0 && haru_pos.y == 460.0 {
            harux = 0.0;
            haruy = 5.0;
        }


        haru_pos.y += haruy;
        haru_pos.x += harux;
        haru.set_position(haru_pos);

// Louis AI

let mut louis_pos = louis.pos();

        if louis_pos.x == 950.0 && louis_pos.y == 60.0 {
            louisx = 0.0;
            louisy = 5.0;
        }

        if louis_pos.x == 950.0 && louis_pos.y == 500.0 {
            louisx = 5.0;
            louisy = 0.0;
        }

        if louis_pos.x == 1080.0 && louis_pos.y == 500.0 {
            louisx = 0.0;
            louisy = 5.0;
        }

        if louis_pos.x == 1080.0 && louis_pos.y == 1050.0 {
            louisx = -5.0;
            louisy = 0.0;
        }

        if louis_pos.x == 250.0 && louis_pos.y == 1050.0 {
            louisx = -5.0;
            louisy = -5.0;
        }

        if louis_pos.x == 25.0 && louis_pos.y <= 1000.0 {
            louisx = 0.0;
            louisy = -5.0;
        }

        if louis_pos.x == 25.0 && louis_pos.y == 20.0 {
            louisx = 5.0;
            louisy = 0.0;
        }

        if louis_pos.x == 450.0 && louis_pos.y == 20.0 {
            louisx = 0.0;
            louisy = 5.0;
        }

        if louis_pos.x == 450.0 && louis_pos.y == 250.0 {
            louisx = -5.0;
            louisy = 0.0;
        }

        if louis_pos.x == 250.0 && louis_pos.y == 250.0 {
            louisx = 0.0;
            louisy = 5.0;
        }

        if louis_pos.x == 250.0 && louis_pos.y == 725.0 {
            louisx = 5.0;
            louisy = 5.0;
        }

        if louis_pos.x == 490.0 && louis_pos.y <= 1000.0 && louis_pos.y >= 725.0 {
            louisx = 0.0;
            louisy = -5.0;
        }

        if louis_pos.x == 490.0 && louis_pos.y == 60.0 {
            louisx = 5.0;
            louisy = 0.0;
        }


        louis_pos.y += louisy;
        louis_pos.x += louisx;
        louis.set_position(louis_pos);

// Enemy Collision
        if check_collision(&legoshi, &juno, 1) {
            let junoscary_value = 1;
            let current_streak = 0;
            return ("death".to_string(), junoscary_value, haruscary_value, louisscary_value, score_value, current_streak, highscore_value, tm);

            }

        if check_collision(&legoshi, &haru, 1) {
            let haruscary_value = 1;
            let current_streak = 0;
            return ("death".to_string(), junoscary_value, haruscary_value, louisscary_value, score_value, current_streak, highscore_value, tm);


            }

        if check_collision(&legoshi, &louis, 1) {
            let louisscary_value = 1;
            let current_streak = 0;

            if highscore_value < score_value {
                highscore_value = score_value;
            }
            return ("death".to_string(), junoscary_value, haruscary_value, louisscary_value, score_value, current_streak, highscore_value, tm);


            }

// timer
let elapsed_time = start_time.elapsed();
        let remaining_time = duration - elapsed_time;
        if remaining_time.as_secs() == 0 {
            score_value += 1;
            current_streak += 1;
            highscore_value += 1;
            return ("win".to_string(), junoscary_value, haruscary_value, louisscary_value, score_value, current_streak, highscore_value, tm);
        }

        let seconds = remaining_time.as_secs() % 60;
        let minutes = remaining_time.as_secs() / 60;
        lbl_timer.set_text(format!("{}:{:02}", minutes, seconds));



        background.draw();
        legoshi.draw();
        juno.draw();
        haru.draw();
        louis.draw();
        lbl_timer.draw();

        next_frame().await;
        }
}