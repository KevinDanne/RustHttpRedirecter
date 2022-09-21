const HTTP_STATUS_200: &str = "HTTP/1.1 200 OK";
const HTTP_STATUS_301: &str = "HTTP/1.1 301 Moved Permanently";

use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::Error;

pub fn create_webserver(port: u16) -> io::Result<TcpListener> {
    TcpListener::bind(format!("127.0.0.1:{}", port))
}

pub fn get_url_from_tcpstream(stream: &mut TcpStream) -> Result<String, Error> {
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

pub fn send_response(stream: &mut TcpStream, content: &str) -> io::Result<()> {
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        HTTP_STATUS_200, length, content
    );
    stream.write_all(response.as_bytes())
}

pub fn redirect_client(stream: &mut TcpStream, to: &str) -> io::Result<()> {
    let response = format!("{}\r\nLocation: {}", HTTP_STATUS_301, to);

    stream.write_all(response.as_bytes())
}
