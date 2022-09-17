use std::{
    error::Error,
    fmt,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct InvalidRequest;

impl fmt::Display for InvalidRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request is invalid")
    }
}

impl Error for InvalidRequest {}

pub fn create_webserver(port: &str) -> io::Result<TcpListener> {
    return TcpListener::bind("127.0.0.1:".to_string() + port);
}

pub fn get_url_from_tcpstream(stream: &mut TcpStream) -> Option<String> {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let url_line = http_request.iter().find(|line| line.contains("GET"))?;
    let url = url_line.split(" ").skip(1).next()?;
    Some(url.to_string())
}

pub fn send_response(stream: &mut TcpStream, content: &str) -> io::Result<()> {
    let status_line = "HTTP/1.1 200 OK";
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, content
    );
    stream.write_all(response.as_bytes())
}

pub fn redirect_client(stream: &mut TcpStream, to: &str) -> io::Result<()> {
    let status_line = "HTTP/1.1 301 Moved Permanently";
    let response = format!("{}\r\nLocation: {}", status_line, to);

    stream.write_all(response.as_bytes())
}
