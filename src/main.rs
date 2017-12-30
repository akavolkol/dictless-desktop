extern crate gtk;
extern crate glib;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use gtk::prelude::*;
use std::process::{ Command };
use gtk::{
    Window,
    MenuBar,
    MenuItem,
    Label,
    AboutDialog,
    Box,
    Orientation,
    ScrolledWindow,
    Widget,
    SelectionData
};
use glib::object::GObject;
use glib::signal::connect;
use std::mem::transmute;
use std::boxed::Box as Box_;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

mod ui;


fn main() {

t()
    //init();
}

fn t() {
       let url = "http://google.com.ua";


    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return;
    }

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .no_proto()
        .build(&handle);

    let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
       // println!("Headers: \n{}", res.headers());

        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("\n\nDone.");
    });

core.run(work).unwrap();
}

fn init() {
     ui::run();
}

