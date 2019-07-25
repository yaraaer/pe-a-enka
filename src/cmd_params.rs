use structopt::StructOpt;
use structopt::clap::AppSettings;

#[derive(StructOpt)]
#[structopt(raw(settings="&[AppSettings::DeriveDisplayOrder,AppSettings::UnifiedHelpMessage,AppSettings::DisableVersion,AppSettings::DisableHelpSubcommand,AppSettings::ArgRequiredElseHelp,AppSettings::ColorAuto,AppSettings::ColoredHelp]"))]
pub struct Args {
    #[structopt(short = "p", help = "parse the raw shit to fields like a boss")]
    parse: bool,
}