mod state;

use adw::ApplicationWindow as AdwApplicationWindow;
use gdk::Display;
use gtk::prelude::*;
use gtk::{Application, Builder, CssProvider};
use state::AppState;

const APP_ID: &str = "kz.findmyname284.anixartd";
const APP_PATH: &str = "/kz/findmyname284/anixartd";

fn main() {
    // Register compiled resources at runtime
    if let Err(e) = gio::resources_register_include!("app.gresource") {
        eprintln!("Failed to register resources: {}", e);
        std::process::exit(1);
    }

    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(|app| {
        load_css();

        let state = AppState::load();

        if state.first_run {
            show_login_window(app, state);
        } else {
            show_main_window(app, state.token.is_some());
        }
    });
    application.run();
}

fn show_login_window(app: &Application, state: AppState) {
    let builder = Builder::from_resource(&format!("{}/ui/login_window.ui", APP_PATH));

    let window: AdwApplicationWindow = builder
        .object("login_window")
        .expect("Couldn't get login_window");

    let logo = builder
        .object::<gtk::Image>("logo")
        .expect("Couldn't get logo image");

    let theme = "dark";

    if theme.to_lowercase() == "light" {
        logo.set_resource(Some(&format!("{}/img/logo_dark.png", APP_PATH)));
    } else {
        logo.set_resource(Some(&format!("{}/img/logo_light.png", APP_PATH)));
    }

    let _login_button: gtk::Button = builder
        .object("login_button")
        .expect("Login button not found");
    let _register_button: gtk::Button = builder
        .object("register_button")
        .expect("Register button not found");
    let _forgot_button: gtk::Button = builder
        .object("forgot_button")
        .expect("Forgot button not found");
    let skip_button: gtk::Button = builder
        .object("skip_button")
        .expect("Skip button not found");

    let app_clone = app.clone();
    let window_clone = window.clone();
    skip_button.connect_clicked(move |_| {
        // state.token = None;
        // state.first_run = false;
        state.save();
        
        window_clone.close();
        show_main_window(&app_clone, false);
    });

    window.set_application(Some(app));
    window.present();
}

fn show_main_window(app: &Application, is_authenticated: bool) {
    let builder = Builder::from_resource(&format!("{}/ui/main_window.ui", APP_PATH));
    let window: AdwApplicationWindow = builder.object("main_window").expect("Main window not found");

    if !is_authenticated {
        if let Some(premium_section) = builder.object::<gtk::Box>("premium_section") {
            premium_section.set_visible(false);
        }
    }

    window.set_application(Some(app));
    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../resources/style.css"));

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
