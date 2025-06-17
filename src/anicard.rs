use crate::utils::image_download::*;
use gtk::subclass::prelude::*;
use gtk::{Label, Image, glib};

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
        pub image: TemplateChild<Image>,
        #[template_child]
        pub title: TemplateChild<Label>,
        #[template_child]
        pub description: TemplateChild<Label>,
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
    pub fn new(image: &str, title: &str, description: &str) -> Self {
        let obj: Self = glib::Object::new::<Self>();
        obj.set_properties(image, title, description);
        obj
    }

    pub fn set_properties(&self, image: &str, title: &str, description: &str) {
        let imp = self.imp();
        
        load_image(&imp.image, image);
        imp.title.set_text(title);
        imp.description.set_text(description);
    }
}
