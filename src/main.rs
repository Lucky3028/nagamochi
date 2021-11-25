use nagamochi::Config;
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
        let config = nagamochi::find_config().unwrap_or_else(|e| {
            eprintln!("{:?}", e);
            Config::default()
        });
        // TODO: BAT0の場合などにも対応する
        let path = PathBuf::from("/sys/class/power_supply/").join("BAT1/capacity");
        let capa = nagamochi::read_capacity(path).unwrap();
        config
            .triggers
            .iter()
            .filter(|trigger| trigger.is_fired(capa))
            .filter(|trigger| {
                let is_ac =
                    nagamochi::is_ac_connected(PathBuf::from("/sys/class/power_supply/ACAD"))
                        .unwrap_or_else(|e| {
                            eprintln!("{:?}", e);
                            false
                        });
                trigger.suppressors.iter().any(|sup| !sup.is_enabled(is_ac))
            })
            .for_each(|trigger| {
                if let Err(e) = Notification::new()
                    .summary("nagamochi")
                    .body(&trigger.message)
                    .show()
                {
                    eprintln!("Error: Failed to send a notification: {}", e);
                }
                let _ = nagamochi::play_sound(&PathBuf::from("/usr/share/sounds/purple/receive.wav"));
            });
        std::thread::sleep(time::Duration::from_secs(config.check_interval));
    }
}
