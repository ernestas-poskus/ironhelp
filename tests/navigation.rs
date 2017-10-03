extern crate ironhelp;

use ironhelp::navigation::{Navigation, Item};

const ITEMS: &'static [&'static Item] = &[
    &Item {
        path: "/tx1",
        title: "Some title",
        text: "TEXT1",
    },
    &Item {
        path: "/tx2",
        title: "Some title",
        text: "TEXT2",
    },
    &Item {
        path: "/tx3",
        title: "Some title",
        text: "TEXT3",
    },
];

static NAV1: Navigation = Navigation {
    title: Some(&ITEMS[0]),
    left: None,
    right: Some(&ITEMS),
};

static NAV2: Navigation = Navigation {
    title: Some(&ITEMS[0]),
    left: Some(&ITEMS),
    right: None,
};

static NAV3: Navigation = Navigation {
    title: None,
    left: Some(&ITEMS),
    right: Some(&ITEMS),
};

#[test]
fn test_nav1() {
    assert_eq!(NAV1.render("/tx1").into_string(), r#"<div class="nav-top"><nav class="top-bar"><div class="top-bar-title"><a href="/tx1" title="Some title">TEXT1</a></div><div class="top-bar-right"><ul class="menu"><li><a href="/tx1" title="Some title" class="active">TEXT1</a></li><li><a href="/tx2" title="Some title">TEXT2</a></li><li><a href="/tx3" title="Some title">TEXT3</a></li></ul></div></nav></div>"#);
}

#[test]
fn test_nav2() {
    assert_eq!(NAV2.render("/tx3").into_string(), r#"<div class="nav-top"><nav class="top-bar"><div class="top-bar-title"><a href="/tx1" title="Some title">TEXT1</a></div><div class="top-bar-left"><ul class="menu"><li><a href="/tx1" title="Some title">TEXT1</a></li><li><a href="/tx2" title="Some title">TEXT2</a></li><li><a href="/tx3" title="Some title" class="active">TEXT3</a></li></ul></div></nav></div>"#);
}

#[test]
fn test_nav3() {
    assert_eq!(NAV3.render("/tx2").into_string(), r#"<div class="nav-top"><nav class="top-bar"><div class="top-bar-left"><ul class="menu"><li><a href="/tx1" title="Some title">TEXT1</a></li><li><a href="/tx2" title="Some title" class="active">TEXT2</a></li><li><a href="/tx3" title="Some title">TEXT3</a></li></ul></div><div class="top-bar-right"><ul class="menu"><li><a href="/tx1" title="Some title">TEXT1</a></li><li><a href="/tx2" title="Some title" class="active">TEXT2</a></li><li><a href="/tx3" title="Some title">TEXT3</a></li></ul></div></nav></div>"#);
}

#[test]
fn test_nav_no_such_path() {
    assert_eq!(NAV3.render("waaw").into_string(), r#"<div class="nav-top"><nav class="top-bar"><div class="top-bar-left"><ul class="menu"><li><a href="/tx1" title="Some title">TEXT1</a></li><li><a href="/tx2" title="Some title">TEXT2</a></li><li><a href="/tx3" title="Some title">TEXT3</a></li></ul></div><div class="top-bar-right"><ul class="menu"><li><a href="/tx1" title="Some title">TEXT1</a></li><li><a href="/tx2" title="Some title">TEXT2</a></li><li><a href="/tx3" title="Some title">TEXT3</a></li></ul></div></nav></div>"#);
}
