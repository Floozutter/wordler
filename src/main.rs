fn main() {
    let arg_words = clap::Arg::new("words")
        .long("words")
        .short('w')
        .takes_value(true)
        .value_name("FILE")
        .help("newline-delimited lexicon of valid solutions")
        .required(true);
    let arg_extra = clap::Arg::new("extra")
        .long("extra")
        .short('e')
        .takes_value(true)
        .value_name("FILE")
        .help("newline-delimited lexicon of guessable nonsolutions");
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
                .arg(arg_extra.clone())
        )
        .subcommand(
            clap::App::new("every")
                .about("play every possible game of Wordle")
                .arg(arg_words.clone())
                .arg(arg_extra.clone())
        )
        .subcommand(
            clap::App::new("solve")
                .about("play a game of Wordle")
                .arg(arg_words.clone())
                .arg(arg_extra.clone())
                .arg(
                    clap::Arg::new("known")
                        .long("known")
                        .short('k')
                        .takes_value(true)
                        .value_name("WORD")
                        .help("solution to the game (avoids manual hinting)")
                )
                .arg(
                    clap::Arg::new("query-quash")
                        .long("query-quash")
                        .short('q')
                        .help("prompt to override suggested guess")
                )
                .arg(
                    clap::Arg::new("query-spill")
                        .long("query-spill")
                        .short('s')
                        .help("prompt to reveal all remaining solutions")
                )
        );
    let matches = app.get_matches();
    println!("{matches:?}");
}
