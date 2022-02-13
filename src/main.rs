fn main() {
    let app = clap::app_from_crate!()
        .global_setting(clap::AppSettings::PropagateVersion)
        .global_setting(clap::AppSettings::UseLongFormatForHelpSubcommand)
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .mut_arg("help", |a| a.help("print help information"))
        .mut_arg("version", |a| a.help("print version information").short('v'))
        .subcommand(
            clap::App::new("blind")
                .about("predetermine guesses without seeing any hints")
        )
        .subcommand(
            clap::App::new("solve")
                .about("play a game of Wordle")
        );
    let matches = app.get_matches();
    println!("{matches:?}");
}
