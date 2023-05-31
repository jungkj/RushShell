use std::fs::{File};
use std::io::{self, Write};
use std::process::{Command, Stdio};


fn main() {
    let mut background = Vec::new();

    loop {
        print!("rush$ ");
        io::stdout().flush().expect("Could not flush stdout!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error!");

        let mut line = input.trim().split_whitespace();
        let command = line.next().expect("Next failed");
        let mut args_vec = line.collect::<Vec<&str>>();

        if command.to_lowercase().eq("exit") {
            break;
        }

        let mut input_file = "";
        let mut output_file = "";
        let mut output_append = "";
        let mut inputs = [0, 0, 0, 0, 0];
        let mut i = 0;

        while i < args_vec.len() {
            // Handle <
            if args_vec[i].eq("<") && inputs[1] == 0 {
                // Check & is not before
                if inputs[3] != 0 {
                    println!("& must be after <!");
                    inputs[4] = 1;
                    break;
                }
                inputs[1] = 1;
                // check there is a file after
                if i == args_vec.len() - 1 {
                    println!("< must have a file afterwards!");
                    inputs[4] = 1;
                    break;
                }
                input_file = args_vec[i + 1];
                args_vec.remove(i);
                // Remove file name also
                args_vec.remove(i);
            } else if args_vec[i].eq("<") {
                // Check there is only 1 <
                println!("Error! Can't have two <'s");
                inputs[0] = 1;
                inputs[4] = 1;
                break;
            } else if args_vec[i].eq(">") || args_vec[i].eq(">>") && inputs[2] == 0 {
                // Check there is no & before > or >>
                if inputs[3] != 0 {
                    if output_append.eq(">") {
                        println!("& must be after >!");
                    } else {
                        println!("& must be after >>!");
                    }
                    inputs[4] = 1;
                    break;
                }
                inputs[2] = 1;
                output_append = args_vec[i];
                // Check there is a file after
                if i == args_vec.len() - 1 {
                    if output_append.eq(">") {
                        println!("> must have a file afterwards!");
                    } else {
                        println!(">> must have a file afterwars!");
                    }
                    inputs[4] = 1;
                    break;
                }
                output_file = args_vec[i + 1];
                args_vec.remove(i);
                // Remove output file also
                args_vec.remove(i);
            } else if args_vec[i].eq(">") || args_vec[i].eq(">>") {
                // Check only 1 > or >>
                println!("Error! Can't have two >'s or >>'s!");
                inputs[0] = 1;
                inputs[4] = 1;
                break;
            } else if args_vec[i].eq("&") && inputs[3] == 0 {
                // Check at end of line
                if i != args_vec.len() - 1 {
                    println!("& must be at the end of the line!");
                    inputs[4] = 1;
                    break;
                }
                inputs[3] = 1;
                args_vec.remove(i);
            } else if inputs[3] != 0 {
                // Check for only 1 &
                println!("Error! Can't have more than two &'s");
                inputs[0] = 1;
                inputs[4] = 1;
                break;
            } else {
                i += 1;
            }
        }
        if inputs[4] != 0 {
            continue;
        }

        // Get stdin source
        let fdin = if inputs[1] == 0 {
            // if flag not tripped, inherit from parent
            Stdio::inherit()
        } else {
            // If flag, get it from file
            let infile = File::options()
                .read(true)
                .open(input_file)
                .expect("Could not open file!");
            Stdio::from(infile)
        };

        let fdout = if inputs[2] == 0 {
            Stdio::inherit()
        } else {
            if output_append.eq(">") {
                let outfile = File::options()
                    .write(true)
                    .create(true)
                    .open(output_file)
                    .expect("Could not open file!");
                Stdio::from(outfile)
            } else {
                let outfile = File::options()
                    .append(true)
                    .create(true)
                    .open(output_file)
                    .expect("Could not open file!");
                Stdio::from(outfile)
            }
        };


        match command {
            "exit" => return,
            command => {
                let child = Command::new(command)
                    .args(args_vec)
                    .stdin(fdin)
                    .stdout(fdout)
                    .spawn();
                match child {
                    Ok(mut child) => {
                        if inputs[3] == 0 {
                            child.wait().expect("command wasn't running");
                        } else {
                            match child.try_wait() {
                                Ok(Some(_status)) => return,
                                Ok(None) => {
                                    background.push(child);
                                }
                                Err(e) => eprintln!("{}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        wait(&mut background);
    }
}

fn wait(background: &mut Vec<std::process::Child>) {
    let mut i = 0;
    while i < background.len() {
        match background[i].try_wait() {
            Ok(Some(_status)) => {
                background.remove(i);
            }
            Ok(None) => {
                i += 1;
            }
            Err(e) => println!("error attempting to wait: {e}"),
        }
    }
}


