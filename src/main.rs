fn main() {
    let app = clap::app_from_crate!();
    let matches = app.get_matches();
    println!("{matches:?}");
}
