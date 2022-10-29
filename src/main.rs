// Copyright 2022 David Terhune
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use fltk::{
    app,
    dialog,
    enums::Shortcut,
    menu,
    prelude::*,
    window::Window,
};

mod campaign;

// Width and height of main window
const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// Menu item message types.
#[derive(Clone)]
enum Message {
    Quit,
    NewCampaign,
    HelpAbout,
}

// Center of screen
fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

fn new_campaign() {
    let name = dialog::input_default("New Campaign Name", "");
    if let Some(_name) = name {
        // TODO Create new campaign
    }
}

fn show_about() {
    let loc = center();
    let mut help = dialog::HelpDialog::new(
        loc.0 - 250, loc.1 - 150, 500, 300);
    help.set_value("
        <head>
        <title>About VBAM Campaign Moderator's Assistant</title>
        </head>
        <body>
        <center>
        VBAM Campaign Moderator's Assistant is Copyright 2022 David Terhune
        <p>
        Released under the Apache License, Version 2.0
        <p>
        Victory By Any Means is a trademark of Victory By Any Means Games, Incorporated.
        Used with permission.
        </center>
        </body>
    ");
    help.show();
    while help.shown() {
        app::wait();
    }
}

fn main() {
    let app = app::App::default();
    let (s, r) = app::channel();
    
    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .center_screen()
        .with_label("VBAM Campain Moderator's Assistant");

    let mut menu = menu::MenuBar::default().with_size(WIDTH, 25);

    menu.add_emit("&File/&Quit\t", Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal, s.clone(), Message::Quit);
    
    menu.add_emit("&Campaign/&New...\t", Shortcut::Ctrl | 'n',
        menu::MenuFlag::Normal, s.clone(), Message::NewCampaign);

    menu.add_emit("&Help/&About...\t", Shortcut::None,
        menu::MenuFlag::Normal, s.clone(), Message::HelpAbout);

    wind.end();
    wind.show();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => app::quit(),
                Message::NewCampaign => new_campaign(),
                Message::HelpAbout => show_about(),
            }
        }
    }
}
