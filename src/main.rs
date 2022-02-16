fn main() {
    let arg_words = clap::Arg::new("words")
        .long("words")
        .short('w')
        .takes_value(true)
        .value_name("FILE")
        .help("newline-delimited dictionary of valid solutions")
        .required(true);
    let app = clap::app_from_crate!()
        .global_setting(clap::AppSettings::PropagateVersion)
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .setting(clap::AppSettings::DisableHelpSubcommand)
        .mut_arg("help", |a| a.help("print help information"))
        .mut_arg("version", |a| a.help("print version information").short('v'))
        .subcommand(
            clap::App::new("blind")
                .about("predetermine guesses without seeing any hints")
                .arg(arg_words.clone())
        )
        .subcommand(
            clap::App::new("every")
                .about("play every possible game of Wordle")
                .arg(arg_words.clone())
        )
        .subcommand(
            clap::App::new("solve")
                .about("play a game of Wordle")
                .arg(arg_words.clone())
        );
    let matches = app.get_matches();
    println!("{matches:?}");
}
