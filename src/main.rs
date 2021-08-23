use std::{fs, io, process};
use std::io::Write;

use gemini_hacking::{
    self as gh,
    response_codes as codes,
    url::Url, 
};

const GEMROOT: &str = "./gemroot";

macro_rules! header {
    ($code:expr, $meta:expr) => {
        {
            format!("{} {}\r\n", $code, $meta).into_bytes()
        }
    };
}

fn read_request() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn serve_file(path: &str) {
    let path = format!("{}{}", GEMROOT, path);
    
    let response = match fs::read(&path) {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => header!(codes::NOT_FOUND, ""),
            _ => header!(codes::CGI_ERROR, "io error while reading file"),
        },
        Ok(contents) => {
            // TODO: MIME TYPES
            let mime = "text/gemini";
            let header = header!(codes::SUCCESS, mime);
            let mut response = Vec::new();
            response.extend_from_slice(&header);
            response.extend_from_slice(&contents);
            response
        },
    };

    io::stdout().write(&response).unwrap_or_else(|_| {
        print!("{} {}\r\n", codes::CGI_ERROR, "unable to write to stdout");
        process::exit(1);
    });
}

fn main() {
    let request = read_request().unwrap_or_else(|_| {
        print!("{} {}\r\n", codes::CGI_ERROR, "unable to read request");
        process::exit(1);
    });

    let url = Url::parse(&request).unwrap_or_else(|_| {
        print!("{} {}\r\n", codes::CGI_ERROR, "unable to parse url");
        process::exit(1);
    });

    let mut path = match gh::ue::decode(&url.path()) {
        Ok(cow) => cow.to_string(),
        Err(_) => panic!("unable to decode url"),
    };

    if path.is_empty() || path == "/" {
        path = "/index.gmi".to_string();
    }

    serve_file(&path);
}
