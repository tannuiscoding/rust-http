mod request;
mod response;
mod router;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::{
    request::{parse_request, Method, Request},
    response::Response,
    router::Router,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("bind failed");
    println!("rustHTTP listening on http://127.0.0.1:3000");

    let mut router = Router::new();
    router.add_route(Method::Get, "/", |_| Response::text(200, "Hello from rustHTTP"));
    router.add_route(Method::Get, "/health", |_| Response::text(200, "ok"));
    router.add_route(Method::Get, "/users/:id", |req: &Request| {
        let id = req.params.get("id").cloned().unwrap_or_else(|| "unknown".to_string());
        Response::json(200, format!("{{\"id\":\"{id}\"}}"))
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let router = router.clone();
                thread::spawn(move || handle_connection(stream, router));
            }
            Err(e) => eprintln!("connection failed: {e}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: Router) {
    let mut buffer = [0; 4096];

    match stream.set_read_timeout(Some(Duration::from_secs(2))) {
        Ok(_) => {}
        Err(e) => eprintln!("set read timeout failed: {e}"),
    }

    let mut request = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(_) => match parse_request(&buffer) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("parse failed: {e}");
                let response = Response::text(400, "bad request");
                let _ = stream.write_all(&response.to_bytes());
                return;
            }
        },
        Err(e) => {
            eprintln!("read failed: {e}");
            return;
        }
    };

    let response = router.route(&mut request);
    if let Err(e) = stream.write_all(&response.to_bytes()) {
        eprintln!("write failed: {e}");
    }
}
