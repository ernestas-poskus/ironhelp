#![feature(plugin)]
#![feature(proc_macro)]

extern crate maud;

use maud::{Markup, Render, html};

pub struct Css<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Render for Css<T> {
    fn render(&self) -> Markup {
        html! {
            link rel="stylesheet" href=(self.0.as_ref()) /
        }
    }
}

pub struct Js<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Render for Js<T> {
    fn render(&self) -> Markup {
        html! {
            script src=(self.0.as_ref()) {}
        }
    }
}

pub struct Meta<T: AsRef<str>, U: AsRef<str>>(pub T, pub U);

impl<T: AsRef<str>, U: AsRef<str>> Render for Meta<T, U> {
    fn render(&self) -> Markup {
        html! {
            meta name=(self.0.as_ref()) content=(self.1.as_ref()) /
        }
    }
}

pub struct Title<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Render for Title<T> {
    fn render(&self) -> Markup {
        html! {
            title (self.0.as_ref())
        }
    }
}

pub struct Charset<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> Render for Charset<T> {
    fn render(&self) -> Markup {
        html! {
            meta charset=(self.0.as_ref()) /
        }
    }
}

pub struct MetaProperty<T: AsRef<str>, U: AsRef<str>>(pub T, pub U);

impl<T: AsRef<str>, U: AsRef<str>> Render for MetaProperty<T, U> {
    fn render(&self) -> Markup {
        html! {
            meta property=(self.0.as_ref()) content=(self.1.as_ref()) /
        }
    }
}

pub struct MetaRobots {
    pub index: bool,
    pub follow: bool,
}

impl Render for MetaRobots {
    fn render(&self) -> Markup {
        let index = if self.index { "index" } else { "noindex" };
        let follow = if self.follow { "follow" } else { "nofollow" };
        html! {
            meta name="robots" content={ (index) "," (follow) } /
        }
    }
}
