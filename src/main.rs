use std::{env, fs, io};
use std::path::Path;
use std::io::Write;

use gemini_hacking::{
    self as gh,
    response_codes as codes,
    url::Url, 
};

const GEMROOT: &str = "./gemroot";

macro_rules! exit_error {
    ($code:expr, $meta:expr) => {
        {
            print!("{} {}\r\n", $code, $meta);
            std::process::exit(1);
        }
    };
}

fn read_request() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn serve_file(gemroot: &str, path: &str) {
    let path = format!("{}{}", gemroot, path);
    
    let contents = fs::read(&path).unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => exit_error!(codes::NOT_FOUND, ""),
            _ => exit_error!(codes::CGI_ERROR, "io error while reading file"),
        }
    });

    // TODO: MIME TYPES
    let mime = "text/gemini";
    let header = format!("{} {}\r\n", codes::SUCCESS, mime);

    let mut response = Vec::new();
    response.extend_from_slice(&header.as_bytes());
    response.extend_from_slice(&contents);

    io::stdout().write(&response).unwrap_or_else(|_| {
        exit_error!(codes::CGI_ERROR, "unable to write to stdout");
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        exit_error!(codes::CGI_ERROR, "usage: ./gemini-inetd <gemroot path>");
    }

    let gemroot = &args[1];

    if ! Path::new(gemroot).exists() {
        exit_error!(codes::CGI_ERROR, "invalid gemroot path");
    }

    let request = read_request().unwrap_or_else(|_| {
        exit_error!(codes::CGI_ERROR, "unable to read request");
    });

    let url = Url::parse(&request).unwrap_or_else(|_| {
        exit_error!(codes::CGI_ERROR, "unable to parse url");
    });

    let mut path = match gh::ue::decode(&url.path()) {
        Ok(cow) => cow.to_string(),
        Err(_) => exit_error!(codes::CGI_ERROR, "unable to decode url"),
    };

    if path.is_empty() || path == "/" {
        path = "/index.gmi".to_string();
    }

    serve_file(gemroot, &path);
}
