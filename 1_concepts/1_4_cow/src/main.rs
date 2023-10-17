use clap::Parser;
use std::borrow::Cow;
use std::env;

const DEFAULT_PATH: &str = "/etc/app/app.conf";
const ENV_CONF: &str = "APP_CONF";

fn main() {
    let conf: Cow<'_, str> = {
        match Args::parse().conf {
            Some(flag) if !flag.is_empty() => Cow::Owned(flag),
            _ => match env::var(ENV_CONF) {
                Ok(env_conf) if !env_conf.is_empty() => Cow::Owned(env_conf),
                _ => Cow::Borrowed(DEFAULT_PATH),
            },
        }
    };
    println!("{conf}");
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    conf: Option<String>,
}
