mod cmd_params;
mod error_handling;
use error_handling::MError;
use cmd_params::Args;
use structopt::StructOpt;


fn main() -> Result<(), MError> {
    let args = Args::from_args();
    Ok(())
}