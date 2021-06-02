
use exprs::{App, Config};

fn main() {
    let config = Config::new(None, None);
    let app = App::new(config);
    app.start();
}
