use std::{cell::RefCell, rc::Rc};

use adw::{self};
use gdk::Display;
use gtk::{prelude::*, *};

use crate::utils::tab_loader::TabLoader;

pub const APP_ID: &str = "kz.findmyname284.anixartd";
pub const APP_PATH: &str = "/kz/findmyname284/anixartd";

// Структура для загрузки UI-шаблонов
#[derive(Clone)]
pub struct UiTemplates {
    pub main: Builder,
    pub home: Builder,
    pub bookmarks: Builder,
    pub login: Builder,
    pub anime_detail: Builder,
}

impl UiTemplates {
    pub fn load() -> Self {
        Self {
            main: Builder::from_resource(&format!("{}/ui/main_window.ui", APP_PATH)),
            home: Builder::from_resource(&format!("{}/ui/home_page.ui", APP_PATH)),
            bookmarks: Builder::from_resource(&format!("{}/ui/bookmarks_page.ui", APP_PATH)),
            login: Builder::from_resource(&format!("{}/ui/login_window.ui", APP_PATH)),
            anime_detail: Builder::from_resource(&format!("{}/ui/anime_detail.ui", APP_PATH)),
        }
    }
}

#[derive(Clone)]
pub struct AppUi {
    pub window: adw::ApplicationWindow,
    pub view_stack: adw::ViewStack,
    pub home_tabs: Notebook,
    pub bookmarks_tabs: Notebook,
    pub home_scrolled_windows: Vec<ScrolledWindow>,
    pub bm_scrolled_windows: Vec<ScrolledWindow>,
    pub home_cards_containers: Vec<gtk::Box>,
    pub bm_cards_containers: Vec<gtk::Box>,
    pub tab_loaders: RefCell<Vec<Rc<TabLoader>>>,
    pub templates: UiTemplates,
}

impl AppUi {
    pub fn new(app: &Application, templates: &UiTemplates) -> Self {
        let window: adw::ApplicationWindow = templates
            .main
            .object("main_window")
            .expect("Failed to find main_window in main template");
        window.set_application(Some(app));
        // window.set_hide_on_close(true);
        // window.fullscreen();

        let view_stack: adw::ViewStack = templates
            .main
            .object("view_stack")
            .expect("Failed to find view_stack in main template");

        view_stack.set_visible_child_name("home");

        let home_container: gtk::Box = templates
            .main
            .object("home_container")
            .expect("Failed to find home_container in main template");

        let home_content: gtk::Box = templates
            .home
            .object("home_content")
            .expect("Failed to find home_content in home template");
        home_container.append(&home_content);

        let anime_detail_container: gtk::Box = templates
            .main
            .object("anime_detail_container")
            .expect("Failed to find anime_detail_container in main template");

        let anime_detail_box: gtk::Box = templates
            .anime_detail
            .object("anime_detail_box")
            .expect("Failed to find anime_detail_box in anime_detail template");
        anime_detail_container.append(&anime_detail_box);

        let my_scrolled_window = templates
            .home
            .object("my_scrolled_window")
            .expect("Failed to find my_scrolled_window in home template");
        let latest_scrolled_window = templates
            .home
            .object("latest_scrolled_window")
            .expect("Failed to find latest_scrolled_window in home template");
        let ongoing_scrolled_window = templates
            .home
            .object("ongoing_scrolled_window")
            .expect("Failed to find ongoing_scrolled_window in home template");
        let announce_scrolled_window = templates
            .home
            .object("announce_scrolled_window")
            .expect("Failed to find announce_scrolled_window in home template");
        let completed_scrolled_window = templates
            .home
            .object("completed_scrolled_window")
            .expect("Failed to find completed_scrolled_window in home template");
        let movies_scrolled_window = templates
            .home
            .object("movies_scrolled_window")
            .expect("Failed to find movies_scrolled_window in home template");

        let home_scrolled_windows = vec![
            my_scrolled_window,
            latest_scrolled_window,
            ongoing_scrolled_window,
            announce_scrolled_window,
            completed_scrolled_window,
            movies_scrolled_window,
        ];

        let collections_scrolled_window: ScrolledWindow = templates
            .bookmarks
            .object("collections_scrolled_window")
            .expect("Failed to find collections_scrolled_window in bookmarks template");

        let history_scrolled_window = templates
            .bookmarks
            .object("history_scrolled_window")
            .expect("Failed to find history_scrolled_window in bookmarks template");

        let favorite_scrolled_window = templates
            .bookmarks
            .object("favorite_scrolled_window")
            .expect("Failed to find favorite_scrolled_window in bookmarks template");

        let watching_scrolled_window = templates
            .bookmarks
            .object("watching_scrolled_window")
            .expect("Failed to find watching_scrolled_window in bookmarks template");

        let plans_scrolled_window = templates
            .bookmarks
            .object("plans_scrolled_window")
            .expect("Failed to find plans_scrolled_window in bookmarks template");

        let viewed_scrolled_window = templates
            .bookmarks
            .object("viewed_scrolled_window")
            .expect("Failed to find viewed_scrolled_window in bookmarks template");

        let hold_scrolled_window = templates
            .bookmarks
            .object("hold_scrolled_window")
            .expect("Failed to find hold_scrolled_window in bookmarks template");

        let dropped_scrolled_window = templates
            .bookmarks
            .object("dropped_scrolled_window")
            .expect("Failed to find dropped_scrolled_window in bookmarks template");

        let bm_scrolled_windows = vec![
            collections_scrolled_window,
            history_scrolled_window,
            favorite_scrolled_window,
            watching_scrolled_window,
            plans_scrolled_window,
            viewed_scrolled_window,
            hold_scrolled_window,
            dropped_scrolled_window,
        ];

        let bookmarks_container: gtk::Box = templates
            .main
            .object("bookmarks_container")
            .expect("Failed to find bookmarks_container in main template");

        let bookmarks_content: gtk::Box = templates
            .bookmarks
            .object("bookmarks_content")
            .expect("Failed to find bookmarks_content in bookmarks template");
        bookmarks_container.append(&bookmarks_content);

        let home_tabs: Notebook = templates
            .home
            .object("tabs")
            .expect("Failed to find tabs in home template");

        home_tabs.set_current_page(Some(1));

        let my_tab_cards_container: gtk::Box = templates
            .home
            .object("my_tab_cards_container")
            .expect("Failed to find my_tab_cards_container in home template");

        let latest_tab_cards_container: gtk::Box = templates
            .home
            .object("latest_tab_cards_container")
            .expect("Failed to find latest_tab_cards_container in home template");

        let ongoing_tab_cards_container: gtk::Box = templates
            .home
            .object("ongoing_tab_cards_container")
            .expect("Failed to find ongoing_tab_cards_container in home template");

        let announce_tab_cards_container: gtk::Box = templates
            .home
            .object("announce_tab_cards_container")
            .expect("Failed to find announce_tab_cards_container in home template");

        let completed_tab_cards_container: gtk::Box = templates
            .home
            .object("completed_tab_cards_container")
            .expect("Failed to find completed_tab_cards_container in home template");

        let movies_tab_cards_container: gtk::Box = templates
            .home
            .object("movies_tab_cards_container")
            .expect("Failed to find movies_tab_cards_container in home template");

        let bookmarks_tabs: Notebook = templates
            .bookmarks
            .object("tabs")
            .expect("Failed to find tabs in bookmarks template");
        bookmarks_tabs.set_current_page(Some(2));

        let collections_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("collections_tab_cards_container")
            .expect("Failed to find collections_tab_cards_container in bookmarks template");

        let history_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("history_tab_cards_container")
            .expect("Failed to find history_tab_cards_container in bookmarks template");

        let favorite_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("favorite_tab_cards_container")
            .expect("Failed to find favorite_tab_cards_container in bookmarks template");

        let watching_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("watching_tab_cards_container")
            .expect("Failed to find watching_tab_cards_container in bookmarks template");

        let plans_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("plans_tab_cards_container")
            .expect("Failed to find plans_tab_cards_container in bookmarks template");

        let viewed_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("viewed_tab_cards_container")
            .expect("Failed to find viewed_tab_cards_container in bookmarks template");

        let hold_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("hold_tab_cards_container")
            .expect("Failed to find hold_tab_cards_container in bookmarks template");

        let dropped_tab_cards_container: gtk::Box = templates
            .bookmarks
            .object("dropped_tab_cards_container")
            .expect("Failed to find dropped_tab_cards_container in bookmarks template");

        let bm_cards_containers = vec![
            collections_tab_cards_container,
            history_tab_cards_container,
            favorite_tab_cards_container,
            watching_tab_cards_container,
            plans_tab_cards_container,
            viewed_tab_cards_container,
            hold_tab_cards_container,
            dropped_tab_cards_container,
        ];

        let home_cards_containers = vec![
            my_tab_cards_container,
            latest_tab_cards_container,
            ongoing_tab_cards_container,
            announce_tab_cards_container,
            completed_tab_cards_container,
            movies_tab_cards_container,
        ];

        Self {
            window,
            templates: templates.clone(),
            view_stack,
            home_tabs,
            bookmarks_tabs,
            home_scrolled_windows,
            bm_scrolled_windows,
            home_cards_containers,
            bm_cards_containers,
            tab_loaders: Vec::new().into(),
        }
    }
}

pub fn set_dark_theme(gtk_application_prefer_dark_theme: bool) {
    if let Some(settings) = Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(gtk_application_prefer_dark_theme);
    }
}

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("style.css");

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

pub fn is_renderer_cairo() -> bool {
    let name = std::env::var("GSK_RENDERER")
        .unwrap_or_else(|_| String::from("unknown"))
        .to_lowercase();

    name == "cairo"
}

pub fn disable_animation(gtk_enable_animations: bool) {
    if let Some(settings) = Settings::default() {
        settings.set_gtk_enable_animations(!gtk_enable_animations);
    }
}

#[derive(Clone)]
pub struct LoginUi {
    pub window: gtk::ApplicationWindow,
    pub login_button: gtk::Button,
    pub skip_button: gtk::Button,
    pub username_entry: gtk::Entry,
    pub password_entry: gtk::Entry,
}

impl LoginUi {
    pub fn new(templates: &UiTemplates) -> Self {
        let window: gtk::ApplicationWindow = templates.login.object("login_window").unwrap();
        let login_button: gtk::Button = templates.login.object("login_button").unwrap();
        let skip_button: gtk::Button = templates.login.object("skip_button").unwrap();
        let username_entry: gtk::Entry = templates.login.object("username_row").unwrap();
        let password_entry: gtk::Entry = templates.login.object("password_row").unwrap();

        // Настройка логотипа
        let logo: gtk::Image = templates.login.object("logo").unwrap();
        let theme = "dark";
        logo.set_resource(Some(&format!(
            "{}/img/logo_{}.png",
            APP_PATH,
            if theme == "light" { "dark" } else { "light" }
        )));

        Self {
            window,
            login_button,
            skip_button,
            username_entry,
            password_entry,
        }
    }
}
