extern crate colored;
extern crate futures;
extern crate hyper;
extern crate select;
extern crate tokio_core;

use colored::*;

use futures::{Future, Stream};

use hyper::Client;

use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

use std::io::{self, Write, Read};

use tokio_core::reactor::Core;

fn main() {
    //let hackernews = open_testing();
    let hackernews = open_hacker_news();
    let document = Document::from(hackernews.as_str());

    let test_results = open_hacker_news();
    println!("{}", test_results);

    //for node in document.find(Class("athing")){
        //let score = node.find(Class("rank"))
            //.next()
            //.unwrap()
            //.text();

        //let storylink = node.find(Class("storylink")).next().unwrap();
        //let title = storylink.text();
        //let link = storylink.attr("href").unwrap();

        //println!("{} {} ({:?})", score.blue(), title.white().bold(), link)
    //}
}

fn open_hacker_news() -> String {
    // Setup a Tokio Core which works as an event loop for handling requests
    // - we use this in our client to handle responses efficiently
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://news.ycombinator.com/news".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map(|_| ()).map_err(From::from)
        })
    });

    core.run(work).unwrap();

    let mut body = String::new();
    //response.read_to_string(&mut body).expect("failed to read request to string");

    body
}

fn open_testing() -> &'static str {
    // include_str! is a macro that will read in a file as a string
    include_str!("hacker-news.html")
}

