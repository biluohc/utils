use flexi_logger::{Logger, Cleanup, FlexiLoggerError};
use log::Record;
use chrono;

use std::{self, io};
use std::sync::atomic::{AtomicUsize, Ordering};

use self::colors::{Color, ColoredLevelConfig};


static mut COLORS: Option<ColoredLevelConfig> = None;

pub fn log_format(w: &mut io::Write, record: &Record) -> Result<(), io::Error> {
    let level = if let Some(cos) = unsafe { COLORS.as_ref() } {
        format!("{}", cos.color(record.level()))
    } else {
        format!("{:5}", record.level())
    };

    write!(
        w,
        "{date} {level} [{thread_name}] ({file_name}:{line}) [{target}] -- {msg}",
        date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
        level = level,
        thread_name = current_thread_name(),
        file_name = record.file().unwrap_or("*"),
        line = record.line().unwrap_or(0),
        target = record.target(),
        msg = record.args()
    )
}

pub fn set(warn0_info1_debug2_trace3: u64, nocolor: bool) -> Result<(), FlexiLoggerError> {
    if !nocolor {
        let colors = ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::Green)
            .debug(Color::White)
            .trace(Color::White);
        unsafe { COLORS = Some(colors) };
    }
    
    let mut filiter = match warn0_info1_debug2_trace3 {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _3_or_more => "trace",
    }.to_owned();

    vec![
        "mio",
        "tokio_reactor",
        "tokio_core",
        "tokio",
        "tokio_threadpool",
        "hyper",
        "want",
        "tokio_io",
    ].into_iter().for_each(|c|filiter.push_str(&format!(", {}=info", c)));

    Logger::with_env_or_str(filiter)
            // .log_to_file()
            .suffix("#a495")
            .o_rotate(Some((1024*1024*1024, Cleanup::Never)))
            .o_timestamp(true)
            .directory("log")
            .format(log_format)
            .print_message()
            .start()
            .map(|_|())
}

pub struct ThreadId(u64);

fn current_thread_name() -> &'static str {
    thread_local!(static TNAME: String = {
        let thread = std::thread::current();
        format!("{}_{}", unsafe { std::mem::transmute::<_, ThreadId>(thread.id()).0 }, thread.name()
        .map(|s| s.to_owned())
        .unwrap_or_else(||format!("<uname-{:2}>", uname_count())))
    });

    TNAME.with(|tname| unsafe { std::mem::transmute::<&str, &'static str>(tname.as_str()) })
}

fn uname_count() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    COUNT.fetch_add(1, Ordering::SeqCst)
}

pub mod colors {
    pub use colored::Color;
    use log::Level;
    use std::fmt;

    pub trait ColoredLogLevel {
        fn colored(&self, color: Color) -> WithFgColor<Level>;
    }

    pub struct WithFgColor<T>
    where
        T: fmt::Display,
    {
        text: T,
        color: Color,
    }

    impl<T> fmt::Display for WithFgColor<T>
    where
        T: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\x1B[{}m{:5}\x1B[0m", self.color.to_fg_str(), self.text)
        }
    }

    #[derive(Copy, Clone)]
    pub struct ColoredLevelConfig {
        pub error: Color,
        pub warn: Color,
        pub info: Color,
        pub debug: Color,
        pub trace: Color,
    }

    impl ColoredLevelConfig {
        #[inline]
        pub fn new() -> Self {
            Self::default()
        }
        pub fn error(mut self, error: Color) -> Self {
            self.error = error;
            self
        }
        pub fn warn(mut self, warn: Color) -> Self {
            self.warn = warn;
            self
        }
        pub fn info(mut self, info: Color) -> Self {
            self.info = info;
            self
        }
        pub fn debug(mut self, debug: Color) -> Self {
            self.debug = debug;
            self
        }
        pub fn trace(mut self, trace: Color) -> Self {
            self.trace = trace;
            self
        }
        pub fn color(&self, level: Level) -> WithFgColor<Level> {
            level.colored(self.get_color(&level))
        }
        pub fn get_color(&self, level: &Level) -> Color {
            match *level {
                Level::Error => self.error,
                Level::Warn => self.warn,
                Level::Info => self.info,
                Level::Debug => self.debug,
                Level::Trace => self.trace,
            }
        }
    }
    impl Default for ColoredLevelConfig {
        fn default() -> Self {
            ColoredLevelConfig {
                error: Color::Red,
                warn: Color::Yellow,
                debug: Color::White,
                info: Color::White,
                trace: Color::White,
            }
        }
    }
    impl ColoredLogLevel for Level {
        fn colored(&self, color: Color) -> WithFgColor<Level> {
            WithFgColor {
                text: *self,
                color: color,
            }
        }
    }

}
