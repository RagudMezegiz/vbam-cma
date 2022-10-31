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
const MAIN_WIDTH: i32 = 800;
const MAIN_HEIGHT: i32 = 600;

// Main window base title
const MAIN_TITLE: &str = "VBAM Campaign Moderator's Assistant";

// Spacing for all the pack groups.
const PACK_SPACING: i32 = 20;

// Menu item message types.
#[derive(Clone)]
enum Message {
    Quit,
    NewCampaign,
    OpenCampaign,
    DeleteCampaign,
    HelpAbout,
}

struct VBAMApp {
    app: app::App,
    main_win: window::Window,
    rcvr: app::Receiver<Message>,
    cmpgn: Option<campaign::Campaign>
}

impl VBAMApp {
    fn new() -> Self {
        let app = app::App::default();
        let (s, rcvr) = app::channel();
    
        let mut main_win = window::Window::default()
            .with_size(MAIN_WIDTH, MAIN_HEIGHT)
            .center_screen()
            .with_label(MAIN_TITLE);

        let mut menu = menu::MenuBar::default().with_size(MAIN_WIDTH, 25);

        menu.add_emit("&File/&Quit\t", Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal, s.clone(), Message::Quit);
        
        menu.add_emit("&Campaign/&New...\t", Shortcut::Ctrl | 'n',
            menu::MenuFlag::Normal, s.clone(), Message::NewCampaign);
        
        menu.add_emit("&Campaign/&Open...\t", Shortcut::Ctrl | 'o',
            menu::MenuFlag::MenuDivider, s.clone(), Message::OpenCampaign);

        menu.add_emit("&Campaign/&Delete...\t", Shortcut::Ctrl | 'd',
            menu::MenuFlag::Normal, s.clone(), Message::DeleteCampaign);

        menu.add_emit("&Help/&About...\t", Shortcut::None,
            menu::MenuFlag::Normal, s, Message::HelpAbout);

        main_win.end();
        main_win.show();

        Self {
            app,
            main_win,
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
                    Message::OpenCampaign => self.open_campaign().await,
                    Message::DeleteCampaign => self.delete_campaign().await,
                    Message::HelpAbout => show_about(),
                }
            }
        }
    }

    // Pop up new campaign dialog and set parameters.
    async fn new_campaign(&mut self) {
        if let Some(cm) = &self.cmpgn {
            cm.close().await;
            self.cmpgn = None;
        }

        let mut wind = window::Window::default()
            .with_size(300, 300)
            .center_screen()
            .with_label("Create New Campaign");

        let mut vbox = group::Pack::default()
            .with_size(300, 300)
            .with_type(group::PackType::Vertical);
        vbox.set_spacing(PACK_SPACING);
        frame::Frame::default()
            .with_label("New Campaign Name");
        let name_input = input::Input::default();

        // TODO Add Campaign options controls

        let mut bbox = group::Pack::default()
            .with_align(Align::BottomRight)
            .with_size(300, 0)
            .with_type(group::PackType::Horizontal);
        bbox.set_spacing(PACK_SPACING);
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
        ok.emit(s, true);
        cancel.emit(s, false);
        
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
                    println!("Created {} campaign", cm.name());
                    Some(cm)
                },
                Err(s) => {
                    dialog::alert_default(s.as_str());
                    None
                },
            };
            self.set_title();
        }
    }

    // Pop up list of campaigns to select from.
    async fn open_campaign(&mut self) {
        if let Some(name) = self.list_campaigns("Open".to_string()) {
            if let Some(cm) = &self.cmpgn {
                cm.close().await;
            }
            let c = campaign::Campaign::open(&name).await;
            self.cmpgn = match c {
                Ok(cm) => {
                    println!("Opened {} campaign", name);
                    Some(cm)
                },
                Err(s) => {
                    dialog::alert_default(s.as_str());
                    None
                },
            };
        }
        self.set_title();
    }

    // Pop up list of campaigns to select one to delete.
    async fn delete_campaign(&mut self) {
        if let Some(name) = self.list_campaigns("Delete".to_string()) {
            match &self.cmpgn {
                Some(cm) => {
                    cm.close().await;
                    self.cmpgn = None;
                    self.set_title();
                },
                None => ()
            }
            match campaign::Campaign::delete(&name) {
                Ok(_) => println!("Deleted {} campaign", name),
                Err(s) => dialog::alert_default(s.as_str()),
            }
        }
    }

    // Pop up the select campaign dialog and return the user's choice.
    fn list_campaigns(&mut self, function: String) -> Option<String> {
        let names = match campaign::list() {
            Ok(v) => v.join("|"),
            _ => return None
        };

        let mut wind = window::Window::default()
            .with_size(150, 150)
            .with_label(format!("{} Campaign", function).as_str())
            .center_screen();
        let mut vbox = group::Pack::default()
            .with_size(150, 150)
            .with_type(group::PackType::Vertical);
        vbox.set_spacing(PACK_SPACING);
        let mut choice = menu::Choice::default();
        choice.add_choice(names.as_str());
        let mut bbox = group::Pack::default()
            .with_size(150, 0)
            .with_type(group::PackType::Horizontal);
        bbox.set_spacing(PACK_SPACING);
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
        ok.emit(s, true);
        cancel.emit(s, false);
        
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

        if is_ok {
            choice.choice()
        } else {
            None
        }
    }

    // Set the main window title. Includes campaign name if one is active.
    fn set_title(&mut self) {
        let title = match &self.cmpgn {
            Some(cm) => format!("{} ({} campaign)", MAIN_TITLE, cm.name()),
            None => MAIN_TITLE.to_string(),
        };
        self.main_win.set_label(title.as_str());
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
