use std::{env, error::Error};

mod redirection;
mod webserver;

struct ProgramConfig {
    file_name: String,
}

fn parse_args(args: &[String]) -> Option<ProgramConfig> {
    if args.len() > 1 {
        return None;
    }

    let file_name = args
        .get(0)
        .unwrap_or(&"default".to_string())
        .trim()
        .to_owned();

    Some(ProgramConfig { file_name })
}

fn show_help() {
    println!("http-redirecter <routes file>");
    println!("Config files needs to be in the directory 'routes'");
    println!("If no config file is given. The file routes/default will be used.");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = if let Some(config) = parse_args(&args) {
        config
    } else {
        show_help();
        return Ok(());
    };

    let redirections = match redirection::get_redirections(&config.file_name) {
        Ok(val) => val,
        Err(err) => return Err(Box::new(err)),
    };

    let listener = webserver::create_webserver("7878")?;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let url = match webserver::get_url_from_tcpstream(&mut stream) {
            Some(val) => val,
            None => return Err(Box::new(webserver::InvalidRequest)),
        };

        if let Some(redirection) = redirections
            .iter()
            .find(|redirection| redirection.from == url)
        {
            if let Err(err) = webserver::redirect_client(&mut stream, &redirection.to) {
                return Err(Box::new(err));
            }
        } else {
            let content = "No redirection found for url ".to_string() + &url;
            if let Err(err) = webserver::send_response(&mut stream, &content) {
                return Err(Box::new(err));
            }
        }
    }
    Ok(())
}
