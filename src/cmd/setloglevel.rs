extern crate log;
use env_logger::Env;
use crate::cmd;


pub fn setloglevel(c: &cmd::Cli) {
    let env = Env::default().filter_or("BUND_LOG_LEVEL", "error").write_style_or("NRAPM_LOG_STYLE", "always");
    match c.debug {
        0 => {
            env_logger::init_from_env(env);
        }
        1 => {
            let env = Env::default().filter_or("BUND_LOG_LEVEL", "info").write_style_or("NRAPM_LOG_STYLE", "always");
            env_logger::init_from_env(env);
        }
        2 => {
            let env = Env::default().filter_or("BUND_LOG_LEVEL", "debug").write_style_or("NRAPM_LOG_STYLE", "always");
            env_logger::init_from_env(env);
        }
        _ => {
            let env = Env::default().filter_or("BUND_LOG_LEVEL", "trace").write_style_or("NRAPM_LOG_STYLE", "always");
            env_logger::init_from_env(env);
        }
    }
    log::trace!("setloglevel::setloglevel() reached")
}
