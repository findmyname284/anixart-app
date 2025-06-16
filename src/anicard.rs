use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct AnimeCard(ObjectSubclass<imp::AnimeCardImp>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible;
}

mod imp {
    use super::*;
    use gtk::{CompositeTemplate, TemplateChild};

    #[derive(CompositeTemplate, Default)]
    #[template(file = "anicard.ui")]
    pub struct AnimeCardImp {
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub subtitle: TemplateChild<gtk::Label>,
        #[template_child]
        pub info: TemplateChild<gtk::Label>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AnimeCardImp {
        const NAME: &'static str = "AnimeCard";
        type Type = AnimeCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AnimeCardImp {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for AnimeCardImp {}
    impl BoxImpl for AnimeCardImp {}
}

impl AnimeCard {
    pub fn new(
        title: &str,
        subtitle: &str,
        info: &str,
        description: &str,
    ) -> Self {
        let obj: Self = glib::Object::new::<Self>();
        obj.set_properties(title, subtitle, info, description);
        obj
    }

    pub fn set_properties(
        &self,
        title: &str,
        subtitle: &str,
        info: &str,
        description: &str,
    ) {
        let imp = self.imp();
        imp.title.set_text(title);
        imp.subtitle.set_text(subtitle);
        imp.info.set_text(info);
        imp.description.set_text(description);
    }
}