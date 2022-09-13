use std::{
    fs::read_to_string,
    io::{Read, Write},
    net::TcpListener,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use crate::responses::{Response, StatusLines};

mod paths {
    pub const ROOT: &'static str = "";
    pub const CREATE_ROOM: &'static str = "create";
}

mod methods {
    pub const POST: &'static str = "POST";
    pub const GET: &'static str = "GET";
}

mod content_types {
    pub const JSON: &'static str = "application/json";
    pub const HTML: &'static str = "text/html";
    pub const TEXT: &'static str = "text/plain";
}

mod responses {
    type StatusLine<'a> = &'a str;

    pub enum StatusLines {}

    impl StatusLines {
        pub const OK: StatusLine<'static> = "HTTP/1.1 200 OK";
        pub const NOT_FOUND: StatusLine<'static> = "HTTP/1.1 404 NOT FOUND";
    }
    pub struct Response<'a> {
        pub status_line: StatusLine<'a>,
        pub content: &'a str,
    }

    pub const PING_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::OK,
        content: "\"pong\"",
    };

    pub const NOT_FOUND_RESPONSE: Response<'static> = Response {
        status_line: StatusLines::NOT_FOUND,
        content: "\"404\"",
    };
}

fn get_header_from_request(request: &String) -> &str {
    let mut splits = request.split(" ");
    splits.nth(0).unwrap()
}

fn get_path_from_request(request: &String) -> &str {
    let mut splits = request.split(" ");
    splits.nth(1).unwrap()
}

fn get_data_from_request(request: &String) -> String {
    let mut response = "".to_string();
    let mut data_found = false;
    for line in request.lines() {
        if data_found {
            if response.len() == 0 {
                response = format!("{}", line)
            } else if !line.starts_with('\0') {
                response = format!("{}\n{}", response, line)
            }
        }
        if line.len() == 0 {
            data_found = true
        };
    }
    response.trim_matches(char::from(0)).to_string()
}
pub struct HttpServer {
    pub port: u16,
}

impl HttpServer {
    pub fn run(&mut self) {
        let counter = Arc::new(Mutex::new(0));

        for stream in TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .unwrap()
            .incoming()
        {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let mut stream = stream.unwrap();

                let mut buffer = [0; 20000];
                stream.read(&mut buffer).unwrap();

                let request = String::from_utf8(buffer.to_vec()).unwrap();
                let method = get_header_from_request(&request);
                let path = &get_path_from_request(&request)[1..];
                let args: Vec<&str> = path.split("/").collect();
                let path = args[0];
                let args = &args[1..];

                println!("{} {} {:?}", method, path, args);

                // let url: String;
                // let res_code: String;
                // let res_log: String;
                // let mut string_build = "could not run simulation\n".to_string();

                let mut content_type = content_types::TEXT;

                let index = read_to_string("index.html").unwrap();
                let room_name: String;
                let mut guard = counter.lock().unwrap();

                let response = match (method, path) {
                    (methods::GET, paths::ROOT) => {
                        content_type = content_types::HTML;
                        Response {
                            status_line: StatusLines::OK,
                            content: &index,
                        }
                    }
                    (methods::GET, paths::CREATE_ROOM) => {
                        *guard += 1;
                        room_name = format!("{:?}", *guard);
                        content_type = content_types::TEXT;
                        Response {
                            status_line: StatusLines::OK,
                            content: &room_name,
                        }
                    }
                    _ => responses::NOT_FOUND_RESPONSE,
                };
                let response_string = format!(
                    "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET\r\ncharset=UTF-8\r\n\r\n{}",
                    response.status_line,
                    response.content.len(),
                    content_type,
                    response.content
                );

                println!("{}", response_string);

                stream.write(response_string.as_bytes()).unwrap();
                stream.flush().unwrap();
            });
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut server = HttpServer { port: 8088 };
    server.run();
}
