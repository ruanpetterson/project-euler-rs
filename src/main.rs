use std::str::FromStr;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Project Euler answers")
        .version("0.1.0")
        .author("Ruan Petterson <ruan@ruan.eng.br")
        .arg(
            Arg::with_name("problem")
                .value_name("PROBLEM ID")
                .help("Problem ID")
                .index(1)
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(problem) = matches.value_of("problem") {
        if let Ok(problem_id) = usize::from_str(problem) {
            match problem_id {
                _ => unimplemented!(),
            }
        } else {
            let error = clap::Error::with_description(
                "Problem ID must be a number",
                clap::ErrorKind::InvalidValue,
            );
            eprint!("{}", error);
            std::process::exit(1);
        }
    }
}
