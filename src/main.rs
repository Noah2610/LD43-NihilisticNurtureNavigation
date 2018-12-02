extern crate LD43;

fn main() {
  if let Err(err) = LD43::run() {
    eprintln!("Error: {:#?}", err);
    std::process::exit(1);
  }
}
