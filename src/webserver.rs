use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    fmt::{self, Display}
};

use crate::error::{self, Error};

const HTTP_VERSION: &str = "HTTP/1.1";
enum HTTPStatusCode {
    C200,
    C301
}

impl Display for HTTPStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::C200 => "200 OK",
            Self::C301 => "301 Moved Permanently"
        };

        write!(f, "{} {}", HTTP_VERSION, message)
    }
}

pub fn create_webserver(port: u16) -> error::Result<TcpListener> {
    Ok(TcpListener::bind(format!("0.0.0.0:{}", port))?)
}

pub fn get_url_from_tcpstream(stream: &mut TcpStream) -> error::Result<String> {
    let buf_reader = BufReader::new(stream);
    Ok(buf_reader
        .lines()
        .filter_map(|result| result.ok())
        .find(|line| line.contains("GET"))
        .ok_or(Error::InvalidRequest)?
        .split(' ')
        .nth(1)
        .ok_or(Error::InvalidRequest)?
        .to_owned())
}

pub fn send_response(stream: &mut TcpStream, content: &str) -> error::Result<()> {
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        HTTPStatusCode::C200, length, content
    );
    Ok(stream.write_all(response.as_bytes())?)
}

pub fn redirect_client(stream: &mut TcpStream, to: &str) -> error::Result<()> {
    let response = format!("{}\r\nLocation: {}", HTTPStatusCode::C301, to);

    Ok(stream.write_all(response.as_bytes())?)
}
