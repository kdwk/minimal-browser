#[rustfmt::skip]
mod config;
mod setup;

use gtk::prelude::ApplicationExt;
use relm4::adw::prelude::*;
use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    gtk, main_application, ApplicationBuilderExt, RelmApp,
};
use setup::setup;
use webkit6::prelude::*;

relm4::new_action_group!(AppActionGroup, "app");
relm4::new_stateless_action!(QuitAction, AppActionGroup, "quit");

struct BrowserWindow {
    url: String,
}

#[derive(Debug)]
enum BrowserWindowInput {
    NewBrowserWindow,
}

#[relm4::component]
impl SimpleComponent for BrowserWindow {
    type Init = String;
    type Input = BrowserWindowInput;
    type Output = ();

    view! {
        adw::Window {
            set_default_width: 500,
            set_default_height: 500,
            set_title: Some("Minimal Browser"),
            adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    set_decoration_layout: Some(":close"),
                    add_css_class: "flat",
                    pack_start = &gtk::Button {
                        set_icon_name: "plus",
                        connect_clicked => BrowserWindowInput::NewBrowserWindow,
                    },
                    #[wrap(Some)]
                    set_title_widget = &gtk::Label {
                        set_label: model.url.as_str(),
                        set_ellipsize: gtk::pango::EllipsizeMode::End,
                    },
                },
                #[wrap(Some)]
                set_content = &webkit6::WebView {
                    set_vexpand: true,
                    load_uri = model.url.as_str(),
                }
            },
            present: ()
        }
    }
    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BrowserWindow { url: init };
        let widgets = view_output!();
        ComponentParts {
            model: model,
            widgets: widgets,
        }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            BrowserWindowInput::NewBrowserWindow => {
                BrowserWindow::builder()
                    .launch(String::from("https://duckduckgo.com"))
                    .detach();
            }
        }
    }
}

fn main() {
    // Enable logging
    tracing_subscriber::fmt()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    setup();

    let app = adw::Application::builder()
        .application_id("com.github.kdwk.MinimalBrowser")
        .build();
    let initial_browserwindow =
        BrowserWindow::builder().launch(String::from("https://duckduckgo.com"));

    // gtk::Application::builder()
    // .application_id("org.relm4.SettingsListExample")
    // .launch(|_app, window| {
    //     // Initialize a component's root widget
    //     let mut component = App::builder()
    //         // Attach the root widget to the given window.
    //         .attach_to(&window)
    //         // Start the component service with an initial parameter
    //         .launch("Settings List Demo".into())
    //         // Attach the returned receiver's messages to this closure.
    //         .connect_receiver(move |sender, message| match message {
    //             Output::Clicked(id) => {
    //                 eprintln!("ID {id} Clicked");

    //                 match id {
    //                     0 => xdg_open("https://github.com/Relm4/Relm4".into()),
    //                     1 => xdg_open("https://docs.rs/relm4/".into()),
    //                     2 => {
    //                         sender.send(Input::Clear).unwrap();
    //                     }
    //                     _ => (),
    //                 }
    //             }

    //             Output::Reload => {
    //                 sender
    //                     .send(Input::AddSetting {
    //                         description: "Browse GitHub Repository".into(),
    //                         button: "GitHub".into(),
    //                         id: 0,
    //                     })
    //                     .unwrap();

    //                 sender
    //                     .send(Input::AddSetting {
    //                         description: "Browse Documentation".into(),
    //                         button: "Docs".into(),
    //                         id: 1,
    //                     })
    //                     .unwrap();

    //                 sender
    //                     .send(Input::AddSetting {
    //                         description: "Clear List".into(),
    //                         button: "Clear".into(),
    //                         id: 2,
    //                     })
    //                     .unwrap();
    //             }
    //         });

    //     // Keep runtime alive after the component is dropped
    //     component.detach_runtime();

    //     println!("parent is {:?}", component.widget().toplevel_window());
    // });

    gtk::Application::builder()
        .application_id("com.github.kdwk.MinimalBrowser")
        .launch(|_app, window| {
            let mut component = BrowserWindow::builder()
                .attach_to(&window)
                .launch(String::from("https://duckduckgo.com"));
            component.detach_runtime();
        })

    // let app = main_application();
    // app.set_resource_base_path(Some("/com/github/kdwk/MinimalBrowser/"));

    // let mut actions = RelmActionGroup::<AppActionGroup>::new();

    // let quit_action = {
    //     let app = app.clone();
    //     RelmAction::<QuitAction>::new_stateless(move |_| {
    //         app.quit();
    //     })
    // };
    // actions.add_action(quit_action);
    // actions.register_for_main_application();

    // app.set_accelerators_for_action::<QuitAction>(&["<Control>q"]);

    // let app = RelmApp::from_app(app);

    // app.run::<App>(());
}
