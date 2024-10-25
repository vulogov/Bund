extern crate log;
use crate::stdlib::{init_stdlib};
use clap::{Parser, Subcommand, Args};

pub mod setloglevel;

pub mod bund_version;
pub mod bund_shell;
pub mod bund_eval;
pub mod bund_script;

pub mod bund_display_banner;

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    init_stdlib(&cli);
    log::debug!("BUND interpterer initialized ...");
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
        Commands::Version(_) => {
            bund_version::run(&cli);
        }
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

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable colors in output")]
    pub nocolor: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable bund.eval group of functions")]
    pub noeval: bool,

    #[clap(long, action = clap::ArgAction::SetTrue, help="Disable I/O group of functions")]
    pub noio: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    Script(Script),
    Eval(Eval),
    Shell(Shell),
    Version(Version),
}

#[derive(Args, Clone, Debug)]
#[clap(about="Execute BUND script")]
pub struct Script {
    #[clap(flatten)]
    source: ScriptSrcArgGroup,

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
    #[clap(last = true)]
    args: Vec<String>,
}

#[derive(Args, Clone, Debug)]
#[clap(about="Get the version of the BUND")]
pub struct Version {

}
