mod anicard;
mod api_client;
mod utils;
mod windows;

use crate::utils::{config::Config, filter, ui::*};
use crate::windows::login_window::show_login_window;
use gtk::Application;
use gtk::prelude::*;
use std::collections::HashSet;
use std::{cell::RefCell, rc::Rc};

// use crate::utils::image::clear_image_cache;

async fn my_task() {
    println!("Requesting IP address...");
    let client = api_client::AnixartClient::global();
    if let Ok(ip) = client.get_ip().await {
        println!("IP address: {:#?}", ip);
    } else {
        println!("Error fetching IP address");
    }

    // match client.episode_target(18909, 30, 3).await {
    //     Ok(target) => {
    //         println!("Episode target: {:#?}", target);
    //     }
    //     Err(e) => {
    //         println!("Error fetching episode target: {}", e.to_string());
    //     }
    // }

    // let link = "https://kodik.info/seria/1447065/00e6639fa57b1f86b8c3bad55de978ed/720p?d=2025062113&s=18a85af08d061f0bc14b2d12cc6e25ce638ca8f09c42ed7b7a3be10f2898f85f&ip=162.158.163.73";
    // match client.kodik_video_links(link).await {
    //     Ok(video) => {
    //         println!("Video link: {:#?}", video);
    //     }
    //     Err(e) => {
    //         println!("Error fetching video link: {}", e.to_string());
    //     }
    // }

    // match client.filter(0).await {
    //     Ok(filter) => {
    //         println!("Filter result: {:#?}", filter);
    //     }
    //     Err(e) => {
    //         println!("Error fetching filter: {}", e.to_string());
    //     }
    // }
}

#[tokio::main]
async fn main() {
    let task = tokio::task::spawn(my_task());
    task.await.unwrap();
    // clear_image_cache();

    if gio::resources_register_include!("app.gresource").is_err() {
        eprintln!("Failed to register resources");
        std::process::exit(1);
    }

    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(|app| {
        let config = Rc::new(RefCell::new(Config::load()));
        let config_borrowed = config.borrow().clone();
        set_dark_theme(config_borrowed.ui.is_dark_theme);
        if is_renderer_cairo() {
            disable_animation(true);
        }
        load_css();

        if config_borrowed.first_run {
            show_login_window(app, config.clone());
            config.borrow_mut().save();
        } else {
            show_main_window(app, config);
        }
    });
    application.run();
}

fn show_main_window(app: &Application, state: Rc<RefCell<Config>>) {
    let templates = UiTemplates::load();
    let ui = AppUi::new(app, &templates);

    if state.borrow().auth.token.is_none() {
        println!("User is not authenticated");
    }

    let active_tab = 1;

    let loaded_tabs: Rc<RefCell<HashSet<u32>>> = Rc::new(RefCell::new(HashSet::new()));
    loaded_tabs.borrow_mut().insert(active_tab);

    let tabs = ui.home_tabs.clone();
    let cards_containers = ui.home_cards_containers.clone();
    tabs.connect_switch_page({
        let loaded_tabs = loaded_tabs.clone();
        let cards_containers = cards_containers.clone();
        move |_, _, index| {
            let mut loaded = loaded_tabs.borrow_mut();
            if !loaded.contains(&index) {
                loaded.insert(index);
                println!("Switched to tab {}: ", index);

                match index {
                    1 => {
                        gtk::glib::spawn_future_local(load_latest_tab(
                            cards_containers[index as usize].clone(),
                        ));
                    }
                    2 => {
                        gtk::glib::spawn_future_local(load_ongoing_tab(
                            cards_containers[index as usize].clone(),
                        ));
                    }
                    3 => {
                        gtk::glib::spawn_future_local(load_announce_tab(
                            cards_containers[index as usize].clone(),
                        ));
                    }
                    4 => {
                        gtk::glib::spawn_future_local(load_completed_tab(
                            cards_containers[index as usize].clone(),
                        ));
                    }
                    5 => {
                        gtk::glib::spawn_future_local(load_movies_tab(
                            cards_containers[index as usize].clone(),
                        ));
                    }
                    _ => {
                        println!("Tab {} is not implemented yet", index);
                    }
                }
            } else {
                println!("Tab {} already loaded", index);
            }
        }
    });

    gtk::glib::spawn_future_local(load_latest_tab(
        cards_containers[active_tab as usize].clone(),
    ));

    ui.view_stack.connect_visible_child_notify({
        let ui = ui.clone();
        move |stack| match stack.visible_child_name().unwrap().as_str() {
            "home" => println!("Home tab activated"),
            "review" => println!("Review tab activated"),
            "bookmarks" => {
                gtk::glib::spawn_future_local(load_bookmarks_page(
                    ui.bookmarks_tabs.clone(),
                    ui.bookmarks_cards_containers.clone(),
                ));
            }
            _ => (),
        }
    });

    // ui.window.add_css_class("low-perf");
    ui.window.present();
}

async fn load_latest_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    let filter = filter::FilterRequestBuilder::new().build();
    match client.filter(filter, 0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_ongoing_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    let filter = filter::FilterRequestBuilder::new()
        .status_id(Some(2))
        .build();
    match client.filter(filter, 0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_announce_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    let filter = filter::FilterRequestBuilder::new()
        .status_id(Some(3))
        .build();
    match client.filter(filter, 0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_completed_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    let filter = filter::FilterRequestBuilder::new()
        .status_id(Some(1))
        .build();
    match client.filter(filter, 0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_movies_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    let filter = filter::FilterRequestBuilder::new()
        .category_id(Some(2))
        .build();
    match client.filter(filter, 0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

// async fn load_review_page() {
//     let client = api_client::AnixartClient::global();
// }

async fn load_bookmarks_page(tabs: gtk::Notebook, cards_containers: Vec<gtk::Box>) {
    let loaded_tabs: Rc<RefCell<HashSet<u32>>> = Rc::new(RefCell::new(HashSet::new()));

    load_bookmark_page(2, cards_containers.clone()).await;

    tabs.connect_switch_page({
        let loaded_tabs = loaded_tabs.clone();
        move |_, _, index| {
            let mut loaded = loaded_tabs.borrow_mut();
            if !loaded.contains(&index) {
                loaded.insert(index);
                gtk::glib::spawn_future_local(load_bookmark_page(index as usize, cards_containers.clone()));
                println!("Switched to tab {}: ", index);
            } else {
                println!("Tab {} already loaded", index);
            }
        }
    });
}

async fn load_bookmark_page(index: usize, cards_containers: Vec<gtk::Box>) {
    match index {
        0 => load_collection_tab(cards_containers[index].clone()).await,
        1 => load_history_tab(cards_containers[index].clone()).await,
        2 => load_favorite_tab(cards_containers[index].clone()).await,
        3 => load_profile_list(1, cards_containers[index].clone()).await,
        4 => load_profile_list(2, cards_containers[index].clone()).await,
        5 => load_profile_list(3, cards_containers[index].clone()).await,
        6 => load_profile_list(4, cards_containers[index].clone()).await,
        7 => load_profile_list(5, cards_containers[index].clone()).await,
        _ => {
            println!("Tab {} is not implemented yet", index);
        }
    }
}

async fn load_history_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    match client.history(0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_favorite_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    match client.favorite(0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_profile_list(status_id: u32, cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    match client.profile_list(status_id, 0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn load_collection_tab(cards_container: gtk::Box) {
    let client = api_client::AnixartClient::global();
    match client.collection_favorite(0).await {
        Ok(json) => show_titles(cards_container, json).await,
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
    };
}

async fn show_titles(cards_container: gtk::Box, json: serde_json::Value) {
    if let Some(content) = json["content"].as_array() {
        for item in content {
            if let (Some(title), Some(desc), Some(image), grade_opt) = (
                item["title_ru"].as_str(),
                item["description"].as_str(),
                item["image"].as_str(),
                item["grade"].as_f64(),
            ) {
                let episodes_released = item["episodes_released"].as_u64();
                let episodes_total = item["episodes_total"].as_u64();

                let average = if let Some(grade) = grade_opt {
                    if grade > 0.0 {
                        format!("{:.1} ⭐", grade)
                    } else {
                        String::from("")
                    }
                } else {
                    String::from("")
                };

                let subtitle = if episodes_released.is_none() && episodes_total.is_none() {
                    String::from("Анонс ? эп.")
                } else if episodes_released.is_none() || episodes_total.is_none() {
                    format!(
                        "{} из {} эп. {}",
                        episodes_released
                            .map(|v| v.to_string())
                            .unwrap_or("?".to_string()),
                        episodes_total
                            .map(|v| v.to_string())
                            .unwrap_or("?".to_string()),
                        average
                    )
                } else {
                    let released = episodes_released.unwrap();
                    let total = episodes_total.unwrap();
                    if released == total {
                        format!("{} эп • {}", released, average)
                    } else {
                        format!("{} из {} эп. {}", released, total, average)
                    }
                };

                let card = anicard::AnimeCard::new(image, title, &subtitle, desc);
                cards_container.append(&card);
                // glib::idle_add_local(move || glib::ControlFlow::Break);
            }
        }
    }
}
