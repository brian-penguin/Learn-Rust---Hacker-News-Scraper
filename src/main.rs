extern crate hyper;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

struct Article {
    title: String,
    link: String,
}

impl Article {

}

fn main() {
    let hackernews = open_testing();
    let document = Document::from(hackernews);
    //println!("{}", hackernews);

    for node in document.find(Class("storylink")) {
        println!("{} ({:?})", node.text(), node.attr("href").unwrap());
        println!("---")
    }

}

fn open_testing() -> &'static str {
    // include_str! is a macro that will read in a file as a string
    include_str!("hacker-news.html")
}
