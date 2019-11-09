use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};

fn main() -> goku::GokuResult<()> {
    let mut app = build_app();
    match app.clone().get_matches().subcommand() {
        ("kamehameha", Some(_)) => goku::attack()?,
        ("help", Some(_)) | _ => app.print_help()?,
    }
    Ok(())
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
                .arg(Arg::with_name("url").help("Target url").required(true)),
        )
}
