use clap::{
    crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand,
};
use env_logger as logger;
use url::Url;

fn main() -> goku::GokuResult<()> {
    std::env::set_var("RUST_LOG", "debug");
    logger::init();

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

    let host = match url.host_str() {
        Some(host) => host,
        None => {
            // TODO スキーマがなくてもパースできるようにしたい
            println!("url parse error");
            return Ok(());
        }
    };

    let port = url.port().unwrap_or(80);

    let report = goku::attack(concurrency, requests, host, port)?;

    let output_format = matches.value_of("output");
    if output_format == Some("json") {
        let j = serde_json::to_string(&report).unwrap_or_default();
        println!("{}", j);
    } else {
        if matches.is_present("verbose") {
            report.errors().iter().for_each(|e| println!("{}", e));
        }

        println!("{}", report);
    }

    Ok(())
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("help")
                .alias("h")
                .about("Show help")
                .setting(AppSettings::ColoredHelp),
        )
        .subcommand(
            SubCommand::with_name("kamehameha")
                .setting(AppSettings::ColoredHelp)
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
                )
                .arg(
                    Arg::with_name("output")
                        .help("Output format.")
                        .short("o")
                        .long("output")
                        .possible_values(&["text", "json"])
                        .value_name("output"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .help("Output all message")
                        .short("v")
                        .long("verbose"),
                ),
        )
}
