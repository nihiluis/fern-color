extern crate log;
extern crate simplecolor;

use simplecolor::{Color, Colorize};

pub trait ColoredLogLevel {
    fn colored(&self, color: Color) -> String;
}

#[derive(Copy, Clone)]
pub struct ColoredLogLevelConfig {
    pub trace: Color,
    pub error: Color,
    pub warn: Color,
    pub debug: Color,
    pub info: Color,
}

impl ColoredLogLevelConfig {
    pub fn new(trace: Color, error: Color, warn: Color, debug: Color, info: Color) -> Self {
        ColoredLogLevelConfig {
            trace: trace,
            error: error,
            warn: warn,
            debug: debug,
            info: info,
        }
    }

    pub fn default() -> Self {
        ColoredLogLevelConfig {
            trace: Color::White,
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::White,
            info: Color::White,
        }
    }

    pub fn color(&self, level: log::Level) -> String {
        level.colored(self.get_color(&level))
    }

    fn get_color(&self, level: &log::Level) -> Color {
        match *level {
            log::Level::Error => self.error,
            log::Level::Warn => self.warn,
            log::Level::Info => self.info,
            log::Level::Debug => self.debug,
            log::Level::Trace => self.trace,
        }
    }
}

impl ColoredLogLevel for log::Level {
    fn colored(&self, color: Color) -> String {
        let s = format!("{:?}", self);

        s.color(color)
    }
}