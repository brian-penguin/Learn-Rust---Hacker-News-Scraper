extern crate colored;
extern crate select;
extern crate reqwest;

use colored::*;

use select::document::Document;
use select::predicate::Class;
//use select::node::Node;

use std::io::Read;

fn main() {
    //let hackernews = open_testing();
    let hackernews = open_hacker_news();
    let document = Document::from(hackernews.as_str());

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

fn open_hacker_news() -> String {
    let mut resp = reqwest::get("https://news.ycombinator.com").unwrap();
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    content
}

//fn open_testing() -> &'static str {
    //// include_str! is a macro that will read in a file as a string
    //include_str!("hacker-news.html")
//}

