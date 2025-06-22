use std::{cell::RefCell, rc::Rc};

use gtk::{prelude::*, Application};

use crate::{api_client, show_main_window, utils::{config::Config, ui::{LoginUi, UiTemplates}}};

pub fn show_login_window(app: &Application, state: Rc<RefCell<Config>>) {
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