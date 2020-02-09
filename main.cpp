#include "SFML/Graphics.hpp"
#include "vlcpp/vlc.hpp"
#include <curlpp/cURLpp.hpp>
#include <curlpp/Options.hpp>

curlpp::Cleanup myCleanup;

#include <random>
#include <iostream>
#include <vector>
#include <ctime>
#include <string>
#include <sstream>


std::string how_long(unsigned long long time1) {
    std::time_t start = std::time(nullptr);

    unsigned long long delta = start - time1;

    if (delta < 60) {
        return "few second ago";
    } else if (delta < 1800) {
        return "few minutes ago";
    } else if (delta < 3600) {
        return "half an hour ago";
    } else {
        return "a long time ago";
    }
}

void moving(sf::RenderWindow* window, sf::ConvexShape* display_rect,
            sf::Text* player, sf::Text* history, sf::Text* about_text, sf::Text* close_text,
            sf::Text* text_lofi, sf::Text* text_radio, sf::ConvexShape* play_rect,
            sf::Text* play_text, sf::ConvexShape* play_triangle, sf::Text* nwplaying,
            sf::Text* presstxt, sf::Text* abouttxt, double delta, sf::Vector3f v, double emp,
            bool smooth, sf::ConvexShape* rect1, sf::ConvexShape* rect2, sf::ConvexShape* rect3,
            sf::ConvexShape* rect4, sf::ConvexShape* rect5, sf::Text* track1,
            sf::Text* date1, sf::Text* track2, sf::Text* date2, sf::Text* track3,
            sf::Text* date3, sf::Text* track4, sf::Text* date4, sf::Text* track5,
            sf::Text* date5, sf::ConvexShape* pause1, sf::ConvexShape* pause2, bool music_on,
            bool real_playing, int* sleeper, int* counter, int* wait, std::string dir) {
    double xd = v.z - v.y;
    int iters = 30;

    int y_size = 70;
    double cur_l = 30.0;
    double cur_h = 100.0;
    double step = ((y_size / 2) / iters) * 1.3;

    for (int q = 0; q < iters; ++q) {
        std::string name = "/Gif/ss/";
        name += std::to_string(*counter) + ".png";
        sf::Texture image = sf::Texture();
        image.loadFromFile(dir + name);

        sf::Sprite sprite = sf::Sprite();
        sprite.setTexture(image);
        sprite.setTextureRect(sf::IntRect(0, 0, 800, 800));
        sprite.setColor(sf::Color(255, 255, 255, 255));
        sprite.setPosition(sf::Vector2f(0.0, 0.0));

                if (*sleeper == 0) {
                    if (*counter == 1) {
                        if (*wait == 0) {
                            *counter = 1 + (*counter + 1) % 260;
                            *wait = 1;
                        } else {
                            *wait = (*wait + 1) % 75;
                        }
                    } else {
                        *counter = 1 + (*counter + 1) % 260;
                    }
                }
                if (window->hasFocus()) {
                    *sleeper = (*sleeper + 1) % 15;
                } else {
                    *sleeper = (*sleeper + 1) % 1;
                }

        if (smooth) {
            if (q < iters / 2) {
                double offset = xd * (q * 2.0) / iters;
                cur_l += step;
                cur_h -= step;

                display_rect->setPoint(0, sf::Vector2f(v.x, cur_l));
                display_rect->setPoint(1, sf::Vector2f(v.x, cur_h));
                display_rect->setPoint(2, sf::Vector2f(v.y + offset, cur_h));
                display_rect->setPoint(3, sf::Vector2f(v.y + offset, cur_l));
            } else {
                double offset = xd * (q * 2.0) / iters - xd;
                cur_l -= step;
                cur_h += step;

                display_rect->setPoint(0, sf::Vector2f(v.z, cur_h));
                display_rect->setPoint(1, sf::Vector2f(v.z, cur_l));
                if (q == iters / 2) {
                    display_rect->setPoint(2, sf::Vector2f(v.x + offset, cur_l));
                    display_rect->setPoint(3, sf::Vector2f(v.x + offset, cur_h));
                } else {
                    display_rect->setPoint(2, sf::Vector2f(v.x + offset + emp, cur_l));
                    display_rect->setPoint(3, sf::Vector2f(v.x + offset + emp, cur_h));
                }
            }
        }

        window->clear(sf::Color::White);
        window->draw(sprite);
        window->draw(*display_rect);
        window->draw(*player);
        window->draw(*history);
        window->draw(*about_text);
        window->draw(*close_text);

        sf::Vector2f pos = text_lofi->getPosition();
        text_lofi->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = text_radio->getPosition();
        text_radio->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = play_rect->getPosition();
        play_rect->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = play_text->getPosition();
        play_text->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = play_triangle->getPosition();
        play_triangle->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = nwplaying->getPosition();
        nwplaying->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = presstxt->getPosition();
        presstxt->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = abouttxt->getPosition();
        abouttxt->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = track1->getPosition();
        track1->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = date1->getPosition();
        date1->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = track2->getPosition();
        track2->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = date2->getPosition();
        date2->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = track3->getPosition();
        track3->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = date3->getPosition();
        date3->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = track4->getPosition();
        track4->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = date4->getPosition();
        date4->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = track5->getPosition();
        track5->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = date5->getPosition();
        date5->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = rect1->getPosition();
        rect1->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = rect2->getPosition();
        rect2->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = rect3->getPosition();
        rect3->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = rect4->getPosition();
        rect4->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = rect5->getPosition();
        rect5->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = pause1->getPosition();
        pause1->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        pos = pause2->getPosition();
        pause2->setPosition(sf::Vector2f(pos.x + delta, pos.y));

        window->draw(*text_lofi);
        window->draw(*text_radio);
        window->draw(*play_rect);
        window->draw(*play_text);
        window->draw(*nwplaying);
        window->draw(*presstxt);
        window->draw(*abouttxt);
        window->draw(*track1);
        window->draw(*date1);
        window->draw(*track2);
        window->draw(*date2);
        window->draw(*track3);
        window->draw(*date3);
        window->draw(*track4);
        window->draw(*date4);
        window->draw(*track5);
        window->draw(*date5);
        window->draw(*rect1);
        window->draw(*rect2);
        window->draw(*rect3);
        window->draw(*rect4);
        window->draw(*rect5);

        if (music_on && real_playing) {
            window->draw(*pause1);
            window->draw(*pause2);
        } else if (!music_on && !real_playing) {
            window->draw(*play_triangle);
        }

        window->display();
    }
}

void gen_hist_rects(sf::Text* date1, sf::Text* date2, sf::Text* date3, sf::Text* date4,
                    sf::Text* date5, std::vector<unsigned long long>::iterator hist_time, bool is_on) {

    if (is_on) {
        date1->setString("right now");
    } else {
        date1->setString(how_long(*(hist_time + 4)));
    }

    date2->setString(how_long(*(hist_time + 3)));
    date3->setString(how_long(*(hist_time + 2)));
    date4->setString(how_long(*(hist_time + 1)));
    date5->setString(how_long(*(hist_time + 0)));

}

bool is_clicked_on_left(int x, int y) {
    return (45 <= x && x <= 225 && 40 <= y && y <= 92);
}

bool is_clicked_on_center(int x, int y) {
    return (310 <= x && x <= 495 && 40 <= y && y <= 92);
}

bool is_clicked_on_right(int x, int y) {
    return (560 <= x && x <= 770 && 40 <= y && y <= 92);
}

int main() {
    const std::string url = "http://opml.radiotime.com/Search.ashx?query=Lofi%20HipHop%20Radio%20/%20Chillsky";

    VLC::Instance instance = VLC::Instance(0, nullptr);
    VLC::Media md = VLC::Media(instance, "http://hyades.shoutca.st:8043/stream", VLC::Media::FromLocation);
    VLC::MediaPlayer mdp = VLC::MediaPlayer(md);

//    let md = Media::new_location(&instance, "http://hyades.shoutca.st:8043/stream").unwrap();

    //http://hyades.shoutca.st:8043/stream
    //http://streamingv2.shoutcast.com/BeyondMetal
//    let mdp = MediaPlayer::new(&instance).unwrap();
//    mdp.set_media(&md);

    std::string dir = "/home/afentev/CLionProjects/lofi_rust";
    int width = 800;
    int height = 800;

    sf::Font jb = sf::Font();
    jb.loadFromFile(dir + "/jb.ttf");

    sf::Font _fabryka = sf::Font();
    _fabryka.loadFromFile(dir + "/fabryka.ttf");

    sf::Text text_lofi = sf::Text();
    text_lofi.setString("lo-fi");
    text_lofi.setFont(jb);
    text_lofi.setCharacterSize(100);
    text_lofi.setFillColor(sf::Color::White);
    text_lofi.setOutlineColor(sf::Color::White);
    text_lofi.setOutlineThickness(0.6);
    text_lofi.setPosition(sf::Vector2f(251, (height / 5)));

    sf::Text text_radio = sf::Text();
    text_radio.setString("radio");
    text_radio.setFont(jb);
    text_radio.setCharacterSize(55);
    text_radio.setFillColor(sf::Color::White);
    text_radio.setOutlineColor(sf::Color::White);
    text_radio.setOutlineThickness(0.8);
    text_radio.setPosition(sf::Vector2f(389.0, (height / 2 - 140) ));

    sf::Text close_text = sf::Text();
    close_text.setString("close");
    close_text.setFont(jb);
    close_text.setCharacterSize(20);
    close_text.setFillColor(sf::Color::White);
    close_text.setOutlineColor(sf::Color(200, 200, 200, 200));
    close_text.setPosition(sf::Vector2f(727.0, 5.0));

    sf::ConvexShape play_rect = sf::ConvexShape(4);
    play_rect.setPoint(0, sf::Vector2f(310.0, 600.0));
    play_rect.setPoint(1, sf::Vector2f(490.0, 600.0));
    play_rect.setPoint(2, sf::Vector2f(490.0, 640.0));
    play_rect.setPoint(3, sf::Vector2f(310.0, 640.0));
    play_rect.setFillColor(sf::Color(0, 0, 0, 180));
    play_rect.setOutlineThickness(3.0);
    play_rect.setOutlineColor(sf::Color::White);

    sf::Text play_text = sf::Text();
    play_text.setString("PLAY");
    play_text.setFont(jb);
    play_text.setCharacterSize(30);
    play_text.setFillColor(sf::Color::White);
    play_text.setPosition(sf::Vector2f(390.0, 601 ));

    sf::ConvexShape play_triangle = sf::ConvexShape(3);
    play_triangle.setPoint(0, sf::Vector2f(345.0, 608.0));
    play_triangle.setPoint(1, sf::Vector2f(345.0, 631.0));
    play_triangle.setPoint(2, sf::Vector2f(363.0, 619.5));
    play_triangle.setFillColor(sf::Color::White);

    sf::Text nwplaying = sf::Text();
    nwplaying.setString("Now playing:");
    nwplaying.setFont(_fabryka);
    nwplaying.setCharacterSize(26);
    nwplaying.setFillColor(sf::Color::White);
    nwplaying.setOutlineColor(sf::Color(100, 100, 100, 80));
    nwplaying.setOutlineThickness(0.3);
    nwplaying.setPosition(sf::Vector2f(331.0, 445.0));

    sf::Text presstxt = sf::Text();
    presstxt.setString("Press play to start");
    presstxt.setFont(_fabryka);
    presstxt.setCharacterSize(35);
    presstxt.setFillColor(sf::Color::White);
    presstxt.setOutlineColor(sf::Color(100, 100, 100, 80));
    presstxt.setOutlineThickness(0.5);
    presstxt.setPosition(sf::Vector2f(245.0, 475.0));

    sf::Text player = sf::Text();
    player.setString("PLAYER");
    player.setFont(jb);
    player.setCharacterSize(35);
    player.setFillColor(sf::Color::White);
    player.setOutlineColor(sf::Color(200, 200, 200, 200));
    player.setPosition(sf::Vector2f(340.0, 45.0));

    sf::Text history = sf::Text();
    history.setString("HISTORY");
    history.setFont(jb);
    history.setCharacterSize(35);
    history.setFillColor(sf::Color::White);
    history.setOutlineColor(sf::Color(200, 200, 200, 200));
    history.setPosition(sf::Vector2f(60.0, 45.0));

    sf::Text about_text = sf::Text();
    about_text.setString("ABOUT");
    about_text.setFont(jb);
    about_text.setCharacterSize(35);
    about_text.setFillColor(sf::Color::White);
    about_text.setOutlineColor(sf::Color(200, 200, 200, 200));
    about_text.setPosition(sf::Vector2f(610.0, 45.0));

    sf::ConvexShape display_rect = sf::ConvexShape(4);
    display_rect.setFillColor(sf::Color(255, 255, 255, 110));

    sf::Text about_page = sf::Text();
    about_page.setFont(_fabryka);
    about_page.setCharacterSize(30);
    about_page.setOutlineThickness(2.5);
    about_page.setOutlineColor(sf::Color(45, 45, 45, 200));
    about_page.setString("Github: github.com/afentev/LoFi\n\nDev channel: t.me/lofiradiodesktop\n\nContact: t.me/afentev");
    about_page.setPosition(sf::Vector2f(1100.0, 160.0));

    sf::ConvexShape pause1 = sf::ConvexShape(4);
    pause1.setPoint(0, sf::Vector2f(344.0, 631.0));
    pause1.setPoint(1, sf::Vector2f(352.0, 631.0));
    pause1.setPoint(2, sf::Vector2f(352.0, 608.0));
    pause1.setPoint(3, sf::Vector2f(344.0, 608.0));

    sf::ConvexShape pause2 = sf::ConvexShape(4);
    pause2.setPoint(0, sf::Vector2f(356.0, 631.0));
    pause2.setPoint(1, sf::Vector2f(364.0, 631.0));
    pause2.setPoint(2, sf::Vector2f(364.0, 608.0));
    pause2.setPoint(3, sf::Vector2f(356.0, 608.0));

    pause1.setFillColor(sf::Color::White);

    char whirligig = 'p';

    sf::RenderWindow window(sf::VideoMode(width, height), "LoFi Radio", sf::Style::Titlebar);
    sf::Image im = sf::Image();
    im.loadFromFile(dir + "/icon.jpg");
    window.setIcon(32, 32, im.getPixelsPtr());
//    window.set_vertical_sync_enabled(true);
    window.setKeyRepeatEnabled(false);

    int counter = 1;
    int sleeper = 0;
    int wait = 0;
    int missing = 0;

    sf::ConvexShape rect1 = sf::ConvexShape(4);
    sf::ConvexShape rect2 = sf::ConvexShape(4);
    sf::ConvexShape rect3 = sf::ConvexShape(4);
    sf::ConvexShape rect4 = sf::ConvexShape(4);
    sf::ConvexShape rect5 = sf::ConvexShape(4);

    sf::Text track1 = sf::Text();
    track1.setFont(jb);
    track1.setCharacterSize(23);

    sf::Text date1 = sf::Text();
    date1.setFont(jb);
    date1.setCharacterSize(15);

    sf::Text track2 = sf::Text();
    track2.setFont(jb);
    track2.setCharacterSize(23);

    sf::Text date2 = sf::Text();
    date2.setFont(jb);
    date2.setCharacterSize(15);

    sf::Text track3 = sf::Text();
    track3.setFont(jb);
    track3.setCharacterSize(23);

    sf::Text date3 = sf::Text();
    date3.setFont(jb);
    date3.setCharacterSize(15);

    sf::Text track4 = sf::Text();
    track4.setFont(jb);
    track4.setCharacterSize(23);

    sf::Text date4 = sf::Text();
    date4.setFont(jb);
    date4.setCharacterSize(15);

    sf::Text track5 = sf::Text();
    track5.setFont(jb);
    track5.setCharacterSize(23);

    sf::Text date5 = sf::Text();
    date5.setFont(jb);
    date5.setCharacterSize(15);

    rect1.setPoint(0, sf::Vector2f(-970.0, 170.0));
    rect1.setPoint(1, sf::Vector2f(-970.0, 260.0));
    rect1.setPoint(2, sf::Vector2f(-330.0, 260.0));
    rect1.setPoint(3, sf::Vector2f(-330.0, 170.0));

    rect2.setPoint(0, sf::Vector2f(-970.0, 290.0));
    rect2.setPoint(1, sf::Vector2f(-970.0, 380.0));
    rect2.setPoint(2, sf::Vector2f(-330.0, 380.0));
    rect2.setPoint(3, sf::Vector2f(-330.0, 290.0));

    rect3.setPoint(0, sf::Vector2f(-970.0, 410.0));
    rect3.setPoint(1, sf::Vector2f(-970.0, 500.0));
    rect3.setPoint(2, sf::Vector2f(-330.0, 500.0));
    rect3.setPoint(3, sf::Vector2f(-330.0, 410.0));

    rect4.setPoint(0, sf::Vector2f(-970.0, 530.0));
    rect4.setPoint(1, sf::Vector2f(-970.0, 620.0));
    rect4.setPoint(2, sf::Vector2f(-330.0, 620.0));
    rect4.setPoint(3, sf::Vector2f(-330.0, 530.0));

    rect5.setPoint(0, sf::Vector2f(-970.0, 650.0));
    rect5.setPoint(1, sf::Vector2f(-970.0, 740.0));
    rect5.setPoint(2, sf::Vector2f(-330.0, 740.0));
    rect5.setPoint(3, sf::Vector2f(-330.0, 650.0));

    rect1.setFillColor(sf::Color(10, 10, 10, 100));
    rect1.setOutlineThickness(6.0);
    rect1.setOutlineColor(sf::Color(10, 10, 10, 165));

    rect2.setFillColor(sf::Color(10, 10, 10, 100));
    rect2.setOutlineThickness(6.0);
    rect2.setOutlineColor(sf::Color(10, 10, 10, 165));

    rect3.setFillColor(sf::Color(10, 10, 10, 100));
    rect3.setOutlineThickness(6.0);
    rect3.setOutlineColor(sf::Color(10, 10, 10, 165));

    rect4.setFillColor(sf::Color(10, 10, 10, 100));
    rect4.setOutlineThickness(6.0);
    rect4.setOutlineColor(sf::Color(10, 10, 10, 165));

    rect5.setFillColor(sf::Color(10, 10, 10, 100));
    rect5.setOutlineThickness(6.0);
    rect5.setOutlineColor(sf::Color(10, 10, 10, 165));

    date1.setFillColor(sf::Color(250, 250, 250, 255));
    date2.setFillColor(sf::Color(250, 250, 250, 255));
    date3.setFillColor(sf::Color(250, 250, 250, 255));
    date4.setFillColor(sf::Color(250, 250, 250, 255));
    date5.setFillColor(sf::Color(250, 250, 250, 255));

    track1.setString("No data");
    track2.setString("No data");
    track3.setString("No data");
    track4.setString("No data");
    track5.setString("No data");

    date1.setString("a long time ago");
    date2.setString("a long time ago");
    date3.setString("a long time ago");
    date4.setString("a long time ago");
    date5.setString("a long time ago");

    track1.setPosition(sf::Vector2f(-960.0, 180.0));
    date1.setPosition(sf::Vector2f(-958.0, 230.0));
    track2.setPosition(sf::Vector2f(-960.0, 300.0));
    date2.setPosition(sf::Vector2f(-958.0, 350.0));
    track3.setPosition(sf::Vector2f(-960.0, 420.0));
    date3.setPosition(sf::Vector2f(-958.0, 470.0));
    track4.setPosition(sf::Vector2f(-960.0, 540.0));
    date4.setPosition(sf::Vector2f(-958.0, 590.0));
    track5.setPosition(sf::Vector2f(-960.0, 660.0));
    date5.setPosition(sf::Vector2f(-958.0, 710.0));

    std::vector<unsigned long long> hist_time = std::vector<unsigned long long> {0, 0, 0, 0, 0};

    gen_hist_rects(&date1, &date2, &date3, &date4, &date5, hist_time.begin(), false);

    bool is_music_playing = false;
    bool real_playing;

    bool prev = window.hasFocus();

    while (window.isOpen()) {
            if (window.hasFocus()) {
                window.setFramerateLimit(160);
            } else {
                window.setFramerateLimit(3);
            }
            real_playing = mdp.isPlaying();

            if (whirligig == 'p') {
                if (!is_music_playing) {
                    play_text.setString("PLAY");
                    play_text.setPosition(sf::Vector2f(390.0, 601.0));
                    play_rect.setPoint(0, sf::Vector2f(310.0, 600.0));
                    play_rect.setPoint(1, sf::Vector2f(490.0, 600.0));
                    play_rect.setPoint(2, sf::Vector2f(490.0, 640.0));
                    play_rect.setPoint(3, sf::Vector2f(310.0, 640.0));
                } else if (mdp.isPlaying()) {
                    play_text.setString("PAUSE");
                    play_text.setPosition(sf::Vector2f(383.0, 601.0));
                    play_rect.setPoint(0, sf::Vector2f(310.0, 600.0));
                    play_rect.setPoint(1, sf::Vector2f(490.0, 600.0));
                    play_rect.setPoint(2, sf::Vector2f(490.0, 640.0));
                    play_rect.setPoint(3, sf::Vector2f(310.0, 640.0));
                } else {
                    play_text.setString("Wait, connecting...");
                    play_text.setPosition(sf::Vector2f(232.0, 601.0));
                    play_rect.setPoint(0, sf::Vector2f(210.0, 600.0));
                    play_rect.setPoint(1, sf::Vector2f(590.0, 600.0));
                    play_rect.setPoint(2, sf::Vector2f(590.0, 640.0));
                    play_rect.setPoint(3, sf::Vector2f(210.0, 640.0));
                }
            }

            window.clear(sf::Color::White);

            if (whirligig == 'p') {
                player.setOutlineThickness(1.0);
                player.setOutlineColor(sf::Color::White);
                history.setOutlineThickness(0.0);
                history.setOutlineColor(sf::Color(200, 200, 200, 200));
                about_text.setOutlineThickness(0.0);
                about_text.setOutlineColor(sf::Color(200, 200, 200, 200));

                display_rect.setPoint(0, sf::Vector2f(300.0, 30.0));
                display_rect.setPoint(1, sf::Vector2f(300.0, 100.0));
                display_rect.setPoint(2, sf::Vector2f(505.0, 100.0));
                display_rect.setPoint(3, sf::Vector2f(505.0, 30.0));
            } else if (whirligig == 'h') {
                history.setOutlineThickness(1.0);
                history.setOutlineColor(sf::Color::White);
                player.setOutlineThickness(0.0);
                player.setOutlineColor(sf::Color(200, 200, 200, 200));
                about_text.setOutlineThickness(0.0);
                about_text.setOutlineColor(sf::Color(200, 200, 200, 200));

                display_rect.setPoint(0, sf::Vector2f(15.0, 30.0));
                display_rect.setPoint(1, sf::Vector2f(15.0, 100.0));
                display_rect.setPoint(2, sf::Vector2f(250.0, 100.0));
                display_rect.setPoint(3, sf::Vector2f(250.0, 30.0));
            } else if (whirligig == 'a') {
                about_text.setOutlineThickness(1.0);
                about_text.setOutlineColor(sf::Color::White);
                history.setOutlineThickness(0.0);
                history.setOutlineColor(sf::Color(200, 200, 200, 200));
                player.setOutlineThickness(0.0);
                player.setOutlineColor(sf::Color(200, 200, 200, 200));

                about_page.setPosition(sf::Vector2f(50.0, 160.0));

                display_rect.setPoint(0, sf::Vector2f(565.0, 30.0));
                display_rect.setPoint(1, sf::Vector2f(565.0, 100.0));
                display_rect.setPoint(2, sf::Vector2f(760.0, 100.0));
                display_rect.setPoint(3, sf::Vector2f(760.0, 30.0));
            }

            sf::Vector2i cords = sf::Mouse::getPosition(window);

            if (is_music_playing && real_playing || !is_music_playing && !real_playing && window.hasFocus()) {
                if (310 <= cords.x && cords.x <= 490 && 600 <= cords.y && cords.y <= 640) {
                    play_rect.setFillColor(sf::Color(130, 130, 130, 180));
                } else {
                    play_rect.setFillColor(sf::Color(0, 0, 0, 180));
                }
            }

            std::string name = "/Gif/ss/";
            name += std::to_string(counter) + ".png";
            sf::Texture image = sf::Texture();
            image.loadFromFile(dir + name);

            sf::Sprite sprite = sf::Sprite();
            sprite.setTexture(image);
            sprite.setTextureRect(sf::IntRect(0, 0, width, height));
            sprite.setColor(sf::Color(255, 255, 255, 255));
            sprite.setPosition(sf::Vector2f(0.0, 0.0));

            sf::Event event;
            while (window.pollEvent(event)) {
                switch (event.type) {
                    case sf::Event::Closed:
                        return 0;
                    case sf::Event::KeyPressed:
                        if (event.key.code == sf::Keyboard::Escape) {
                            return 0;
                        }
                    case sf::Event::MouseButtonPressed:
                        sf::Mouse::Button button = event.mouseButton.button;
                        int x = event.mouseButton.x;
                        int y = event.mouseButton.y;
                        if (button == sf::Mouse::Left && window.hasFocus() && prev) {
                            if (729 <= x && x <= 787 && 6 <= y && y <= 32) {
                                return 0;
                            }

                            if (310 <= x && x <= 490 && 600 <= y && y <= 640) {
                                if (is_music_playing && real_playing || !is_music_playing && !real_playing) {
                                    if (is_music_playing) {
                                        mdp.stop();
                                        play_text.setString("PLAY");
                                        play_text.setPosition(sf::Vector2f(390.0, 601.0));
                                        is_music_playing = false;
                                    } else {
                                        mdp.play();
                                        play_text.setPosition(sf::Vector2f(383.0, 601.0));
                                        is_music_playing = true;
                                        play_rect.setFillColor(sf::Color(0, 0, 0, 180));
                                    }
                                }
                                real_playing = mdp.isPlaying();
                            }

                            sf::Vector3f v = sf::Vector3f(300.0, 505.0, 760.0);
                            if (is_clicked_on_right(x, y)) {
                                char prev = whirligig;
                                whirligig = 'a';
                                if (prev == 'p') {
                                    sf::Vector3f v = sf::Vector3f(300.0, 505.0, 760.0);
                                    moving(&window, &display_rect, &player, &history, &about_text,
                                           &close_text, &text_lofi, &text_radio, &play_rect, &play_text,
                                           &play_triangle, &nwplaying, &presstxt, &about_page, -35.0, v,
                                           27.0, true, &rect1, &rect2, &rect3, &rect4, &rect5,
                                           &track1, &date1, &track2, &date2, &track3, &date3, &track4, &date4,
                                           &track5, &date5, &pause1, &pause2, is_music_playing, real_playing,
                                           &sleeper, &counter, &wait, dir);
                                } else if (prev == 'h') {
                                    moving(&window, &display_rect, &player, &history, &about_text,
                                           &close_text, &text_lofi, &text_radio, &play_rect, &play_text,
                                           &play_triangle, &nwplaying, &presstxt, &about_page, -70.0, v,
                                           0.0, false, &rect1, &rect2, &rect3, &rect4, &rect5,
                                           &track1, &date1, &track2, &date2, &track3, &date3, &track4, &date4,
                                           &track5, &date5, &pause1, &pause2, is_music_playing, real_playing,
                                           &sleeper, &counter, &wait, dir);
                                }
                            }
                            if (is_clicked_on_left(x, y)) {
                                char prev = whirligig;
                                whirligig = 'h';
                                if (prev == 'p') {
                                    sf::Vector3f v = sf::Vector3f(505.0, 300.0, 15.0);
                                    moving(&window, &display_rect, &player, &history, &about_text,
                                           &close_text, &text_lofi, &text_radio, &play_rect, &play_text,
                                           &play_triangle, &nwplaying, &presstxt, &about_page, 35.0, v,
                                           11.0, true, &rect1, &rect2, &rect3, &rect4, &rect5,
                                           &track1, &date1, &track2, &date2, &track3, &date3, &track4, &date4,
                                           &track5, &date5, &pause1, &pause2, is_music_playing, real_playing,
                                           &sleeper, &counter, &wait, dir);
                                        } else if (prev == 'a') {
                                            moving(&window, &display_rect, &player, &history, &about_text,
                                                   &close_text, &text_lofi, &text_radio, &play_rect, &play_text,
                                                   &play_triangle, &nwplaying, &presstxt, &about_page, 70.0, v,
                                                   0.0, false, &rect1, &rect2, &rect3, &rect4, &rect5,
                                                   &track1, &date1, &track2, &date2, &track3, &date3, &track4, &date4,
                                                   &track5, &date5, &pause1, &pause2, is_music_playing, real_playing,
                                                   &sleeper, &counter, &wait, dir);
                                        }
                                    }
                                    if (is_clicked_on_center(x, y)) {
                                        char prev = whirligig;
                                        whirligig = 'p';
                                        if (prev == 'h') {
                                            sf::Vector3f v = sf::Vector3f(15.0, 250.0, 505.0);
                                            moving(&window, &display_rect, &player, &history, &about_text,
                                                   &close_text, &text_lofi, &text_radio, &play_rect, &play_text,
                                                   &play_triangle, &nwplaying, &presstxt, &about_page, -35.0, v,
                                                   47.0, true, &rect1, &rect2, &rect3, &rect4, &rect5,
                                                   &track1, &date1, &track2, &date2, &track3, &date3, &track4, &date4,
                                                   &track5, &date5, &pause1, &pause2, is_music_playing, real_playing,
                                                   &sleeper, &counter, &wait, dir);
                                        } else if (prev == 'a') {
                                            sf::Vector3f v = sf::Vector3f(760.0, 505.0, 300.0);
                                            moving(&window, &display_rect, &player, &history, &about_text,
                                                    &close_text, &text_lofi, &text_radio, &play_rect, &play_text,
                                                    &play_triangle, &nwplaying, &presstxt, &about_page, 35.0, v,
                                                    0.0, true, &rect1, &rect2, &rect3, &rect4, &rect5,
                                                    &track1, &date1, &track2, &date2, &track3, &date3, &track4, &date4,
                                                    &track5, &date5, &pause1, &pause2, is_music_playing, real_playing,
                                                    &sleeper, &counter, &wait, dir);
                                        }
                                    }
                            }
                }
            }

            window.draw(sprite);
            window.draw(display_rect);

            if (whirligig == 'p') {
                window.draw(text_lofi);
                window.draw(text_radio);
                window.draw(play_rect);
                window.draw(play_text);
                window.draw(nwplaying);
                window.draw(presstxt);
                if (is_music_playing && real_playing) {
                    window.draw(pause1);
                    window.draw(pause2);
                } else if (!is_music_playing && !real_playing) {
                    window.draw(play_triangle);
                }
            } else if (whirligig == 'a') {
                window.draw(about_page);
            } else if (whirligig == 'h') {
                window.draw(rect1);
                window.draw(rect2);
                window.draw(rect3);
                window.draw(rect4);
                window.draw(rect5);
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
            }

            if (window.hasFocus() && prev) {
                if (0 <= cords.x && cords.x <= 800 && 0 <= cords.y && cords.y <= 800) {
                    if ((800 - cords.x) * (800 - cords.x) + cords.y * cords.y <= 125000) {
                        double delta = ((800 - cords.x) * (800 - cords.x) + (cords.y) * (cords.y));
                        delta = (1.0 - delta / 125000.0) * 255.0;
                        close_text.setFillColor(sf::Color(255, 255, 255, delta));
                    } else {
                        close_text.setFillColor(sf::Color(255, 255, 255, 0));
                        close_text.setOutlineThickness(0.0);
                    }
                    if (729 <= cords.x && cords.x <= 787 && 6 <= cords.y && cords.y <= 32) {
                        close_text.setOutlineThickness(0.9);
                    } else {
                        close_text.setOutlineThickness(0.0);
                    }

                    if (560 <= cords.x && cords.x <= 770 && 40 <= cords.y && cords.y <= 92) {
                        if (whirligig != 'a') {
                            about_text.setOutlineThickness(1.1);
                        }
                    }

                    if (45 <= cords.x && cords.x <= 225 && 40 <= cords.y && cords.y <= 92) {
                        if (whirligig != 'h') {
                            history.setOutlineThickness(1.1);
                        }
                    }
                    if (310 <= cords.x && cords.x <= 495 && 40 <= cords.y && cords.y <= 92) {
                        if (whirligig != 'p') {
                            player.setOutlineThickness(1.1);
                        }
                    }

                } else {
                    close_text.setFillColor(sf::Color(255, 255, 255, 0));
                    close_text.setOutlineThickness(0.0);
                }
            }

            window.draw(player);
            window.draw(history);
            window.draw(about_text);
            window.draw(close_text);

            window.display();

            if (sleeper == 0) {
                missing = (1 + missing) % 15;
                if (missing == 1) {
                    if (real_playing) {
                        std::stringstream os;
                        os << curlpp::options::Url(url);

                        std::string xml = os.str();
                        int lhs = xml.find("subtext=") + 9;
                        int rhs = xml.substr(lhs).find('"');
                        std::string title = xml.substr(lhs, rhs);
                        if (title.length() > 40) {
                            title = title.substr(0, 40) + "...";
                        }
                        presstxt.setString(title);

                        if (whirligig == 'p') {
                            int delta = title.length() - 19.0;
                            presstxt.setPosition(sf::Vector2f(245 - 7.8 * delta, 475.0));
                        }
                        if (title != track1.getString()) {
                            track5.setString(track4.getString());
                            track4.setString(track3.getString());
                            track3.setString(track2.getString());
                            track2.setString(track1.getString());
                            track1.setString(title);

                            std::time_t start = std::time(nullptr);

                            hist_time.push_back(start);
                            hist_time.erase(hist_time.begin());
                        }
                    } else {
                        presstxt.setString("Press play to start");
                        if (whirligig == 'p') {
                            presstxt.setPosition(sf::Vector2f(245.0, 475.0));
                        }
                    }
                    //`Unknown - ｃｈｅｒｒｙ ｘ Ａｐｏ`
                    gen_hist_rects(&date1, &date2, &date3, &date4, &date5, hist_time.begin(), real_playing);
                }
                if (counter == 1) {
                    if (wait == 0) {
                        counter = 1 + (counter + 1) % 260;
                        wait = 1;
                    } else {
                        wait = (wait + 1) % 75;
                    }
                } else {
                    counter = 1 + (counter + 1) % 260;
                }
            }

            if (window.hasFocus()) {
                sleeper = (sleeper + 1) % 15;
            } else {
                sleeper = (sleeper + 1) % 1;
            }

            prev = window.hasFocus();

//        sleep(Duration::from_nanos(1000000000));
    }
    return 0;
}
