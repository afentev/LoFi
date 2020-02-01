extern crate sfml;
extern crate vlc;
extern crate curl;

use curl::http;

use sfml::graphics::*;
use sfml::window::{Event, Key, Style, mouse, Window};
use sfml::system::{Vector2f, Vector3f, Vector2i};

use vlc::{Instance, Media, MediaPlayer};
use std::thread::sleep;

use std::time::Duration;
use std::borrow::Borrow;

fn moving(window: &mut RenderWindow, sprite: &mut Sprite, display_rect: &mut ConvexShape,
          player: &mut Text, history: &mut Text, about_text: &mut Text, close_text: &mut Text,
          text_lofi: &mut Text, text_radio: &mut Text, play_rect: &mut ConvexShape,
          play_text: &mut Text, play_triangle: &mut ConvexShape, nwplaying: &mut Text,
          presstxt: &mut Text, abouttxt: &mut Text, delta: f32, v: &mut Vector3f, emp: f32,
          smooth: bool, rect1: &mut ConvexShape, rect2: &mut ConvexShape, rect3: &mut ConvexShape,
          rect4: &mut ConvexShape, rect5: &mut ConvexShape, track1: &mut Text,
          date1: &mut Text, track2: &mut Text, date2: &mut Text, track3: &mut Text,
          date3: &mut Text, track4: &mut Text, date4: &mut Text, track5: &mut Text,
          date5: &mut Text, pause1: &mut ConvexShape, pause2: &mut ConvexShape, music_on: bool,
          real_playing: bool, sleeper: i32, counter: i32, wait: i32, dir: &str) -> Vec<i32> {
    let xd = v.z - v.y;
    let iters = 30;

    let y_size = 70;
    let mut cur_l = 30.0;
    let mut cur_h = 100.0;
    let step = ((y_size / 2) / iters) as f32 * 1.3;

    let mut sl = sleeper;
    let mut cn = counter;
    let mut wt = wait;

    for q in 0..iters {
        let mut name: String = "/Gif/ss/".to_owned();
        name = name + &cn.to_string() + ".png";
        let image = Texture::from_file((dir.to_owned() + &name).as_str()).unwrap();

        let mut sprite = Sprite::with_texture(&image);
        sprite.set_texture_rect(&IntRect::new(0, 0, 800, 800));
        sprite.set_color(Color::rgba(255, 255, 255, 255));
        sprite.set_position(Vector2f::new(0.0, 0.0));

        if sl == 0 {
            if cn == 1 {
                if wt == 0 {
                    cn = 1 + (cn + 1) % 260;
                    wt = 1;
                } else {
                    wt = (wt + 1) % 75;
                }
            } else {
                cn = 1 + (cn + 1) % 260;
            }
        }
        if window.has_focus() {
            sl = (sl + 1) % 15;
        } else {
            sl = (sl + 1) % 1;
        }

        if smooth {
            if q < iters / 2 {
                let offset = xd * (q as f32 * 2.0) / iters as f32;
                cur_l += step;
                cur_h -= step;

                display_rect.set_point(0, Vector2f::new(v.x, cur_l));
                display_rect.set_point(1, Vector2f::new(v.x, cur_h));
                display_rect.set_point(2, Vector2f::new(v.y + offset, cur_h));
                display_rect.set_point(3, Vector2f::new(v.y + offset, cur_l));
            } else {
                let offset = xd * (q as f32 * 2.0) / iters as f32 - xd;
                cur_l -= step;
                cur_h += step;

                display_rect.set_point(0, Vector2f::new(v.z, cur_h));
                display_rect.set_point(1, Vector2f::new(v.z, cur_l));
                if q == iters / 2 {
                    display_rect.set_point(2, Vector2f::new(v.x + offset, cur_l));
                    display_rect.set_point(3, Vector2f::new(v.x + offset, cur_h));
                } else {
                    display_rect.set_point(2, Vector2f::new(v.x + offset + emp, cur_l));
                    display_rect.set_point(3, Vector2f::new(v.x + offset + emp, cur_h));
                }
            }
        }

        window.clear(Color::WHITE);
        window.draw(&sprite);
        window.draw(display_rect);
        window.draw(player);
        window.draw(history);
        window.draw(about_text);
        window.draw(close_text);

        let pos = text_lofi.position();
        text_lofi.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = text_radio.position();
        text_radio.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = play_rect.position();
        play_rect.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = play_text.position();
        play_text.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = play_triangle.position();
        play_triangle.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = nwplaying.position();
        nwplaying.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = presstxt.position();
        presstxt.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = abouttxt.position();
        abouttxt.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = track1.position();
        track1.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = date1.position();
        date1.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = track2.position();
        track2.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = date2.position();
        date2.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = track3.position();
        track3.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = date3.position();
        date3.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = track4.position();
        track4.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = date4.position();
        date4.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = track5.position();
        track5.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = date5.position();
        date5.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = rect1.position();
        rect1.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = rect2.position();
        rect2.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = rect3.position();
        rect3.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = rect4.position();
        rect4.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = rect5.position();
        rect5.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = pause1.position();
        pause1.set_position(Vector2f::new(pos.x + delta, pos.y));

        let pos = pause2.position();
        pause2.set_position(Vector2f::new(pos.x + delta, pos.y));

        window.draw(text_lofi);
        window.draw(text_radio);
        window.draw(play_rect);
        window.draw(play_text);
        window.draw(nwplaying);
        window.draw(presstxt);
        window.draw(abouttxt);
        window.draw(track1);
        window.draw(date1);
        window.draw(track2);
        window.draw(date2);
        window.draw(track3);
        window.draw(date3);
        window.draw(track4);
        window.draw(date4);
        window.draw(track5);
        window.draw(date5);
        window.draw(rect1);
        window.draw(rect2);
        window.draw(rect3);
        window.draw(rect4);
        window.draw(rect5);

        if music_on && real_playing {
            window.draw(pause1);
            window.draw(pause2);
        } else if !music_on && !real_playing {
            window.draw(play_triangle);
        }

        window.display()
    }
    vec!{sl, cn, wt}
}

fn gen_hist_rects(rect1: &mut ConvexShape, rect2: &mut ConvexShape, rect3: &mut ConvexShape,
                  rect4: &mut ConvexShape, rect5: &mut ConvexShape, track1: &mut Text,
                  date1: &mut Text, track2: &mut Text, date2: &mut Text, track3: &mut Text,
                  date3: &mut Text, track4: &mut Text, date4: &mut Text, track5: &mut Text,
                  date5: &mut Text) {

    rect1.set_point(0, Vector2f::new(-970.0, 170.0));
    rect1.set_point(1, Vector2f::new(-970.0, 260.0));
    rect1.set_point(2, Vector2f::new(-330.0, 260.0));
    rect1.set_point(3, Vector2f::new(-330.0, 170.0));

    rect2.set_point(0, Vector2f::new(-970.0, 290.0));
    rect2.set_point(1, Vector2f::new(-970.0, 380.0));
    rect2.set_point(2, Vector2f::new(-330.0, 380.0));
    rect2.set_point(3, Vector2f::new(-330.0, 290.0));

    rect3.set_point(0, Vector2f::new(-970.0, 410.0));
    rect3.set_point(1, Vector2f::new(-970.0, 500.0));
    rect3.set_point(2, Vector2f::new(-330.0, 500.0));
    rect3.set_point(3, Vector2f::new(-330.0, 410.0));

    rect4.set_point(0, Vector2f::new(-970.0, 530.0));
    rect4.set_point(1, Vector2f::new(-970.0, 620.0));
    rect4.set_point(2, Vector2f::new(-330.0, 620.0));
    rect4.set_point(3, Vector2f::new(-330.0, 530.0));

    rect5.set_point(0, Vector2f::new(-970.0, 650.0));
    rect5.set_point(1, Vector2f::new(-970.0, 740.0));
    rect5.set_point(2, Vector2f::new(-330.0, 740.0));
    rect5.set_point(3, Vector2f::new(-330.0, 650.0));

    rect1.set_fill_color(Color::rgba(10, 10, 10, 100));
    rect1.set_outline_thickness(6.0);
    rect1.set_outline_color(Color::rgba(10, 10, 10, 165));

    rect2.set_fill_color(Color::rgba(10, 10, 10, 100));
    rect2.set_outline_thickness(6.0);
    rect2.set_outline_color(Color::rgba(10, 10, 10, 165));

    rect3.set_fill_color(Color::rgba(10, 10, 10, 100));
    rect3.set_outline_thickness(6.0);
    rect3.set_outline_color(Color::rgba(10, 10, 10, 165));

    rect4.set_fill_color(Color::rgba(10, 10, 10, 100));
    rect4.set_outline_thickness(6.0);
    rect4.set_outline_color(Color::rgba(10, 10, 10, 165));

    rect5.set_fill_color(Color::rgba(10, 10, 10, 100));
    rect5.set_outline_thickness(6.0);
    rect5.set_outline_color(Color::rgba(10, 10, 10, 165));

    track1.set_string("The Beatles - Let it be");
    date1.set_string("a long time ago");
    date1.set_fill_color(Color::rgba(250, 250, 250, 255));

    track2.set_string("Del Shannon - Runaway");
    date2.set_string("a long time ago");
    date2.set_fill_color(Color::rgba(250, 250, 250, 255));

    track3.set_string("Del Shannon - Runaway");
    date3.set_string("a long time ago");
    date3.set_fill_color(Color::rgba(250, 250, 250, 255));

    track4.set_string("Del Shannon - Runaway");
    date4.set_string("a long time ago");
    date4.set_fill_color(Color::rgba(250, 250, 250, 255));

    track5.set_string("Del Shannon - Runaway");
    date5.set_string("a long time ago");
    date5.set_fill_color(Color::rgba(250, 250, 250, 255));

    track1.set_position(Vector2f::new(-960.0, 180.0));
    date1.set_position(Vector2f::new(-958.0, 230.0));
    track2.set_position(Vector2f::new(-960.0, 300.0));
    date2.set_position(Vector2f::new(-958.0, 350.0));
    track3.set_position(Vector2f::new(-960.0, 420.0));
    date3.set_position(Vector2f::new(-958.0, 470.0));
    track4.set_position(Vector2f::new(-960.0, 540.0));
    date4.set_position(Vector2f::new(-958.0, 590.0));
    track5.set_position(Vector2f::new(-960.0, 660.0));
    date5.set_position(Vector2f::new(-958.0, 710.0));

}

fn is_clicked_on_left(x: i32, y: i32) -> bool {
    45 <= x && x <= 225 && 40 <= y && y <= 92
}

fn is_clicked_on_center(x: i32, y: i32) -> bool {
    310 <= x && x <= 495 && 40 <= y && y <= 92
}

fn is_clicked_on_right(x: i32, y: i32) -> bool {
   560 <= x && x <= 770 && 40 <= y && y <= 92
}

fn main() {
    let url = "http://opml.radiotime.com/Search.ashx?query=Lofi%20HipHop%20Radio%20/%20Chillsky";

    let instance = Instance::new().unwrap();
    let md = Media::new_location(&instance, "http://hyades.shoutca.st:8043/stream").unwrap();

    //http://hyades.shoutca.st:8043/stream
    //http://streamingv2.shoutcast.com/BeyondMetal
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    let dir = "/home/afentev/CLionProjects/lofi_rust";
    let width = 800;
    let height = 800;

    let jb = Font::from_file((dir.to_owned() + "/jb.ttf").as_str()).unwrap();
    let _fabryka = Font::from_file((dir.to_owned() + "/fabryka.ttf").as_str()).unwrap();

    let mut text_lofi = Text::new("lo-fi", &jb, 100 as u32);
    text_lofi.set_fill_color(Color::WHITE);
    text_lofi.set_outline_color(Color::WHITE);
    text_lofi.set_outline_thickness(0.6);
    text_lofi.set_position(Vector2f::new(251 as f32, (height / 5) as f32));

    let mut text_radio = Text::new("radio", &jb, 55);
    text_radio.set_fill_color(Color::WHITE);
    text_radio.set_outline_color(Color::WHITE);
    text_radio.set_outline_thickness(0.8);
    text_radio.set_position(Vector2f::new(389.0, (height / 2 - 140) as f32));

    let mut close_text = Text::new("close", &jb, 20);
    close_text.set_fill_color(Color::WHITE);
    close_text.set_outline_color(Color::rgba(200, 200, 200, 200));
    close_text.set_position(Vector2f::new(727.0, 5.0));

    let mut play_rect = ConvexShape::new(4);
    play_rect.set_point(0, Vector2f::new(310.0, 600.0));
    play_rect.set_point(1, Vector2f::new(490.0, 600.0));
    play_rect.set_point(2, Vector2f::new(490.0, 640.0));
    play_rect.set_point(3, Vector2f::new(310.0, 640.0));
    play_rect.set_fill_color(Color::rgba(0, 0, 0, 180));
    play_rect.set_outline_thickness(3.0);
    play_rect.set_outline_color(Color::WHITE);

    let mut play_text = Text::new("PLAY", &jb, 30);
    play_text.set_fill_color(Color::WHITE);
    play_text.set_position(Vector2f::new(390.0, 601 as f32));

    let mut play_triangle = ConvexShape::new(3);
    play_triangle.set_point(0, Vector2f::new(345.0, 608.0));
    play_triangle.set_point(1, Vector2f::new(345.0, 631.0));
    play_triangle.set_point(2, Vector2f::new(363.0, 619.5));
    play_triangle.set_fill_color(Color::WHITE);

    let mut nwplaying = Text::new("Now playing:", &_fabryka, 26);
    nwplaying.set_fill_color(Color::WHITE);
    nwplaying.set_outline_color(Color::rgba(100, 100, 100, 80));
    nwplaying.set_outline_thickness(0.3);
    nwplaying.set_position(Vector2f::new(331.0, 445.0));

    let mut presstxt = Text::new("Press play to start", &_fabryka, 35);
    presstxt.set_fill_color(Color::WHITE);
    presstxt.set_outline_color(Color::rgba(100, 100, 100, 80));
    presstxt.set_outline_thickness(0.5);
    presstxt.set_position(Vector2f::new(245.0, 475.0));

    let mut player = Text::new("PLAYER", &jb, 35);
    player.set_fill_color(Color::WHITE);
    player.set_outline_color(Color::rgba(200, 200, 200, 200));
    player.set_position(Vector2f::new(340.0, 45.0));

    let mut history = Text::new("HISTORY", &jb, 35);
    history.set_fill_color(Color::WHITE);
    history.set_outline_color(Color::rgba(200, 200, 200, 200));
    history.set_position(Vector2f::new(60.0, 45.0));

    let mut about_text = Text::new("ABOUT", &jb, 35);
    about_text.set_fill_color(Color::WHITE);
    about_text.set_outline_color(Color::rgba(200, 200, 200, 200));
    about_text.set_position(Vector2f::new(610.0, 45.0));

    let mut display_rect = ConvexShape::new(4);
    display_rect.set_fill_color(Color::rgba(255, 255, 255, 110));

    let mut about_page = Text::new("", &_fabryka, 30);
    about_page.set_outline_thickness(2.5);
    about_page.set_outline_color(Color::rgba(45, 45, 45, 200));
    about_page.set_string("Github: github.com/afentev/LoFi\n\nDev channel: t.me/lofiradiodesktop\n\nContact: t.me/afentev");
    about_page.set_position(Vector2f::new(1100.0, 160.0));

    let mut pause1 = ConvexShape::new(4);
    pause1.set_point(0, Vector2f::new(344.0, 631.0));
    pause1.set_point(1, Vector2f::new(352.0, 631.0));
    pause1.set_point(2, Vector2f::new(352.0, 608.0));
    pause1.set_point(3, Vector2f::new(344.0, 608.0));

    let mut pause2 = ConvexShape::new(4);
    pause2.set_point(0, Vector2f::new(356.0, 631.0));
    pause2.set_point(1, Vector2f::new(364.0, 631.0));
    pause2.set_point(2, Vector2f::new(364.0, 608.0));
    pause2.set_point(3, Vector2f::new(356.0, 608.0));

    pause1.set_fill_color(Color::WHITE);

    let mut whirligig = "p";

    let mut window = RenderWindow::new(
        (width as u32, height as u32),
        "LoFi Radio",
        Style::TITLEBAR,
        &Default::default(),
    );
    window.set_icon(32, 32, Image::from_file((dir.to_owned() + "/icon.jpg").as_str()).unwrap().pixel_data());
//    window.set_vertical_sync_enabled(true);
    window.set_key_repeat_enabled(false);

    let mut counter = 1;
    let mut sleeper = 0;
    let mut wait = 0;
    let mut missing = 0;

//    let history_data = vec!{vec!{"Undefined", "Undefined", "a long time ago"},
//                                          vec!{"Undefined", "Undefined", "a long time ago"},
//                                          vec!{"Undefined", "Undefined", "a long time ago"},
//                                          vec!{"Undefined", "Undefined", "a long time ago"},
//                                          vec!{"Undefined", "Undefined", "a long time ago"}};

    let mut rect1 = ConvexShape::new(4);
    let mut rect2 = ConvexShape::new(4);
    let mut rect3 = ConvexShape::new(4);
    let mut rect4 = ConvexShape::new(4);
    let mut rect5 = ConvexShape::new(4);

    let mut track1 = Text::new("", &jb, 23);
    let mut date1 = Text::new("", &jb, 15);
    let mut track2 = Text::new("", &jb, 23);
    let mut date2 = Text::new("", &jb, 15);
    let mut track3 = Text::new("", &jb, 23);
    let mut date3 = Text::new("", &jb, 15);
    let mut track4 = Text::new("", &jb, 23);
    let mut date4 = Text::new("", &jb, 15);
    let mut track5 = Text::new("", &jb, 23);
    let mut date5 = Text::new("", &jb, 15);

    gen_hist_rects(&mut rect1, &mut rect2, &mut rect3, &mut rect4, &mut rect5, &mut track1,
                   &mut date1, &mut track2, &mut date2, &mut track3,  &mut date3, &mut track4,
                   &mut date4, &mut track5, &mut date5);

    let mut is_music_playing = false;
    let mut real_playing= false;

    let mut prev = window.has_focus();

    loop {
        if window.has_focus() {
            window.set_framerate_limit(160);
        } else {
            window.set_framerate_limit(3);
        }
        real_playing = mdp.is_playing();

        if whirligig == "p" {
            if !is_music_playing {
                play_text.set_string("PLAY");
                play_text.set_position(Vector2f::new(390.0, 601.0));
                play_rect.set_point(0, Vector2f::new(310.0, 600.0));
                play_rect.set_point(1, Vector2f::new(490.0, 600.0));
                play_rect.set_point(2, Vector2f::new(490.0, 640.0));
                play_rect.set_point(3, Vector2f::new(310.0, 640.0));
            } else if mdp.is_playing(){
                play_text.set_string("PAUSE");
                play_text.set_position(Vector2f::new(383.0, 601.0));
                play_rect.set_point(0, Vector2f::new(310.0, 600.0));
                play_rect.set_point(1, Vector2f::new(490.0, 600.0));
                play_rect.set_point(2, Vector2f::new(490.0, 640.0));
                play_rect.set_point(3, Vector2f::new(310.0, 640.0));
            } else {
                play_text.set_string("Wait, connecting...");
                play_text.set_position(Vector2f::new(232.0, 601.0));
                play_rect.set_point(0, Vector2f::new(210.0, 600.0));
                play_rect.set_point(1, Vector2f::new(590.0, 600.0));
                play_rect.set_point(2, Vector2f::new(590.0, 640.0));
                play_rect.set_point(3, Vector2f::new(210.0, 640.0));
            }
        }

        window.clear(Color::WHITE);

        if whirligig == "p" {
            player.set_outline_thickness(1.0);
            player.set_outline_color(Color::WHITE);
            history.set_outline_thickness(0.0);
            history.set_outline_color(Color::rgba(200, 200, 200, 200));
            about_text.set_outline_thickness(0.0);
            about_text.set_outline_color(Color::rgba(200, 200, 200, 200));

            display_rect.set_point(0, Vector2f::new(300.0, 30.0));
            display_rect.set_point(1, Vector2f::new(300.0, 100.0));
            display_rect.set_point(2, Vector2f::new(505.0, 100.0));
            display_rect.set_point(3, Vector2f::new(505.0, 30.0));
        } else if whirligig == "h" {
            history.set_outline_thickness(1.0);
            history.set_outline_color(Color::WHITE);
            player.set_outline_thickness(0.0);
            player.set_outline_color(Color::rgba(200, 200, 200, 200));
            about_text.set_outline_thickness(0.0);
            about_text.set_outline_color(Color::rgba(200, 200, 200, 200));

            display_rect.set_point(0, Vector2f::new(15.0, 30.0));
            display_rect.set_point(1, Vector2f::new(15.0, 100.0));
            display_rect.set_point(2, Vector2f::new(250.0, 100.0));
            display_rect.set_point(3, Vector2f::new(250.0, 30.0));
        } else if whirligig == "a" {
            about_text.set_outline_thickness(1.0);
            about_text.set_outline_color(Color::WHITE);
            history.set_outline_thickness(0.0);
            history.set_outline_color(Color::rgba(200, 200, 200, 200));
            player.set_outline_thickness(0.0);
            player.set_outline_color(Color::rgba(200, 200, 200, 200));

            about_page.set_position(Vector2f::new(50.0, 160.0));

            display_rect.set_point(0, Vector2f::new(565.0, 30.0));
            display_rect.set_point(1, Vector2f::new(565.0, 100.0));
            display_rect.set_point(2, Vector2f::new(760.0, 100.0));
            display_rect.set_point(3, Vector2f::new(760.0, 30.0));
        }

        let cords = window.mouse_position();
        let ps = mouse::desktop_position();

        if is_music_playing && real_playing || !is_music_playing && !real_playing && window.has_focus() {
            if 310 <= cords.x && cords.x <= 490 && 600 <= cords.y && cords.y <= 640 {
                    play_rect.set_fill_color(Color::rgba(130, 130, 130, 180));
                } else {
                    play_rect.set_fill_color(Color::rgba(0, 0, 0, 180));
                }
        }

        let mut name: String = "/Gif/ss/".to_owned();
        name = name + &counter.to_string() + ".png";
        let image = Texture::from_file((dir.to_owned() + &name).as_str()).unwrap();

        let mut sprite = Sprite::with_texture(&image);
        sprite.set_texture_rect(&IntRect::new(0, 0, width, height));
        sprite.set_color(Color::rgba(255, 255, 255, 255));
        sprite.set_position(Vector2f::new(0.0, 0.0));

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                Event::MouseButtonPressed {
                    button, x, y
                } => {
                    if button == mouse::Button::Left && window.has_focus() && prev{
                        if 729 <= x && x <= 787 && 6 <= y && y <= 32 {
                            return;
                        }

                        if 310 <= x && x <= 490 && 600 <= y && y <= 640 {
                            if is_music_playing && real_playing || !is_music_playing && !real_playing {
                                if is_music_playing {
                                    mdp.stop();
                                    play_text.set_string("PLAY");
                                    play_text.set_position(Vector2f::new(390.0, 601.0));
                                    is_music_playing = false;
                                } else {
                                    mdp.play().unwrap();
                                    play_text.set_string("PAUSE");
                                    play_text.set_position(Vector2f::new(383.0, 601.0));
                                    is_music_playing = true;
                                    play_rect.set_fill_color(Color::rgba(0, 0, 0, 180));
                                }
                            }
                            real_playing = mdp.is_playing();
                        }

                        let mut v = Vector3f::new(300.0, 505.0, 760.0);
                        if is_clicked_on_right(x, y) {
                            let prev = whirligig;
                            whirligig = "a";
                            if prev == "p" {
                                let mut v = Vector3f::new(300.0, 505.0, 760.0);

                                let t = moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                                        &mut history, &mut about_text, &mut close_text,
                                                        &mut text_lofi, &mut text_radio, &mut play_rect,
                                                        &mut play_text, &mut play_triangle, &mut nwplaying,
                                                        &mut presstxt, &mut about_page, -35.0,
                                                        &mut v, 27.0, true, &mut rect1, &mut rect2,
                                                        &mut rect3, &mut rect4, &mut rect5, &mut track1,  &mut date1,
                                                        &mut track2, &mut date2, &mut track3,  &mut date3,
                                                        &mut track4, &mut date4, &mut track5, &mut date5,
                                                        &mut pause1, &mut pause2, is_music_playing,
                                                        real_playing, sleeper, counter, wait, dir);
                                sleeper = t[0];
                                counter = t[1];
                                wait = t[2];
                            } else if prev == "h" {
                                let t = moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                               &mut history, &mut about_text, &mut close_text,
                                               &mut text_lofi, &mut text_radio, &mut play_rect,
                                               &mut play_text, &mut play_triangle, &mut nwplaying,
                                               &mut presstxt, &mut about_page, -70.0, &mut v,
                                               0.0, false, &mut rect1, &mut rect2,
                                               &mut rect3, &mut rect4, &mut rect5, &mut track1,  &mut date1,
                                               &mut track2, &mut date2, &mut track3,  &mut date3,
                                               &mut track4, &mut date4, &mut track5, &mut date5,
                                               &mut pause1, &mut pause2, is_music_playing,
                                               real_playing, sleeper, counter, wait, dir);
                                sleeper = t[0];
                                counter = t[1];
                                wait = t[2];
                            }
                        }
                        if is_clicked_on_left(x, y) {
                            let prev = whirligig;
                            whirligig = "h";
                            if prev == "p" {
                                let mut v = Vector3f::new(505.0, 300.0, 15.0);

                                let t = moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                               &mut history, &mut about_text, &mut close_text,
                                               &mut text_lofi, &mut text_radio, &mut play_rect,
                                               &mut play_text, &mut play_triangle, &mut nwplaying,
                                               &mut presstxt, &mut about_page, 35.0, &mut v,
                                               11.0, true, &mut rect1, &mut rect2,
                                               &mut rect3, &mut rect4, &mut rect5, &mut track1,  &mut date1,
                                               &mut track2, &mut date2, &mut track3,  &mut date3,
                                               &mut track4, &mut date4, &mut track5, &mut date5,
                                               &mut pause1, &mut pause2, is_music_playing,
                                               real_playing, sleeper, counter, wait, dir);
                                sleeper = t[0];
                                counter = t[1];
                                wait = t[2];
                            } else if prev == "a" {
                                let t = moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                               &mut history, &mut about_text, &mut close_text,
                                               &mut text_lofi, &mut text_radio, &mut play_rect,
                                               &mut play_text, &mut play_triangle, &mut nwplaying,
                                               &mut presstxt, &mut about_page, 70.0, &mut v,
                                               0.0, false, &mut rect1, &mut rect2,
                                               &mut rect3, &mut rect4, &mut rect5, &mut track1,  &mut date1,
                                               &mut track2, &mut date2, &mut track3,  &mut date3,
                                               &mut track4, &mut date4, &mut track5, &mut date5,
                                               &mut pause1, &mut pause2, is_music_playing,
                                               real_playing, sleeper, counter, wait, dir);
                                sleeper = t[0];
                                counter = t[1];
                                wait = t[2];
                            }
                        }
                        if is_clicked_on_center(x, y) {
                            let prev = whirligig;
                            whirligig = "p";
                            if prev == "h" {
                                let mut v = Vector3f::new(15.0, 250.0, 505.0);

                                let t = moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                               &mut history, &mut about_text, &mut close_text,
                                               &mut text_lofi, &mut text_radio, &mut play_rect,
                                               &mut play_text, &mut play_triangle, &mut nwplaying,
                                               &mut presstxt, &mut about_page, -35.0, &mut v,
                                               47.0, true, &mut rect1, &mut rect2,
                                               &mut rect3, &mut rect4, &mut rect5, &mut track1,  &mut date1,
                                               &mut track2, &mut date2, &mut track3,  &mut date3,
                                               &mut track4, &mut date4, &mut track5, &mut date5,
                                               &mut pause1, &mut pause2, is_music_playing,
                                               real_playing, sleeper, counter, wait, dir);
                                sleeper = t[0];
                                counter = t[1];
                                wait = t[2];
                            } else if prev == "a" {
                                let mut v = Vector3f::new(760.0, 505.0, 300.0);

                                let t = moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                               &mut history, &mut about_text, &mut close_text,
                                               &mut text_lofi, &mut text_radio, &mut play_rect,
                                               &mut play_text, &mut play_triangle, &mut nwplaying,
                                               &mut presstxt, &mut about_page, 35.0, &mut v,
                                               -63.66, true, &mut rect1, &mut rect2,
                                               &mut rect3, &mut rect4, &mut rect5, &mut track1,  &mut date1,
                                               &mut track2, &mut date2, &mut track3,  &mut date3,
                                               &mut track4, &mut date4, &mut track5, &mut date5,
                                               &mut pause1, &mut pause2, is_music_playing,
                                               real_playing, sleeper, counter, wait, dir);
                                sleeper = t[0];
                                counter = t[1];
                                wait = t[2];
                            }
                        }
                    }

                },
                _ => {}
            }
        }

        window.draw(&sprite);
        window.draw(&display_rect);

        if whirligig == "p" {
            window.draw(&text_lofi);
            window.draw(&text_radio);
            window.draw(&play_rect);
            window.draw(&play_text);
            window.draw(&nwplaying);
            window.draw(&presstxt);
            if is_music_playing && real_playing{
                window.draw(&pause1);
                window.draw(&pause2);
            } else if !is_music_playing && !real_playing {
                window.draw(&play_triangle);
            }
        } else if whirligig == "a" {
            window.draw(&about_page);
        } else if whirligig == "h" {
            window.draw(&rect1);
            window.draw(&rect2);
            window.draw(&rect3);
            window.draw(&rect4);
            window.draw(&rect5);
            window.draw(&track1);
            window.draw(&date1);
            window.draw(&track2);
            window.draw(&date2);
            window.draw(&track3);
            window.draw(&date3);
            window.draw(&track4);
            window.draw(&date4);
            window.draw(&track5);
            window.draw(&date5);
        }

        if window.has_focus() && prev {
            if 0 <= cords.x && cords.x <= 800 && 0 <= cords.y && cords.y <= 800 {
                if (800 - cords.x) * (800 - cords.x) + cords.y * cords.y <= 125000 {
                    let mut delta = ((800 - cords.x) * (800 - cords.x) + (cords.y) * (cords.y)) as f64;
                    delta = (1.0 - delta / 125000.0) * 255.0;
                    close_text.set_fill_color(Color::rgba(255, 255, 255, delta as u8));
                } else {
                    close_text.set_fill_color(Color::rgba(255, 255, 255, 0));
                    close_text.set_outline_thickness(0.0);
                }
                if 729 <= cords.x && cords.x <= 787 && 6 <= cords.y && cords.y <= 32 {
                    close_text.set_outline_thickness(0.9);
                } else {
                    close_text.set_outline_thickness(0.0);
                }

                if 560 <= cords.x && cords.x <= 770 && 40 <= cords.y && cords.y <= 92 {
                    if whirligig != "a" {
                        about_text.set_outline_thickness(1.1);
                    }
                }

                if 45 <= cords.x && cords.x <= 225 && 40 <= cords.y && cords.y <= 92 {
                    if whirligig != "h" {
                        history.set_outline_thickness(1.1);
                    }
                }
                if 310 <= cords.x && cords.x <= 495 && 40 <= cords.y && cords.y <= 92 {
                    if whirligig != "p" {
                        player.set_outline_thickness(1.1);
                    }
                }

            } else {
                close_text.set_fill_color(Color::rgba(255, 255, 255, 0));
                close_text.set_outline_thickness(0.0);
            }
        }

        window.draw(&player);
        window.draw(&history);
        window.draw(&about_text);
        window.draw(&close_text);

        window.display();

        if sleeper == 0 {
            missing = (1 + missing) % 15;
            if missing == 1 {
                if real_playing {
                    let resp = http::handle().get(url).exec().unwrap();
                    let body = std::str::from_utf8(resp.get_body()).unwrap();
                    let lhs = body.find("subtext=").unwrap() + 9;
                    let rhs = body[lhs..].find('"').unwrap();
                    let title =  &body[lhs..lhs + rhs];
                    presstxt.set_string(title);
                    if whirligig == "p" {
                        presstxt.set_position(Vector2f::new(245.0 - 2.0 * title.len() as f32, 475.0));
                    }
                } else {
                    presstxt.set_string("Press play to start");
                    if whirligig == "p" {
                        presstxt.set_position(Vector2f::new(245.0, 475.0));
                    }
                }
            }
            if counter == 1 {
                if wait == 0 {
                    counter = 1 + (counter + 1) % 260;
                    wait = 1;
                } else {
                    wait = (wait + 1) % 75;
                }
            } else {
                counter = 1 + (counter + 1) % 260;
            }
        }
        
        if window.has_focus() {
            sleeper = (sleeper + 1) % 15;
        } else {
            sleeper = (sleeper + 1) % 1;
        }

        prev = window.has_focus();

//        sleep(Duration::from_nanos(1000000000));
    }
}
