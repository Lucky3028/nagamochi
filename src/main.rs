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
        let path = PathBuf::from("/sys/class/power_supply/").join("BAT1/capacity");
        let capa = nagamochi::read_capacity(path).unwrap();
        config.triggers.iter().filter(|trigger| trigger.is_fired(capa)).for_each(|trigger| {
            if let Err(e) = Notification::new().summary("nagamochi").body(&trigger.message).show() {
                eprintln!("Error: Failed to send a notification: {}", e);
            }
        });
        std::thread::sleep(time::Duration::from_secs(config.check_interval));
    }
}

fn find_config() -> anyhow::Result<Config> {
    env::var_os("HOME")
        .map(find_config_path)
        .map_or(anyhow::anyhow!("config not found"), |path| {
            Config::from_file(&path.unwrap())
        })
}

fn find_config_path(home_dir: std::ffi::OsString) -> Option<PathBuf> {
    vec![
        format!("{:?}/.config/nagamochi/nagamochi.yml", home_dir),
        format!("{:?}/nagamochi.yml", home_dir),
        "./nagamochi.yml".to_string(),
    ]
    .iter()
    .map(PathBuf::from)
    .find(|path| path.exists())
}
