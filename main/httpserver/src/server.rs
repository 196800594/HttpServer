use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    sock_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(sock_addr: &'a str) -> Self {
        Server { sock_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.sock_addr).unwrap();
        println!("Running on {}", self.sock_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established!");

            let mut read_buffer = [0; 1024];
            stream.read(&mut read_buffer).unwrap();

            let req:HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}