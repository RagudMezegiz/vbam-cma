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
    button,
    dialog,
    enums::{Align, Shortcut},
    frame,
    group,
    input,
    menu,
    prelude::*,
    window,
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

struct VBAMApp {
    app: app::App,
    rcvr: app::Receiver<Message>,
    cmpgn: Option<campaign::Campaign>
}

impl VBAMApp {
    fn new() -> Self {
        let app = app::App::default();
        let (s, rcvr) = app::channel();
    
        let mut main_win = window::Window::default()
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

        main_win.end();
        main_win.show();

        Self {
            app,
            rcvr,
            cmpgn: Option::None
        }
    }

    async fn run(&mut self) {
        while self.app.wait() {
            if let Some(msg) = self.rcvr.recv() {
                match msg {
                    Message::Quit => {
                        if let Some(c) = &self.cmpgn {
                            c.close().await
                        }
                        app::quit()
                    },
                    Message::NewCampaign => self.new_campaign().await,
                    Message::HelpAbout => show_about(),
                }
            }
        }
    }

    async fn new_campaign(&mut self) {
        let mut wind = window::Window::default()
            .with_size(300, 300)
            .center_screen()
            .with_label("Create New Campaign");

        let mut vbox = group::Pack::default()
            .with_size(300, 300)
            .with_type(group::PackType::Vertical);
        vbox.set_spacing(20);
        frame::Frame::default()
            .with_label("New Campaign Name");
        let name_input = input::Input::default();

        // TODO Add Campaign options controls

        let mut bbox = group::Pack::default()
            .with_align(Align::BottomRight)
            .with_size(300, 0)
            .with_type(group::PackType::Horizontal);
        bbox.set_spacing(20);
        let mut ok = button::Button::default()
            .with_label("Ok");
        let mut cancel = button::Button::default()
            .with_label("Cancel");
        bbox.end();
        bbox.auto_layout();

        vbox.end();
        vbox.auto_layout();

        wind.end();
        wind.make_modal(true);
        wind.show();

        let (s, r) = app::channel();
        ok.emit(s.clone(), true);
        cancel.emit(s.clone(), false);
        
        let mut is_ok = false;
        while wind.shown() && self.app.wait() {
            if let Some(a) = r.recv() {
                is_ok = match a {
                    true => true,
                    false => false,
                };
                wind.hide();
            }
        }

        if is_ok && !name_input.value().is_empty() {
            let c = campaign::Campaign::new(
                name_input.value()).await;
            self.cmpgn = match c {
                Ok(cm) => {
                    println!("Created {} campaign", name_input.value());
                    Some(cm)
                },
                Err(s) => {
                    dialog::alert_default(s.as_str());
                    None
                },
            };
        }
    }
}

// Center of screen
fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
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

#[tokio::main]
async fn main() {
    VBAMApp::new().run().await;
}
