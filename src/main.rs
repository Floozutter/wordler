mod blind;
mod every;
mod solve;

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
    let arg_modus = clap::Arg::new("modus")
        .long("modus")
        .short('m')
        .takes_value(true)
        .value_name("NAME")
        .help("objective function to use to evaluate guesses")
        .possible_values(["minimax"])
        .default_value("minimax");
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
                .arg(arg_modus.clone())
                .arg(
                    clap::Arg::new("count")
                        .long("count")
                        .short('c')
                        .takes_value(true)
                        .value_name("N")
                        .help("number of predetermined guesses")
                        .required(true)
                )
                .arg(
                    clap::Arg::new("given")
                        .long("given")
                        .short('g')
                        .takes_value(true)
                        .multiple_values(true)
                        .value_name("WORD")
                        .help("words that must appear in each group of guesses")
                )
                .arg(
                    clap::Arg::new("tries")
                        .long("tries")
                        .short('t')
                        .takes_value(true)
                        .value_name("N")
                        .help("number of groups of guesses to generate")
                        .default_value("1")
                )
        )
        .subcommand(
            clap::App::new("every")
                .about("play every possible game of Wordle")
                .arg(arg_words.clone())
                .arg(arg_extra.clone())
                .arg(arg_modus.clone())
        )
        .subcommand(
            clap::App::new("solve")
                .about("play a game of Wordle")
                .arg(arg_words.clone())
                .arg(arg_extra.clone())
                .arg(arg_modus.clone())
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
    match matches.subcommand() {
        Some(("blind", _)) => {
            println!("hello blind");
        },
        _ => todo!(),
    }
}
