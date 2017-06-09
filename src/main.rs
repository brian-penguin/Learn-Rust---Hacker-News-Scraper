extern crate colored;
extern crate hyper;
extern crate select;

use colored::*;

use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

fn main() {
    let hackernews = open_testing();
    let document = Document::from(hackernews);
    //println!("{}", hackernews);

    for node in document.find(Class("athing")){
        let score = node.find(Class("rank"))
            .next()
            .unwrap()
            .text();

        let storylink = node.find(Class("storylink")).next().unwrap();
        let title = storylink.text();
        let link = storylink.attr("href").unwrap();

        println!("{} {} ({:?})", score.blue(), title.white().bold(), link)
    }
}

fn open_testing() -> &'static str {
    // include_str! is a macro that will read in a file as a string
    include_str!("hacker-news.html")
}
