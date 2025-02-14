use gettextrs::gettext;
use std::cell::RefCell;
use std::rc::Rc;

use glib::subclass::Signal;
use glib::{clone, ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct LoginPage {
    username_entry: Rc<RefCell<gtk::Entry>>,
    password_entry: Rc<RefCell<gtk::Entry>>,
}

impl LoginPage {
    fn make_title() -> gtk::Label {
        let title = gtk::Label::new(None);
        title.set_markup(&format!("<b>{}</b>", gettext("Sign in to MyAnimeList")));
        title.set_halign(gtk::Align::Center);

        title
    }

    fn make_entry_label(text: &str) -> gtk::Label {
        let label = gtk::Label::new(None);
        label.set_markup(text);
        label.set_halign(gtk::Align::End);

        label
    }

    fn make_username_entry(&self) -> gtk::Entry {
        let entry = gtk::Entry::new();
        entry.set_hexpand(true);

        let this = self.to_owned();
        entry.connect_changed(clone!(@strong this => move |_| {
            this.obj().clone().notify(super::LoginPage::PASSWORD_PROPERTY);
            this.obj().clone().notify(super::LoginPage::READY_PROPERTY);
        }));
        entry.connect_activate(clone!(@strong this => move |_| {
            this.emit_activate(&this.obj());
        }));

        entry
    }

    fn make_password_entry(&self) -> gtk::Entry {
        let entry = gtk::Entry::new();
        entry.set_hexpand(true);
        entry.set_visibility(false);
        entry.set_invisible_char(Some('•'));

        let this = self.to_owned();
        entry.connect_changed(clone!(@strong this => move |_| {
            this.obj().clone().notify(super::LoginPage::PASSWORD_PROPERTY);
            this.obj().clone().notify(super::LoginPage::READY_PROPERTY);
        }));
        entry.connect_activate(clone!(@strong this => move |_| {
            this.emit_activate(&this.obj());
        }));

        entry
    }

    fn username(&self) -> String {
        self.username_entry.borrow().text().as_str().to_owned()
    }

    fn password(&self) -> String {
        self.password_entry.borrow().text().as_str().to_owned()
    }

    fn is_ready(&self) -> bool {
        !self.username().is_empty() && !self.password().is_empty()
    }

    fn emit_activate(&self, obj: &super::LoginPage) {
        if self.is_ready() {
            obj.emit_by_name::<()>(super::LoginPage::ACTIVATE_PROPERTY, &[]);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for LoginPage {
    const NAME: &'static str = "TundraLoginPage";
    type Type = super::LoginPage;
    type ParentType = gtk::Grid;
}

impl ObjectImpl for LoginPage {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_column_spacing(10);
        obj.set_row_spacing(10);
        obj.set_margin_start(10);
        obj.set_margin_end(10);
        obj.set_margin_top(10);
        obj.set_margin_bottom(10);
        obj.set_valign(gtk::Align::Center);

        let title = Self::make_title();
        obj.attach(&title, 0, 0, 2, 1);
        obj.attach(&Self::make_entry_label(&gettext("Username:")), 0, 1, 1, 1);
        obj.attach(&Self::make_entry_label(&gettext("Password:")), 0, 2, 1, 1);
        let username_entry = self.make_username_entry();
        obj.attach(&username_entry, 1, 1, 1, 1);
        let password_entry = self.make_password_entry();
        obj.attach(&password_entry, 1, 2, 1, 1);

        *self.username_entry.borrow_mut() = username_entry;
        *self.password_entry.borrow_mut() = password_entry;
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoolean::builder(super::LoginPage::READY_PROPERTY)
                    .blurb("Whether the login form has been filled out")
                    .default_value(false)
                    .flags(glib::ParamFlags::READABLE)
                    .build(),
                ParamSpecString::builder(super::LoginPage::USERNAME_PROPERTY)
                    .blurb("Value of the username field")
                    .default_value(Some(""))
                    .flags(glib::ParamFlags::READWRITE)
                    .build(),
                ParamSpecString::builder(super::LoginPage::PASSWORD_PROPERTY)
                    .blurb("Value of the password field")
                    .default_value(Some(""))
                    .flags(glib::ParamFlags::READWRITE)
                    .build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder(super::LoginPage::ACTIVATE_PROPERTY).build()]);
        SIGNALS.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            super::LoginPage::READY_PROPERTY => self.is_ready().to_value(),
            super::LoginPage::USERNAME_PROPERTY => self.username().to_value(),
            super::LoginPage::PASSWORD_PROPERTY => self.password().to_value(),
            _ => unimplemented!(),
        }
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            super::LoginPage::USERNAME_PROPERTY => {
                self.username_entry.borrow().set_text(value.get().unwrap())
            }
            super::LoginPage::PASSWORD_PROPERTY => {
                self.password_entry.borrow().set_text(value.get().unwrap())
            }
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for LoginPage {}

impl GridImpl for LoginPage {}
