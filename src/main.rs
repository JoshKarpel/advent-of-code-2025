#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::exit;
use std::time::Instant;

use clap::{Parser, Subcommand};
use itertools::Itertools;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, COOKIE};

use advent_of_code_2025::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11, day_12,
    utils::SolverResult,
};

type SolverFunction = fn() -> SolverResult;

lazy_static! {
    static ref SOLVERS: HashMap<&'static str, SolverFunction> = {
        let mut solvers = HashMap::new();

        solvers.insert("01", day_01::solve as SolverFunction);
        solvers.insert("02", day_02::solve as SolverFunction);
        solvers.insert("03", day_03::solve as SolverFunction);
        solvers.insert("04", day_04::solve as SolverFunction);
        solvers.insert("05", day_05::solve as SolverFunction);
        solvers.insert("06", day_06::solve as SolverFunction);
        solvers.insert("07", day_07::solve as SolverFunction);
        solvers.insert("08", day_08::solve as SolverFunction);
        solvers.insert("09", day_09::solve as SolverFunction);
        solvers.insert("10", day_10::solve as SolverFunction);
        solvers.insert("11", day_11::solve as SolverFunction);
        solvers.insert("12", day_12::solve as SolverFunction);

        solvers
    };
}

fn run_solver(day: &str) -> SolverResult {
    if let Some(solver) = SOLVERS.get(day) {
        println!("★★ Day {} ★★★★★", day);
        println!(
            "★ https://adventofcode.com/2025/day/{}",
            day.trim_start_matches('0')
        );

        let before = Instant::now();

        let result = solver();

        println!("★ Elapsed time: {:.2?}", before.elapsed());
        println!("★★★★★★★★★★★★★★★");

        result
    } else {
        println!("Unknown day: {}", day);
        exit(1)
    }
}

fn download_input(day: &str) -> Result<(), Box<dyn Error>> {
    let session = env::var("AOC_SESSION").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, format!("session={}", session).parse().unwrap());

    let client = Client::new();
    let response = client
        .get(format!(
            "https://adventofcode.com/2025/day/{}/input",
            day.trim_start_matches('0')
        ))
        .headers(headers)
        .send()?
        .error_for_status()?;
    let input = response.text()?;

    let path = format!("data/day_{}.txt", day);
    create_dir_all("data")?;
    let mut file = File::options()
        .write(true)
        .create_new(true)
        .open(path.clone())?;
    file.write_all(input.as_bytes())?;

    println!("Wrote input for day {} to {}", day, path);

    Ok(())
}

#[derive(Parser)]
#[command(name = "Advent of Code 2025")]
#[command(version = "0.1.0")]
#[command(author = "Josh Karpel <josh.karpel@gmail.com>")]
#[command(about = "Josh's solutions for Advent of Code 2025.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download data for a given day.
    GetInput {
        /// The day to download the input for. Elide to download input for all days.
        day: Option<String>,
    },
    /// Solve the puzzle for a given day.
    Solve {
        /// The day to solve the puzzle for. Elide to solve all days.
        day: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GetInput { day } => {
            if let Some(day) = day.map(|d| format!("{:0>2}", d)) {
                download_input(&day)?;
            } else {
                SOLVERS
                    .keys()
                    .sorted()
                    .try_for_each(|day| download_input(day))?
            }
        }
        Commands::Solve { day } => {
            if let Some(day) = day.map(|d| format!("{:0>2}", d)) {
                run_solver(&day)?;
            } else {
                SOLVERS
                    .keys()
                    .sorted()
                    .try_for_each(|day| run_solver(day))?
            }
        }
    }

    Ok(())
}
