fn main() {
    if let Err(e) = rust_cat::get_args().and_then(rust_cat::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
