fn main() {
    let app = clap::App::new("wordler")
        .version(clap::crate_version!());
    let matches = app.get_matches();
    println!("{matches:?}");
}
