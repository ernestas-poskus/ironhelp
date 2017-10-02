use maud::{Markup, PreEscaped};
use maud::html;

/// Navigation Item <a> anchor link
pub struct Item {
    /// anchor link href attribute
    pub path: &'static str,
    /// anchor link title attribute
    pub title: &'static str,
    /// anchor link text attribute
    pub text: &'static str,
}

/// Navigation struct based on Foundation 6 CSS framework
/// possible to define title and 2 menus (left and right).
pub struct Navigation {
    /// Given title to navigation bar
    /// Possible to pass raw safe HTML output is pre escaped
    pub title: Option<&'static Item>,
    /// Left menu items
    pub left: Option<&'static [&'static Item]>,
    /// Right menu items
    pub right: Option<&'static [&'static Item]>,
}

impl Navigation {
    /// Render function which accepts current path as an argument
    /// making it possible to mark 'active' links
    pub fn render(&self, path: &str) -> Markup {
        html!(
            div.nav-top {
                nav.top-bar role="navigation" {
                    @if let Some(ref t) = self.title {
                        div.top-bar-title {
                            a href=(t.path) title=(t.title) (PreEscaped(t.text))
                        }
                    }
                    @if let Some(left_nav) = self.left {
                        div.top-bar-left {
                            ul.menu {
                                @for lt in left_nav {
                                    @if path.starts_with(lt.path) {
                                        li a.active href=(lt.path) title=(lt.title) (lt.text)
                                    } @else {
                                        li a href=(lt.path) title=(lt.title) (lt.text)
                                    }
                                }
                            }
                        }
                    }
                    @if let Some(right_nav) = self.right {
                        div.top-bar-right {
                            ul.menu {
                                @for rt in right_nav {
                                    @if path.starts_with(rt.path) {
                                        li a.active href=(rt.path) title=(rt.title) (rt.text)
                                    } @else {
                                        li a href=(rt.path) title=(rt.title) (rt.text)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        )
    }
}
