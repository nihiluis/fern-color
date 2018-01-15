extern crate fern;
extern crate fern_simplecolor;
extern crate simplecolor;
#[macro_use]
extern crate log;

use simplecolor::Color;
use fern_simplecolor::{ColoredLogLevel, ColoredLogLevelConfig};

fn main() {
    let mut config = ColoredLogLevelConfig::default();
    config.debug = Color::Magenta;

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                config.color(record.level()),
                message
            ))
        })
        .apply()
        .unwrap();

    error!("hi");
    debug!("sup");
    warn!("oh");
}