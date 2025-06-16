use adw::{self};
use gdk::Display;
use gtk::{prelude::*, *};

pub const APP_ID: &str = "kz.findmyname284.anixartd";
pub const APP_PATH: &str = "/kz/findmyname284/anixartd";

// Структура для загрузки UI-шаблонов
pub struct UiTemplates {
    pub main: Builder,
    pub home: Builder,
    pub login: Builder,
}

impl UiTemplates {
    pub fn load() -> Self {
        Self {
            main: Builder::from_resource(&format!("{}/ui/main_window.ui", APP_PATH)),
            home: Builder::from_resource(&format!("{}/ui/home_page.ui", APP_PATH)),
            login: Builder::from_resource(&format!("{}/ui/login_window.ui", APP_PATH)),
        }
    }
}

// Структура основного UI
pub struct AppUi {
    pub window: adw::ApplicationWindow,
    pub view_stack: adw::ViewStack,
    pub cards_container: gtk::Box,
}

impl AppUi {
    pub fn new(app: &Application, templates: &UiTemplates) -> Self {
        let window: adw::ApplicationWindow = templates.main.object("main_window").unwrap();
        window.set_application(Some(app));

        let view_stack: adw::ViewStack = templates.main.object("view_stack").unwrap();
        let home_container: gtk::Box = templates.main.object("home_container").unwrap();

        let home_content: gtk::Box = templates.home.object("home_content").unwrap();
        home_container.append(&home_content);

        let cards_container: gtk::Box = templates.home.object("cards_container").unwrap();

        Self {
            window,
            view_stack,
            cards_container,
        }
    }
}

// Загрузчик CSS
pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../../resources/style.css"));

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

// Инициализация UI для окна логина
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
        let theme = "dark"; // В реальном приложении получать из настроек
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
