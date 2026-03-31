mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn execute() {
    println!("{} {}", env!("CARGO_PKG_NAME"), built_info::PKG_VERSION,);
    println!(
        "os/arch: {}/{}",
        std::env::consts::OS,
        std::env::consts::ARCH,
    );
    println!("rustc:   {}", built_info::RUSTC_VERSION,);
}
