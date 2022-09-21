use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn create_webserver(port: u16) -> io::Result<TcpListener> {
    TcpListener::bind(format!("127.0.0.1:{}", port))
}

pub fn get_url_from_tcpstream(stream: &mut TcpStream) -> Option<String> {
    let buf_reader = BufReader::new(stream);
    let url_line = buf_reader
        .lines()
        .filter_map(|result| result.ok())
        .find(|line| line.contains("GET"))?;

    let url = url_line.split(" ").nth(2)?;
    Some(url.to_owned())
}

pub fn send_response(stream: &mut TcpStream, content: &str) -> io::Result<()> {
    const status_line: &str = "HTTP/1.1 200 OK";
    let length = content.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, content
    );
    stream.write_all(response.as_bytes())
}

pub fn redirect_client(stream: &mut TcpStream, to: &str) -> io::Result<()> {
    const status_line: &str = "HTTP/1.1 301 Moved Permanently";
    let response = format!("{}\r\nLocation: {}", status_line, to);

    stream.write_all(response.as_bytes())
}
