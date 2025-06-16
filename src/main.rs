mod anicard;
mod api_client;
mod utils;

use gtk::Application;
use gtk::prelude::*;
use std::{cell::RefCell, rc::Rc};
use utils::{state::AppState, ui::*};

#[tokio::main]
async fn main() {
    // Тестовый запрос (оставлен для демонстрации)
    gtk::glib::spawn_future_local(async move {
        let client = api_client::AnixartClient::new();
        if let Ok(ip) = client.get_ip().await {
            println!(
                "IP address: {}",
                ip.get("ip")
                    .unwrap_or(&serde_json::Value::String("Unknown".to_string()))
            );
        }
    });

    // Регистрация ресурсов
    if gio::resources_register_include!("app.gresource").is_err() {
        eprintln!("Failed to register resources");
        std::process::exit(1);
    }

    // Создание приложения
    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(|app| {
        load_css();
        let state = Rc::new(RefCell::new(AppState::load()));

        if state.borrow().first_run {
            show_login_window(app, state.clone());
        } else {
            show_main_window(app, state);
        }
    });
    application.run();
}

fn show_login_window(app: &Application, state: Rc<RefCell<AppState>>) {
    let templates = UiTemplates::load();
    let ui = LoginUi::new(&templates);
    ui.window.set_application(Some(app));

    let state_clone = state.clone();
    let window_clone = ui.window.clone();
    let app_clone = app.clone();
    let ui_clone = ui.clone();

    ui.login_button.connect_clicked(move |_| {
        let state = state_clone.clone();
        let username = ui_clone.username_entry.text().to_string();
        let password = ui_clone.password_entry.text().to_string();
        let window_clone = window_clone.clone();
        let app_clone = app_clone.clone();

        gtk::glib::spawn_future_local(async move {
            let client = api_client::AnixartClient::new();
            match client.sign_in(&username, &password).await {
                Ok(auth_data) => {
                    let token = auth_data["profileToken"]["token"]
                        .as_str()
                        .unwrap_or("")
                        .to_string();

                    gtk::glib::idle_add_local_once(move || {
                        state.borrow_mut().update_token(token);
                        window_clone.close();
                        show_main_window(&app_clone, state);
                    });
                }
                Err(e) => eprintln!("Login failed: {}", e),
            }
        });
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

fn show_main_window(app: &Application, state: Rc<RefCell<AppState>>) {
    let templates = UiTemplates::load();
    let ui = AppUi::new(app, &templates);

    if state.borrow().token.is_none() {
        println!("User is not authenticated");
    }

    let cards_container = ui.cards_container.clone();
    let client = api_client::AnixartClient::new();

    gtk::glib::spawn_future_local(async move {
        if let Ok(json) = client
            .apply_filter("c9b442779655b81f2004c96f69d9943e065e338c")
            .await
        {
            if let Some(content) = json["content"].as_array() {
                for item in content {
                    if let (Some(title), Some(desc)) =
                        (item["title_ru"].as_str(), item["description"].as_str())
                    {
                        let card = anicard::AnimeCard::new(title, "", "", desc);
                        cards_container.append(&card);
                    }
                }
            }
        }
    });

    // Обработчик переключения вкладок
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
