use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
    SubCommand,
};
use url::{Host, Position, Url};

fn main() -> goku::GokuResult<()> {
    let mut app = build_app();
    match app.clone().get_matches().subcommand() {
        ("kamehameha", Some(matches)) => cmd_attack(matches)?,
        ("help", Some(_)) | _ => app.print_help()?,
    }
    Ok(())
}

fn cmd_attack(matches: &ArgMatches) -> goku::GokuResult<()> {
    let concurrency = matches
        .value_of("concurrency")
        .unwrap_or_default()
        .parse()?;

    let requests = matches.value_of("requests").unwrap_or_default().parse()?;
    let url = Url::parse(matches.value_of("url").unwrap_or_default())?;

    let url = match url.host_str() {
        Some(url) => url,
        None => {
            println!("url parse error");
            return Ok(());
        }
    };

    goku::attack(concurrency, requests, url)
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(SubCommand::with_name("help").alias("h").about("Show help"))
        .subcommand(
            SubCommand::with_name("kamehameha")
                .visible_alias("attack")
                .about("Run load test")
                .arg(Arg::with_name("url").help("Target url").required(true))
                .arg(
                    Arg::with_name("requests")
                        .help("Number of requests to perform")
                        .short("n")
                        .long("requests")
                        .value_name("requests")
                        .required(true),
                )
                .arg(
                    Arg::with_name("concurrency")
                        .help("Number of multiple requests to make at a time")
                        .short("c")
                        .long("concurrency")
                        .value_name("concurrency")
                        .required(true),
                ),
        )
}
