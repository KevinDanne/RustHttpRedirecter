use std::env;

mod redirection;
mod webserver;

mod error;
use error::Error;

struct ProgramConfig {
    file_name: String,
}

fn parse_args(args: &[String]) -> Result<ProgramConfig, Error> {
    if args.len() > 1 {
        return Err(Error::ArgumentCount);
    }

    let file_name = args
        .get(0)
        .unwrap_or(&"default".to_string())
        .trim()
        .to_owned();

    Ok(ProgramConfig { file_name })
}

fn show_help() {
    println!("http-redirecter <routes file>");
    println!("Config files needs to be in the directory 'routes'");
    println!("If no config file is given. The file routes/default will be used.");
}

fn main() -> Result<(), Error> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let config = match parse_args(&args) {
        Ok(config) => config,
        Err(e) => {
            show_help();
            return Err(e);
        }
    };

    let redirections = redirection::get_redirections(&config.file_name)?;

    let listener = webserver::create_webserver(7878)?;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        if let Err(err) = webserver::get_url_from_tcpstream(&mut stream).and_then(|url| {
            match redirections
                .iter()
                .find(|redirection| redirection.from == url)
            {
                Some(redirection) => webserver::redirect_client(&mut stream, &redirection.to),
                None => {
                    let content = format!("No redirection found for url {}", url);
                    webserver::send_response(&mut stream, &content)
                }
            }
        }) {
            eprintln!("Error: {}", err);
        }
    }
    Ok(())
}
