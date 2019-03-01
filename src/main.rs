use regex::Regex;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::time::Instant;
use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
//mod day_11;
//mod day_12;
//mod day_13;
//mod day_14;
//mod day_15;
//mod day_16;
//mod day_17;
//mod day_18;
//mod day_19;
//mod day_20;
//mod day_21;
//mod day_22;
//mod day_23;
//mod day_24;
//mod day_25;

fn main() -> Result<(), Error> {
    println!("Advent of code 2016.");
    println!();
    let arguments: Vec<String> = env::args().collect();
    let mut run_all = false;

    if arguments.len() == 1 {
        run_all = true;
    } else if !Regex::new(r"^day_\d+").unwrap().is_match(&arguments[1]) {
        println!("Invalid day \"{}\".", &arguments[1]);
        println!("Day should be entered like \"day_1\".");
        println!("Or leave it blank to run all solutions.");
        return Ok(());
    }

    let before = Instant::now();

    if (run_all || &arguments[1] == "day_1") && file_exists("day_1") {
        day_1::solve(&read_file("day_1")?);
    }

    if (run_all || &arguments[1] == "day_2") && file_exists("day_2") {
        day_2::solve(&read_file("day_2")?);
    }

    if (run_all || &arguments[1] == "day_3") && file_exists("day_3") {
        day_3::solve(&read_file("day_3")?);
    }

    if (run_all || &arguments[1] == "day_4") && file_exists("day_4") {
        day_4::solve(&read_file("day_4")?);
    }

    if (run_all || &arguments[1] == "day_5") && file_exists("day_5") {
        day_5::solve(&read_file("day_5")?);
    }

    if (run_all || &arguments[1] == "day_6") && file_exists("day_6") {
        day_6::solve(&read_file("day_6")?);
    }

    if (run_all || &arguments[1] == "day_7") && file_exists("day_7") {
        day_7::solve(&read_file("day_7")?);
    }

    if (run_all || &arguments[1] == "day_8") && file_exists("day_8") {
        day_8::solve(&read_file("day_8")?);
    }

    if (run_all || &arguments[1] == "day_9") && file_exists("day_9") {
        day_9::solve(&read_file("day_9")?);
    }

    if (run_all || &arguments[1] == "day_10") && file_exists("day_10") {
        day_10::solve(&read_file("day_10")?);
    }

//    if (run_all || &arguments[1] == "day_11") && file_exists("day_11") {
//        day_11::solve(&read_file("day_11")?);
//    }
//
//    if (run_all || &arguments[1] == "day_12") && file_exists("day_12") {
//        day_12::solve(&read_file("day_12")?);
//    }
//
//    if (run_all || &arguments[1] == "day_13") && file_exists("day_13") {
//        day_13::solve(&read_file("day_13")?);
//    }
//
//    if (run_all || &arguments[1] == "day_14") && file_exists("day_14") {
//        day_14::solve(&read_file("day_14")?);
//    }
//
//    if (run_all || &arguments[1] == "day_15") && file_exists("day_15") {
//        day_15::solve(&read_file("day_15")?);
//    }
//
//    if (run_all || &arguments[1] == "day_16") && file_exists("day_16") {
//        day_16::solve(&read_file("day_16")?);
//    }
//
//    if (run_all || &arguments[1] == "day_17") && file_exists("day_17") {
//        day_17::solve(&read_file("day_17")?);
//    }
//
//    if (run_all || &arguments[1] == "day_18") && file_exists("day_18") {
//        day_18::solve(&read_file("day_18")?);
//    }
//
//    if (run_all || &arguments[1] == "day_19") && file_exists("day_19") {
//        day_19::solve(&read_file("day_19")?);
//    }
//
//    if (run_all || &arguments[1] == "day_20") && file_exists("day_20") {
//        day_20::solve(&read_file("day_20")?);
//    }
//
//    if (run_all || &arguments[1] == "day_21") && file_exists("day_21") {
//        day_21::solve(&read_file("day_21")?);
//    }
//
//    if (run_all || &arguments[1] == "day_22") && file_exists("day_22") {
//        day_22::solve(&read_file("day_22")?);
//    }
//
//    if (run_all || &arguments[1] == "day_23") && file_exists("day_23") {
//        day_23::solve(&read_file("day_23")?);
//    }
//
//    if (run_all || &arguments[1] == "day_24") && file_exists("day_24") {
//        day_24::solve(&read_file("day_24")?);
//    }
//
//    if (run_all || &arguments[1] == "day_25") && file_exists("day_25") {
//        day_25::solve(&read_file("day_25")?);
//    }

    let after = before.elapsed();
    println!("Total duration: {} ms.", after.as_secs() * 1000 + (after.subsec_nanos() / 1_000_000) as u64);
    Ok(())
}

fn file_exists(path: &str) -> bool {
    let mut full_path = String::from("./res/");
    full_path.push_str(path);
    Path::new(&full_path).exists()
}

fn read_file(path: &str) -> Result<String, Error> {
    let mut full_path = String::from("./res/");
    full_path.push_str(path);
    let mut buffer = fs::read_to_string(full_path)
            .expect(&format!("Could not read file \"{}\"", path));
    buffer = buffer.replace('\r', "").trim_end().to_owned();
    Ok(buffer)
}
