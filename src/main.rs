mod anicard;
mod api_client;
mod utils;
mod windows;

use crate::utils::image::load_image;
use crate::utils::tab_loader::TabLoader;
use crate::utils::{config::Config, filter, ui::*};
use crate::windows::login_window::show_login_window;
use gtk::{Application, GestureClick};
use gtk::{Label, prelude::*};
use std::collections::HashSet;
use std::pin::Pin;
use std::{cell::RefCell, rc::Rc};

// use std::sync::{Arc, RwLock};

// static GLOBAL_UI: RwLock<Option<Arc<AppUi>>> = RwLock::new(None);

async fn my_task() {
    println!("Requesting IP address...");
    let client = api_client::AnixartClient::global();
    if let Ok(ip) = client.get_ip().await {
        println!("IP address: {:#?}", ip);
    } else {
        println!("Error fetching IP address");
    }
}

#[tokio::main]
async fn main() {
    let task = tokio::task::spawn(my_task());
    task.await.unwrap();

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

    let init_tab = ui.home_tabs.current_page().unwrap_or(1) as usize;

    let fetch_fns: Vec<_> = vec![
        |page| {
            api_client::AnixartClient::global()
                .filter(filter::FilterRequestBuilder::new().build(), page)
        },
        |page| {
            api_client::AnixartClient::global()
                .filter(filter::FilterRequestBuilder::new().build(), page)
        },
        |page| {
            api_client::AnixartClient::global().filter(
                filter::FilterRequestBuilder::new()
                    .status_id(Some(2))
                    .build(),
                page,
            )
        },
        |page| {
            api_client::AnixartClient::global().filter(
                filter::FilterRequestBuilder::new()
                    .status_id(Some(3))
                    .build(),
                page,
            )
        },
        |page| {
            api_client::AnixartClient::global().filter(
                filter::FilterRequestBuilder::new()
                    .status_id(Some(1))
                    .build(),
                page,
            )
        },
        |page| {
            api_client::AnixartClient::global().filter(
                filter::FilterRequestBuilder::new()
                    .category_id(Some(2))
                    .build(),
                page,
            )
        },
    ];
    for (i, loader_fn) in fetch_fns.into_iter().enumerate() {
        let container = &ui.home_cards_containers[i];
        let scrolled = &ui.home_scrolled_windows[i];
        let loader = TabLoader::new(container, Rc::new(ui.clone()), loader_fn);
        loader.connect_scroll(scrolled);
        ui.tab_loaders.borrow_mut().push(loader);
    }

    let loaded = Rc::new(RefCell::new(HashSet::new()));
    loaded.borrow_mut().insert(init_tab);

    let initial_loader = ui.tab_loaders.borrow()[init_tab].clone();
    initial_loader.load_more();

    let tabs_switchers = ui.home_tabs.clone();
    let tab_loaders = ui.tab_loaders.clone();
    let loaded_flag = loaded.clone();

    tabs_switchers.connect_switch_page(move |_, _, index| {
        let idx = index as usize;
        let mut loaded = loaded_flag.borrow_mut();
        if !loaded.contains(&idx) {
            loaded.insert(idx);
            let loader = tab_loaders.borrow()[idx].clone();
            loader.load_more();
        }
    });

    let init_bm = ui.bookmarks_tabs.current_page().unwrap_or(2) as usize;
    let bm_containers = ui.bm_cards_containers.clone();
    let bm_loader_refs: Rc<RefCell<Vec<Rc<TabLoader>>>> = Rc::new(RefCell::new(Vec::new()));

    let fetch_bm: Vec<
        Box<
            dyn Fn(u32) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, reqwest::Error>>>>,
        >,
    > = vec![
        Box::new(|page| Box::pin(api_client::AnixartClient::global().collection_favorite(page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().history(page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().favorite(page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().profile_list(1, page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().profile_list(2, page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().profile_list(3, page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().profile_list(4, page))),
        Box::new(|page| Box::pin(api_client::AnixartClient::global().profile_list(5, page))),
    ];

    for (i, fetch_fn) in fetch_bm.into_iter().enumerate() {
        let container = &bm_containers[i];
        let scrolled = &ui.bm_scrolled_windows[i];
        let loader = TabLoader::new(container, Rc::new(ui.clone()), move |p| fetch_fn(p));
        loader.connect_scroll(scrolled);
        bm_loader_refs.borrow_mut().push(loader);
    }

    let bm_loaded = Rc::new(RefCell::new(HashSet::new()));

    ui.view_stack.connect_visible_child_notify({
        let ui = ui.clone();
        move |stack| match stack.visible_child_name().unwrap().as_str() {
            "home" => stack.set_child_visible(true),
            "review" => {}
            "bookmarks" => {
                load_bookmarks_page(
                    ui.bookmarks_tabs.clone(),
                    bm_loader_refs.clone(),
                    bm_loaded.clone(),
                    init_bm,
                );
            }
            _ => (),
        }
    });

    // ui.window.add_css_class("low-perf");
    ui.window.present();
}

fn load_bookmarks_page(
    bookmarks_tabs: gtk::Notebook,
    bm_loader_refs: Rc<RefCell<Vec<Rc<TabLoader>>>>,
    bm_loaded: Rc<RefCell<HashSet<usize>>>,
    init_bm: usize,
) {
    if !bm_loaded.borrow().contains(&init_bm) {
        bm_loaded.borrow_mut().insert(init_bm);
        bm_loader_refs.borrow()[init_bm].clone().load_more();
    }
    bookmarks_tabs.connect_switch_page({
        let loaders = bm_loader_refs.clone();
        let loaded = bm_loaded.clone();
        move |_, _, index| {
            let idx = index as usize;
            if !loaded.borrow().contains(&idx) {
                loaded.borrow_mut().insert(idx);
                loaders.borrow()[idx].clone().load_more();
            }
        }
    });
}

async fn show_titles(ui: Rc<AppUi>, cards_container: gtk::Box, json: serde_json::Value) {
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
                card.add_css_class("anicard");
                cards_container.append(&card);
                let click = GestureClick::new();
                click.connect_released({
                    let item = item.clone();
                    let ui = ui.clone();
                    move |_, _, _, _| {
                        show_details(ui.clone(), item.clone());
                    }
                });
                card.add_controller(click);
            }
        }
    }
}

fn show_details(ui: Rc<AppUi>, item: serde_json::Value) {
    println!("Details: {:#?}", item);
    let id = item["id"].as_u64().unwrap();
    let title = item["title_ru"].as_str().unwrap();
    let title_original = item["title_original"].as_str().unwrap();
    let genres = item["genres"].as_str().unwrap();
    let studio = item["studio"].as_str().unwrap();
    let desc = item["description"].as_str().unwrap();
    let image = item["image"].as_str().unwrap();
    ui.view_stack.set_visible_child_name("details");

    let builder = ui.templates.anime_detail.clone();

    let title_label: gtk::Label = builder.object("anime_title_label").unwrap();
    let original_label: gtk::Label = builder.object("original_label").unwrap();
    let poster_image: gtk::Picture = builder.object("poster_image").unwrap();
    let genres_label: gtk::Label = builder.object("genres_label").unwrap();
    let studio_label: gtk::Label = builder.object("studio_label").unwrap();

    title_label.set_text(title);
    original_label.set_text(title_original);
    genres_label.set_text(genres);
    studio_label.set_text(studio);
    load_image(&poster_image, image);
}
