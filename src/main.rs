#![feature(option_result_contains)]
use structopt::StructOpt;
pub mod structs;
pub mod dom;
pub mod utils;

#[derive(StructOpt)]
pub struct Cli {
  file_type: String,
  number_of_tiers: u16,
}

impl Cli {
  fn get_file_type(&self) -> &String {
    &self.file_type
  }
  fn get_number_of_tiers(&self) -> u16 {
    self.number_of_tiers.clone()
  }
}


fn main() {
  let args = Cli::from_args();
  let cli_args = Cli { file_type: args.file_type, number_of_tiers: args.number_of_tiers };
  println!("Running for file type [{}] for [{}] number of tiers", cli_args.get_file_type(), cli_args.get_number_of_tiers());
  let dom_meta_data_list: Vec<structs::metadata::DomMetaData> = dom::dom::gather_metadata(cli_args);
}
