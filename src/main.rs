extern crate clap;

use clap::{Arg, App};
use std::process::Command;

fn main() {
    let processing = App::new("mypipe")
       .version("1.0")
       .about("Could you please enter the first argument to pass it to the second one?")
       .author("Florian BERGERON")
       .arg(Arg::with_name("input")
            .short("in")
            .long("in")
            .help("Could you please set your first argument?")
            .takes_value(true)
            )
        .arg(Arg::with_name("output")
            .short("out")
            .long("out")
            .help("Could you please set your second argument?")
            .takes_value(true)
            )
        .get_matches();

    if processing.value_of("in") == None || processing.value_of("out") == None {
        println!("You didn\'t have entered an argument.");
        return;
    }

    // Get argument after our input
    let input_user_argument = processing.value_of("input").unwrap();

    // Get argument after our output
    let output_user_argument = processing.value_of("output").unwrap();

    let output_result_from_input = Command::new(input_user_argument.to_string())
                            .output()
                            .expect("Program uncounter an error while processing!");

    let transfert_arg = output_result_from_input.stdout;
    let output_result_from_output = Command::new(output_user_argument.to_string())
                            .arg(String::from_utf8_lossy(&transfert_arg).to_string())
                            .output()
                            .expect("Program uncounter an error while processing!");

    println!("{}", String::from_utf8_lossy(&output_result_from_output.stdout));
}
