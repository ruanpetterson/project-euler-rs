use std::str::FromStr;

use clap::{App, Arg};

use project_euler::problem_0001;
use project_euler::problem_0002;
use project_euler::problem_0003;
use project_euler::problem_0006;
use project_euler::problem_0007;

use project_euler::Solution;

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
                1 => problem_0001::Problem::solve(),
                2 => problem_0002::Problem::solve(),
                3 => problem_0003::Problem::solve(),
                6 => problem_0006::Problem::solve(),
                7 => problem_0007::Problem::solve(),
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
