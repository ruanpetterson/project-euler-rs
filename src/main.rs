use std::str::FromStr;

use clap::{App, Arg};
use project_euler::{problem_0001, problem_0002};

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
                1 => println!(
                    "{}",
                    problem_0001::multiples_of_3_or_5(1000)
                        .into_iter()
                        .sum::<usize>()
                ),
                2 => println!(
                    "{}",
                    problem_0002::Fibonacci::default()
                        .take_while(|&n| n <= 4_000_000)
                        .filter(|&n| n % 2 == 0)
                        .sum::<usize>()
                ),
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
