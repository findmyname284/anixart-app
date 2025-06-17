mod anicard;
mod api_client;
mod utils;

use gtk::Application;
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};
use utils::{config::Config, ui::*};

async fn my_task() {
    println!("Requesting IP address...");
    let client = api_client::AnixartClient::global();
    if let Ok(ip) = client.get_ip().await {
        println!("IP address: {:#?}", ip);
    } else {
        println!("Error");
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(my_task());

    if gio::resources_register_include!("app.gresource").is_err() {
        eprintln!("Failed to register resources");
        std::process::exit(1);
    }

    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(|app| {
        load_css();
        let config = Rc::new(RefCell::new(Config::load()));

        if config.borrow().first_run {
            show_login_window(app, config.clone());
            config.borrow_mut().save();
        } else {
            show_main_window(app, config);
        }
    });
    application.run();
}

fn show_login_window(app: &Application, state: Rc<RefCell<Config>>) {
    let templates = UiTemplates::load();
    let ui = LoginUi::new(&templates);
    ui.window.set_application(Some(app));

    let state_clone = state.clone();
    let window_clone = ui.window.clone();
    let app_clone = app.clone();

    let restore_sensitivity = {
        let login_button = ui.login_button.clone();
        let skip_button = ui.skip_button.clone();
        let username_entry = ui.username_entry.clone();
        let password_entry = ui.password_entry.clone();
        move || {
            login_button.set_sensitive(true);
            skip_button.set_sensitive(true);
            username_entry.set_sensitive(true);
            password_entry.set_sensitive(true);
        }
    };

    ui.login_button.connect_clicked({
        let username_entry = ui.username_entry.clone();
        let password_entry = ui.password_entry.clone();
        let login_button = ui.login_button.clone();
        let skip_button = ui.skip_button.clone();

        move |_| {
            let state = state_clone.clone();
            let username = username_entry.text().to_string();
            let password = password_entry.text().to_string();
            let window_clone = window_clone.clone();
            let app_clone = app_clone.clone();

            login_button.set_sensitive(false);
            skip_button.set_sensitive(false);
            username_entry.set_sensitive(false);
            password_entry.set_sensitive(false);

            let client = api_client::AnixartClient::global();

            glib::spawn_future_local({
                let future_restore = restore_sensitivity.clone();
                async move {
                    match client.sign_in(&username, &password).await {
                        Ok(auth_data) => {
                            let token_opt = auth_data["profileToken"]["token"]
                                .as_str()
                                .filter(|s| !s.is_empty())
                                .map(|s| s.to_string());
                            if let Some(token) = token_opt {
                                glib::idle_add_local_once(move || {
                                    state.borrow_mut().update_token(token);
                                    window_clone.close();
                                    show_main_window(&app_clone, state);
                                });
                            } else {
                                eprintln!("Login failed: Empty token");
                                glib::idle_add_local_once(move || future_restore());
                            }
                        }
                        Err(e) => {
                            eprintln!("Login failed: {}", e);
                            glib::idle_add_local_once(move || future_restore());
                        }
                    }
                }
            });
        }
    });

    let app_clone = app.clone();
    let ui_clone = ui.clone();
    ui.skip_button.connect_clicked(move |_| {
        state.borrow_mut().skip_login();
        ui_clone.window.close();
        show_main_window(&app_clone, state.clone());
    });

    ui.window.present();
}

fn show_main_window(app: &Application, state: Rc<RefCell<Config>>) {
    let templates = UiTemplates::load();
    let ui = AppUi::new(app, &templates);

    if state.borrow().token.is_none() {
        println!("User is not authenticated");
    }

    let cards_container = ui.cards_container.clone();
    let client = api_client::AnixartClient::global();

    // let (sender, receiver) = async_channel::unbounded::<String>();

    // for _ in 0..12 {
    //     let card = anicard::AnimeCard::new("https://anixstatic.com/posters/9AJigmcT0KD9R7lVKohzzTk15o7FR0.jpg", "title", "description");
    //     cards_container.append(&card);
    //     // let _ = image_download::download_image_async("https://anixstatic.com/posters/qIkzXrQ9N3pQRupQJlDxSJHoquupWX.jpg").await;
    // }

    gtk::glib::spawn_future_local(async move {
        match client
            .apply_filter(state.borrow().token.as_ref().unwrap())
            .await
        {
            Ok(json) => {
                if let Some(content) = json["content"].as_array() {
                    for item in content {
                        if let (Some(title), Some(desc), Some(image)) = (
                            item["title_ru"].as_str(),
                            item["description"].as_str(),
                            item["image"].as_str(),
                        ) {
                            let cards_container_clone = cards_container.clone();
                            let card = anicard::AnimeCard::new(image, title, desc);
                            cards_container_clone.append(&card);
                            glib::idle_add_local(move || glib::ControlFlow::Break);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    });

    ui.view_stack
        .connect_notify(Some("visible-child"), |stack, _| {
            if let Some(page) = stack.visible_child() {
                match stack.page(&page).name().unwrap().as_str() {
                    "home" => println!("Home tab activated"),
                    "review" => println!("Review tab activated"),
                    _ => (),
                }
            }
        });

    ui.window.present();
}
