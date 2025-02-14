extern crate log;
use std::env;
use time_graph;
use crate::stdlib::BUND;
use crate::stdlib::{init_stdlib};
use lazy_static::lazy_static;
use std::sync::Mutex;
use clap::{Parser, Subcommand, Args};

pub mod setloglevel;

pub mod bund_version;
pub mod bund_shell;
pub mod bund_eval;
pub mod bund_script;
pub mod bund_wscript;
pub mod bund_load;

pub mod bund_display_banner;
pub mod bund_bootstrap;
pub mod bund_bus;

lazy_static! {
    pub static ref CLI: Mutex<Cli> = {
        let e: Mutex<Cli> = Mutex::new(Cli::parse());
        e
    };
}

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    init_stdlib(&cli);
    let init_cli = CLI.lock().unwrap();
    log::debug!("Initialize global CLI");
    drop(init_cli);
    log::debug!("BUND interpterer initialized ...");
    match &cli.stack {
        Some(initial_stack) => {
            log::debug!("Setting initial stack to {}", &initial_stack);
            let mut bc = BUND.lock().unwrap();
            bc.vm.stack.ensure_stack(initial_stack.to_string());
            drop(bc);
        }
        None => {
            log::debug!("No initial stacks selected");
        }
    }
    match &cli.bootstrap {
        Some(_) => {
            bund_bootstrap::run(&cli);
        }
        None => {
            log::debug!("No BUND bootstrap specified");
        }
    }
    if cli.profile {
        log::debug!("Enable BUND profiler");
        time_graph::enable_data_collection(true);
    }
    bund_bus::bund_bus_init(&cli);
    match &cli.command {
        Commands::Script(script) => {
            bund_script::run(&cli, &script);
        }
        Commands::Eval(eval) => {
            bund_eval::run(&cli, &eval);
        }
        Commands::Shell(shell) => {
            bund_shell::run(&cli, &shell);
        }
        Commands::Load(load) => {
            bund_load::run(&cli, &load);
        }
        Commands::Wscript(wscript) => {
            bund_wscript::run(&cli, &wscript);
        }
        Commands::Version(_) => {
            bund_version::run(&cli);
        }
    }
    if cli.profile {
        log::debug!("Generating BUND profiler report");
        let graph = time_graph::get_full_graph();
        println!("{}", graph.as_table());
    }
}

#[derive(Parser, Clone)]
#[clap(name = "bund")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = "BUND CLI", long_about = "Interpreter and CLI for a Virtual Machine for BUND programming language")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Run BUND code inside debugger")]
    pub debugger: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Drop to DEBUG shell if error occurs")]
    pub debug_shell: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Run BUND interpreter as a node in distributed environment")]
    pub distributed: bool,

    #[clap(help="Filename for an internal database", long)]
    pub internal_db: Option<String>,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Execute internal profiler")]
    pub profile: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable colors in output")]
    pub nocolor: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable bund.eval group of functions")]
    pub noeval: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable I/O group of functions")]
    pub noio: bool,

    #[clap(help="Set the initial stack name", long)]
    pub stack: Option<String>,

    #[clap(short, long, value_delimiter = ' ', num_args = 0..)]
    pub bootstrap: Option<Vec<String>>,

    #[clap(flatten, help="BUND BIS configuration")]
    bus: DistributedArgGroup,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Script(Script),
    Eval(Eval),
    Shell(Shell),
    Load(Load),
    Wscript(Wscript),
    Version(Version),
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = true)]
pub struct DistributedArgGroup {
    #[clap(help="BUS receiving address", long, default_value_t = String::from(env::var("BUND_BUS_RECV_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
    pub bus_recv_connect: String,

    #[clap(help="BUS listen address", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub bus_recv_listen: String,

    #[clap(help="BUS topic to subscribe", long )]
    pub bus_recv_key: Vec<String>,

    #[clap(help="BUS sending address", long, default_value_t = String::from(env::var("BUND_BUS_SEND_ADDRESS").unwrap_or("tcp/127.0.0.1:7447".to_string())))]
    pub bus_send_connect: String,

    #[clap(help="BUS listen address", long, default_value_t = String::from_utf8(vec![]).unwrap())]
    pub bus_send_listen: String,

    #[clap(help="BUS topic to subscribe", long)]
    pub bus_send_key: Vec<String>,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable multicast discovery of ZENOH bus")]
    pub bus_disable_multicast_scout: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Configure CONNECT mode for ZENOH bus")]
    pub bus_set_connect_mode: bool,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Execute BUND script")]
pub struct Script {
    #[clap(flatten)]
    source: ScriptSrcArgGroup,

    #[clap(flatten)]
    ai: ScriptAiArgGroup,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = false)]
pub struct ScriptSrcArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Take BUND script from STDIN")]
    pub stdin: bool,

    #[clap(help="Take BUND script from file (full path required)", long)]
    pub file: Option<String>,

    #[clap(help="Take BUND script from url", long)]
    pub url: Option<String>,

    #[clap(help="Take BUND script from CLI argument", long)]
    pub eval: Option<String>,

}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = true)]
pub struct ScriptAiArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Pre-load languages for linguistic classifier")]
    pub ai_preload_languages: bool,

}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct WscriptSrcArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Add scripts to the WORLD file")]
    pub add: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Remove scripts from the WORLD file")]
    pub remove: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="List scripts in the WORLD file")]
    pub list: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Export script from the WORLD file")]
    pub export: bool,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Evaluate the BUND code snippet")]
pub struct Eval {
    #[clap(flatten)]
    source: EvalSrcArgGroup,
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = false)]
pub struct EvalSrcArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Take BUND snippet from STDIN")]
    pub stdin: bool,

    #[clap(help="Take BUND snippet from CLI argument", long)]
    pub eval: Option<String>,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Run the BUND REPL shell")]
pub struct Shell {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Run BUND code in the shell as script")]
    pub as_script: bool,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Load WORLD file and execute bootstrap scripts in this file")]
pub struct Load {
    #[clap(help="Path to the WORLD file", long, required = true)]
    pub world: String,

    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Manage scripts in the WORLD file")]
pub struct Wscript {
    #[clap(help="Path to the WORLD file", long, required = true)]
    pub world: String,

    #[clap(short, long, value_delimiter = ' ', help="Filename with script")]
    pub script: Option<String>,

    #[clap(short, long, value_delimiter = ' ', help="Name of the scripts")]
    pub name: Option<String>,

    #[clap(flatten, help="Command performed")]
    command: WscriptSrcArgGroup,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the BUND")]
pub struct Version {

}
