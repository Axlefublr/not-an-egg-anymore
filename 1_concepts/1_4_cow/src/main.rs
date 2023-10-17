use clap::Parser;
use std::borrow::Cow;
use std::env;

const DEFAULT_PATH: &str = "/etc/app/app.conf";
const ENV_CONF: &str = "APP_CONF";

fn main() {
    let conf: Cow<'_, str> = {
        if let Some(flag) = Args::parse().conf {
            Cow::Owned(flag)
        } else if matches!(env::var(ENV_CONF), Ok(env_conf) if !env_conf.is_empty()) {
            Cow::Owned(env::var(ENV_CONF).unwrap())
        } else {
            Cow::Borrowed(DEFAULT_PATH)
        }
    };
    println!("{conf}");
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    conf: Option<String>,
}
