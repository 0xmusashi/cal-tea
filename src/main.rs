fn main() {
    if let Err(e) = cal_tea::get_args().and_then(cal_tea::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
