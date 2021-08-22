use std::{fs, io, process};
use std::io::Write;

use gemini_hacking::{
    self as gh,
    url::Url, 
};

const GEMROOT: &str = "./gemroot";

fn cgi_error(desc: &str) -> ! {
    print!("{} {}\r\n", gh::response_codes::CGI_ERROR, desc);
    process::exit(1)
}

fn read_request() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn serve_file(path: &str) {
    let path = format!("{}{}", GEMROOT, path);
    
    let content = match fs::read(&path) {
        Ok(v) => v,
        Err(_) => {
            print!("{}\r\n", gh::response_codes::NOT_FOUND);
            return;
        },
    };

    // TODO: MIME TYPES
    let mime = "text/gemini";
    let header = format!("{} {}\r\n", gh::response_codes::SUCCESS, mime);

    let mut response = Vec::new();
    response.extend_from_slice(header.as_bytes());
    response.extend_from_slice(&contents);

    io::stdout().write(&response).unwrap_or_else(|_| {
        cgi_error("unable to write response");
    });
}

fn main() {
    let request = read_request().unwrap_or_else(|_| {
        cgi_error("unable to read incoming request");
    });

    let url = Url::parse(&request).unwrap_or_else(|_| {
        cgi_error("unable to parse provided url");
    });

    let mut path = match gh::ue::decode(&url.path()) {
        Ok(cow) => cow.to_string(),
        Err(_) => panic!("unable to decode provided url"),
    };

    if path.is_empty() || path == "/" {
        path = "/index.gmi".to_string();
    }

    serve_file(&path);
}
