use crate::Cli;
use std::{fs};
use crate::structs::metadata::{ DomMetaData, TranslationFnDetails };
use crate::utils::string_utils::{ find_start_at };

pub fn gather_metadata(cli_args: Cli) -> Vec<DomMetaData> {
  let dom_meta_data_vector = index_directories(cli_args);
  dom_meta_data_vector
}

fn index_directories(cli_args: Cli) -> Vec<DomMetaData> {
  let paths = fs::read_dir("./example/src").unwrap();
  let mut path_matches: Vec<DomMetaData> = Vec::new();

  // todo look at doing this in parallel
  // https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html
  for path in paths {
    let p = path.unwrap().path().display().to_string();
    if p.contains(cli_args.get_file_type()) {
      let translation_details = find_translation_fns(p.clone());
      let meta_data = DomMetaData { path: p, translation_details };
      path_matches.push(meta_data);
    }
  }

  if path_matches.len() == 0 {
    panic!("Total file matches cannot be 0");
  }

  println!("The total file matches are [{}]", path_matches.len());
  path_matches
}

fn find_translation_fns(file_path: String) -> Vec<TranslationFnDetails> {
  let translation_details = Vec::new();
  let stringified_dom = fs::read_to_string(file_path).expect("Unable to read file at provided path");

  // stringified_dom.index_of
  
  translation_details
}