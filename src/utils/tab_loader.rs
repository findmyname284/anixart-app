use std::{cell::Cell, pin::Pin, rc::Rc};

use gtk::prelude::*;

use crate::{show_titles, utils::ui::{AppUi}};

pub struct TabLoader {
    container: gtk::Box,
    load_fn:
        Rc<dyn Fn(u32) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, reqwest::Error>>>>>,
    page: Cell<u32>,
    loading: Cell<bool>,
    ui: Rc<AppUi>,
}

impl TabLoader {
    pub fn new<Fut>(container: &gtk::Box, ui: Rc<AppUi>, load_fn: impl Fn(u32) -> Fut + 'static) -> Rc<Self>
    where
        Fut: Future<Output = Result<serde_json::Value, reqwest::Error>> + 'static,
    {
        Rc::new(Self {
            container: container.clone(),
            load_fn: Rc::new(move |page| Box::pin(load_fn(page))),
            page: Cell::new(0),
            loading: Cell::new(false),
            ui,
        })
    }

    pub fn connect_scroll(self: &Rc<Self>, scrolled: &gtk::ScrolledWindow) {
        let vadj = scrolled.vadjustment();
        vadj.connect_value_changed({
            let this = Rc::clone(&self);
            move |adj| {
                let value = adj.value();
                let upper = adj.upper();
                let page_size = adj.page_size();
                let threshold = 50.0;

                if upper - (value + page_size) < threshold && !this.loading.get() {
                    this.load_more();
                }
            }
        });
    }

    pub fn load_more(self: &Rc<Self>) {
        if self.loading.get() {
            return;
        }
        self.loading.set(true);

        let container = self.container.clone();
        let loader = self.clone();
        let page = self.page.get();
        let load_fn = self.load_fn.clone();
        let ui = self.ui.clone();

        gtk::glib::MainContext::default().spawn_local(async move {
            match load_fn(page).await {
                Ok(json) => show_titles(ui, container.clone(), json).await,
                Err(err) => eprintln!("Load error: {}", err),
            }
            loader.page.set(page + 1);
            loader.loading.set(false);
        });
    }
}
