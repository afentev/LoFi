extern crate sfml;

use sfml::graphics::*;
use sfml::window::{Event, Key, Style, mouse};
use sfml::system::{Vector2f, Vector3f, Vector2i};
use sfml::audio::{Music, SoundStatus};
use sfml::system::{sleep, Time};
use std::io::Write;

fn moving(window: &mut RenderWindow, sprite: &mut Sprite, display_rect: &mut ConvexShape,
          player: &mut Text, history: &mut Text, about_text: &mut Text, close_text: &mut Text,
          text_lofi: &mut Text, text_radio: &mut Text, play_rect: &mut ConvexShape,
          play_text: &mut Text, play_triangle: &mut ConvexShape, nwplaying: &mut Text,
          presstxt: &mut Text, abouttxt: &mut Text, delta: f32, v: &mut Vector3f, emp: f32,
          smooth: bool) {
    let xd = v.z - v.y;
    let iters = 30;

    let y_size = 70;
    let mut cur_l = 30.0;
    let mut cur_h = 100.0;
    let step = ((y_size / 2) / iters) as f32 * 1.3;

    for q in 0..iters {
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
        window.draw(sprite);
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

        window.draw(text_lofi);
        window.draw(text_radio);
        window.draw(play_rect);
        window.draw(play_text);
        window.draw(play_triangle);
        window.draw(nwplaying);
        window.draw(presstxt);
        window.draw(abouttxt);

        window.display()
    }
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
    about_page.set_string("Репозиторий проекта: github.com/afentev/LoFi\n\nt.me/afentev");
    about_page.set_position(Vector2f::new(1100.0, 160.0));

    let mut whirligig = "p";

    let mut window = RenderWindow::new(
        (width as u32, height as u32),
        "Custom shape",
        Style::NONE,
        &Default::default(),
    );
    window.set_framerate_limit(100);
//    window.set_vertical_sync_enabled(true);
    window.set_key_repeat_enabled(false);

    let mut counter = 1;
    let mut sleeper = 0;
    let mut wait = 0;

    let mut window_moving = false;
    let mut window_prev = Vector2i::new(window.position().x, window.position().y);
    let mut click_prev = Vector2i::new(0, 0);

    loop {
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

        if 310 <= cords.x && cords.x <= 490 && 600 <= cords.y && cords.y <= 640 {
            play_rect.set_fill_color(Color::rgba(130, 130, 130, 180));
        } else {
            play_rect.set_fill_color(Color::rgba(0, 0, 0, 180));
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
                    if button == mouse::Button::Left {
                        if 729 <= x && x <= 787 && 6 <= y && y <= 32 {
                            return
                        }

                        if 310 <= x && x <= 490 && 600 <= y && y <= 640 {
                            let mut music = Music::from_file((dir.to_owned() + "/music.wav").as_str()).unwrap();
                            music.play();

                            while music.status() == SoundStatus::Playing {
                                // Display the playing position
//                                print!("\rPlaying... {:.2}", sound.playing_offset().as_seconds());
                                let _ = std::io::stdout().flush();
                                // Leave some CPU time for other processes
                                sleep(Time::milliseconds(10000));
                            }
                            println!();

                        }

                        let mut v = Vector3f::new(300.0, 505.0, 760.0);
                        if is_clicked_on_right(x, y) {
                            let prev = whirligig;
                            whirligig = "a";
                            if prev == "p" {
                                let mut v = Vector3f::new(300.0, 505.0, 760.0);

                                moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                       &mut history, &mut about_text, &mut close_text,
                                       &mut text_lofi, &mut text_radio, &mut play_rect,
                                       &mut play_text, &mut play_triangle, &mut nwplaying,
                                       &mut presstxt, &mut about_page, -35.0,
                                       &mut v, 27.0, true);
                            } else if prev == "h" {
                                moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                       &mut history, &mut about_text, &mut close_text,
                                       &mut text_lofi, &mut text_radio, &mut play_rect,
                                       &mut play_text, &mut play_triangle, &mut nwplaying,
                                       &mut presstxt, &mut about_page, -70.0, &mut v,
                                       0.0, false);
                            }
                        }
                        if is_clicked_on_left(x, y) {
                            let prev = whirligig;
                            whirligig = "h";
                            if prev == "p" {
                                let mut v = Vector3f::new(505.0, 300.0, 15.0);

                                moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                       &mut history, &mut about_text, &mut close_text,
                                       &mut text_lofi, &mut text_radio, &mut play_rect,
                                       &mut play_text, &mut play_triangle, &mut nwplaying,
                                       &mut presstxt, &mut about_page, 35.0, &mut v,
                                       11.0, true);
                            } else if prev == "a" {
                                moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                       &mut history, &mut about_text, &mut close_text,
                                       &mut text_lofi, &mut text_radio, &mut play_rect,
                                       &mut play_text, &mut play_triangle, &mut nwplaying,
                                       &mut presstxt, &mut about_page, 70.0, &mut v,
                                       0.0, false);
                            }
                        }
                        if is_clicked_on_center(x, y) {
                            let prev = whirligig;
                            whirligig = "p";
                            if prev == "h" {
                                let mut v = Vector3f::new(15.0, 250.0, 505.0);

                                moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                       &mut history, &mut about_text, &mut close_text,
                                       &mut text_lofi, &mut text_radio, &mut play_rect,
                                       &mut play_text, &mut play_triangle, &mut nwplaying,
                                       &mut presstxt, &mut about_page, -35.0, &mut v,
                                       47.0, true);
                            } else if prev == "a" {
                                let mut v = Vector3f::new(760.0, 505.0, 300.0);

                                moving(&mut window, &mut sprite, &mut display_rect, &mut player,
                                       &mut history, &mut about_text, &mut close_text,
                                       &mut text_lofi, &mut text_radio, &mut play_rect,
                                       &mut play_text, &mut play_triangle, &mut nwplaying,
                                       &mut presstxt, &mut about_page, 35.0, &mut v,
                                       -63.66, true);
                            }
                        }
                        if cords.y < 30{
                            if !window_moving {
                                window_moving = true;
                                window_prev.x = window.position().x;
                                window_prev.y = window.position().y;
                                click_prev.x = window.position().x + cords.x;
                                click_prev.y = window.position().y + cords.y;
                            }
                        }
                    }

                }
                Event::MouseButtonReleased {
                    ..
                } => {
                    if window_moving {
                        window_moving = false;
                    }
                }
                Event::MouseMoved {
                    ..
                } => {
                    if window_moving {
                        let delta = Vector2i::new(ps.x - click_prev.x, ps.y - click_prev.y);
                        window.set_position(Vector2i::new(window_prev.x + delta.x, window_prev.y + delta.y));
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
            window.draw(&play_triangle);
            window.draw(&nwplaying);
            window.draw(&presstxt);
        } else if whirligig == "a" {
            window.draw(&about_page);
        }

        if 0 <= cords.x && cords.x <= 800 && 0 <= cords.y && cords.y <= 800 && window.has_focus() {
            if (800 - cords.x) * (800 - cords.x) + cords.y * cords.y <= 125000 {
                let mut delta = ((800 - cords.x) * (800 - cords.x) + (cords.y) * (cords.y)) as f64;
                delta = (1.0 - delta / 125000.0) * 255.0;
                close_text.set_fill_color(Color::rgba(255, 255, 255, delta as u8));
            } else {
                close_text.set_fill_color(Color::rgba(255, 255, 255, 0));
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
        }

        window.draw(&player);
        window.draw(&history);
        window.draw(&about_text);
        window.draw(&close_text);

        window.display();

        if sleeper == 0 {
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
        sleeper = (sleeper + 1) % 15;
    }
}
