use notify_rust::Notification;
use seahorse::{App, Context};
use std::{env, path::PathBuf, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("nagamochi")
        .action(default_action);

    app.run(args);
}

fn default_action(_: &Context) {
    loop {
        let path = PathBuf::from("/sys/class/power_supply/").join("BAT1/capacity");
        let capa = nagamochi::read_capacity(path).unwrap();
        Notification::new().summary(&format!("{}", capa)).show();
        std::thread::sleep(time::Duration::from_secs(3));
    }
}
