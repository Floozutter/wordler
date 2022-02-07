fn main() {
    let app = clap::app_from_crate!()
        .global_setting(clap::AppSettings::PropagateVersion)
        .global_setting(clap::AppSettings::UseLongFormatForHelpSubcommand)
        .setting(clap::AppSettings::SubcommandRequiredElseHelp);
    let matches = app.get_matches();
    println!("{matches:?}");
}
