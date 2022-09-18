use std::env;

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

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;
    let args: Vec<String> = env::args().skip(1).collect();
    let config = if let Some(config) = parse_args(&args) {
        config
    } else {
        show_help();
        return Ok(());
    };

    let redirections = redirection::get_redirections(&config.file_name)?;

    let listener = webserver::create_webserver("7878")?;

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let url =
            webserver::get_url_from_tcpstream(&mut stream).ok_or(webserver::InvalidRequest)?;

        if let Some(redirection) = redirections
            .iter()
            .find(|redirection| redirection.from == url)
        {
            webserver::redirect_client(&mut stream, &redirection.to)?
        } else {
            let content = "No redirection found for url ".to_string() + &url;
            webserver::send_response(&mut stream, &content)?;
        }
    }
    Ok(())
}
