use notify_rust::Notification;
use seahorse::{App, Context};

use std::{env, path::PathBuf, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("nagamochi [args]")
        .action(default_action);

    app.run(args);
}

fn default_action(_: &Context) {
    loop {
        let p = PathBuf::from("/sys/class/power_supply/").join("BAT1/capacity");
        let ss = nagamochi::read_capacity(p).unwrap();
        Notification::new().summary(&format!("{}", ss)).show();
        std::thread::sleep(time::Duration::from_secs(3));
    }
}
