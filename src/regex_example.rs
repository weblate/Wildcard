use glib::{ParamSpec, ParamSpecString, Value};
use gtk::{glib, prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct ExampleData {
    title: String,
    subtitle: String,
    regex: String,
    example: String,
}

impl ExampleData {
    pub fn new(title: &str, subtitle: &str, regex: &str, example: &str) -> Self {
        Self {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            regex: regex.to_string(),
            example: example.to_string(),
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn subtitle(&self) -> &str {
        self.subtitle.as_str()
    }

    pub fn regex(&self) -> &str {
        self.regex.as_str()
    }

    pub fn example(&self) -> &str {
        self.example.as_str()
    }
}

impl Default for ExampleData {
    fn default() -> Self {
        ExampleData {
            title: "Invalid title".to_string(),
            subtitle: "".to_string(),
            regex: "".to_string(),
            example: "".to_string(),
        }
    }
}

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RegexExample {
        pub data: RefCell<ExampleData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RegexExample {
        const NAME: &'static str = "RegexExample";
        type Type = super::RegexExample;
    }

    impl ObjectImpl for RegexExample {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("title").build(),
                    ParamSpecString::builder("subtitle").build(),
                    ParamSpecString::builder("regex").build(),
                    ParamSpecString::builder("example").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "title" => {
                    if let Ok(p) = value.get::<&str>() {
                        let new_data = ExampleData::new(
                            p,
                            &self.data.borrow().subtitle.clone(),
                            &self.data.borrow().regex.clone(),
                            &self.data.borrow().example.clone()
                        );
                        self.data.replace(new_data);
                    }
                }
                "subtitle" => {
                    if let Ok(p) = value.get::<&str>() {
                        let new_data = ExampleData::new(
                            &self.data.borrow().title.clone(),
                            p,
                            &self.data.borrow().regex.clone(),
                            &self.data.borrow().example.clone()
                        );
                        self.data.replace(new_data);
                    }
                }
                "regex" => {
                    if let Ok(p) = value.get::<&str>() {
                        let new_data = ExampleData::new(
                            &self.data.borrow().title.clone(),
                            &self.data.borrow().subtitle.clone(),
                            p,
                            &self.data.borrow().example.clone()
                        );
                        self.data.replace(new_data);
                    }
                }
                "example" => {
                    if let Ok(p) = value.get::<&str>() {
                        let new_data = ExampleData::new(
                            &self.data.borrow().title.clone(),
                            &self.data.borrow().subtitle.clone(),
                            &self.data.borrow().regex.clone(),
                            p
                        );
                        self.data.replace(new_data);
                    }
                }
                _ => unimplemented!()
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            let obj = self.obj();

            match pspec.name() {
                "title" => obj.imp().data.borrow().title.to_value(),
                "subtitle" => obj.imp().data.borrow().subtitle.to_value(),
                "regex" => obj.imp().data.borrow().regex.to_value(),
                "example" => obj.imp().data.borrow().example.to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct RegexExample(ObjectSubclass<imp::RegexExample>);
}

impl RegexExample {
    pub fn new(title: &str, subtitle: &str, regex: &str, example: &str) -> Self {
        glib::Object::builder()
            .property("title", &title)
            .property("subtitle", &subtitle)
            .property("regex", &regex)
            .property("example", &example)
            .build()
    }

    pub fn title(&self) -> String {
        let imp = self.imp();

        imp.data.borrow().title.clone()
    }

    pub fn subtitle(&self) -> String {
        let imp = self.imp();

        imp.data.borrow().subtitle.clone()
    }

    pub fn regex(&self) -> String {
        let imp = self.imp();

        imp.data.borrow().regex.clone()
    }

    pub fn example(&self) -> String {
        let imp = self.imp();

        imp.data.borrow().example.clone()
    }
}

