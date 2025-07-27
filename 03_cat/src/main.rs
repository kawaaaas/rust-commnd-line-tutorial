fn main() {
    if let Err(e) = cat_03::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
