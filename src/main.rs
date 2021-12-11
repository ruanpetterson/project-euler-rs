use std::str::FromStr;

use clap::{App, Arg};

use project_euler::problem_0001;
use project_euler::problem_0002;
use project_euler::problem_0003;
use project_euler::problem_0007;

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
                3 => println!(
                    "{:?}",
                    (3..)
                        .step_by(2)
                        .into_iter()
                        .find(|&n| 600851475143 % n == 0
                            && problem_0003::is_prime(600851475143 / n))
                        .map(|n| 600851475143 / n)
                        .unwrap()
                ),
                6 => {
                    let a = (1u32..=100).sum::<u32>().pow(2);
                    let b = (1u32..=100).map(|n| n.pow(2)).sum::<u32>();
                    println!("{} - {} = {}", a, b, a - b);
                }
                7 => println!(
                    "{}",
                    problem_0007::Prime::default()
                        .into_iter()
                        .nth(10_001 - 1)
                        .unwrap()
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
