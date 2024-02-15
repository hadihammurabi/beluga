use beluga_app::app;

pub fn main() {
    let result = app::launch();
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
