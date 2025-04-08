extern crate log;
use std::env;
use time_graph;
use crate::stdlib::BUND;
use crate::stdlib::{init_stdlib};
use lazy_static::lazy_static;
use std::sync::Mutex;
use uuid::Uuid;
use crate::stdlib::helpers::hostname::get_hostname;
use clap::{Parser, Subcommand, Args};

pub mod setloglevel;

pub mod bund_version;
pub mod bund_shell;
pub mod bund_eval;
pub mod bund_test;
pub mod bund_script;
pub mod bund_wscript;
pub mod bund_bbus;
pub mod bund_cluster;
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

fn do_panic() {
    log::debug!("Setting a global panic handler");
    better_panic::Settings::auto()
        .most_recent_first(false)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();
}

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    do_panic();
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
        Commands::Test(test) => {
            bund_test::run(&cli, &test);
        }
        Commands::Load(load) => {
            bund_load::run(&cli, &load);
        }
        Commands::Wscript(wscript) => {
            bund_wscript::run(&cli, &wscript);
        }
        Commands::Bus(bbus) => {
            bund_bbus::run(&cli, &bbus);
        }
        Commands::Cluster(cluster) => {
            bund_cluster::run(&cli, &cluster);
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

#[derive(Parser, Clone, Debug)]
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
    pub bus: DistributedArgGroup,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Script(Script),
    Eval(Eval),
    Shell(Shell),
    Test(Test),
    Load(Load),
    Wscript(Wscript),
    Bus(Bbus),
    Cluster(Cluster),
    Version(Version),
}

#[derive(Debug, Clone, clap::Args)]
#[group(required = false, multiple = true)]
pub struct DistributedArgGroup {
    #[clap(help="BUS configuration file", long, default_value_t = String::from(env::var("BUND_BUS_CONFIG_FILE").unwrap_or("bund_client.json5".to_string())))]
    pub bus_config: String,

    #[clap(help="Distributed hostname", long, default_value_t = String::from(get_hostname()))]
    pub hostname: String,

    #[clap(help="Distributed node ID", long, default_value_t = String::from(Uuid::new_v4().to_string()))]
    pub nodeid: String,

    #[clap(help="Distributed node role", long, default_value_t = String::from("BUND"))]
    pub noderole: String,

    #[clap(help="Instance receiving queue prefix", long, default_value_t = String::from(env::var("BUND_BUS_RECEIVING_PREFIX").unwrap_or("zbus/receiving".to_string())))]
    pub receiving: String,

    #[clap(help="BUND global variables bus prefix", long, default_value_t = String::from(env::var("BUND_GLOBALS_PREFIX").unwrap_or("zbus/globals".to_string())))]
    pub globals_prefix: String,

    #[clap(help="BUND scripts bus prefix", long, default_value_t = String::from(env::var("BUND_SCRIPTS_PREFIX").unwrap_or("zbus/scripts".to_string())))]
    pub scripts_prefix: String,

    #[clap(help="BUND execution outcome bus prefix", long, default_value_t = String::from(env::var("BUND_OUTCOME_PREFIX").unwrap_or("zbus/result".to_string())))]
    pub outcome_prefix: String,

    #[clap(help="Hostname of the beanstalkd server", long, default_value_t = String::from(env::var("BUND_BEANSTALKD_ADDR").unwrap_or("127.0.0.1".to_string())))]
    pub beanstalk_host: String,

    #[clap(help="Port of the beanstalkd server", long, default_value_t = 11300 )]
    pub beanstalk_port: u16,

    #[clap(help="Beanstalkd tube", long, default_value_t = String::from(env::var("BUND_BEANSTALKD_TUBE").unwrap_or("bund".to_string())))]
    pub beanstalk_tube: String,
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
#[clap(about="Run test cases inside the BUND environment")]
pub struct Test {
    #[clap(last = true)]
    cases: Vec<String>,
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

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct BbusArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Publish data on the BUS")]
    pub publish: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Subscribe to the BUS queue")]
    pub subscribe: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Put value to the BUS")]
    pub put: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Get value from the bus")]
    pub get: bool,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Manage data stored in the BUND data bus")]
pub struct Bbus {

    #[clap(short, long, help="Value for sending to the BUS")]
    pub value: Option<String>,

    #[clap(short, long, value_delimiter = ' ', help="BUS pub/sub/get/put key")]
    pub key: Option<String>,

    #[clap(flatten, help="BUS command")]
    command: BbusArgGroup,

    #[clap(last = true)]
    args: Vec<String>,

}

#[derive(Debug, Clone, clap::Args)]
#[group(required = true, multiple = false)]
pub struct ClusterArgGroup {
    #[clap(long, action = clap::ArgAction::SetTrue, help="Publish script to the BUS for execution")]
    pub publish: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Download previously published script")]
    pub download: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Schedule script for execution")]
    pub schedule: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Running distributed script execution service")]
    pub actor: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Push data to the PUB/SUB queue")]
    pub push: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Pull data from the PUB/SUB queue and optionally running them through ACTOR")]
    pub pull: bool,

}

#[derive(Args, Clone, Debug)]
#[clap(about="BUND distributed cluster operation")]
pub struct Cluster {

    #[clap(flatten)]
    source: ScriptSrcArgGroup,

    #[clap(short, long, help="Value key")]
    pub key: Option<String>,

    #[clap(short, long, help="Cluster script")]
    pub job: Option<String>,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Upload script before schedule")]
    pub upload: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Send data to STDOUT where applicable")]
    pub stdout: bool,

    #[clap(help="Execution ID", long, default_value_t = String::from(Uuid::new_v4().to_string()))]
    pub execid: String,

    #[clap(flatten, help="CLUSTER command")]
    command: ClusterArgGroup,

    #[clap(last = true)]
    args: Vec<String>,

}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the BUND")]
pub struct Version {

}
