use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
get_email_read_to_string: 
    Small: real 0m4.476s
    Large: real 0m15.373s

get_email_bufread_lines:
    Small: 0m15.909s
    Large: 0m45.927s

get_email_bufread_read_line:
    Small: 0m5.558s
    Large: 0m12.386s

bufread_read_line and read_to_string is faster than bufread_lines, 
and while read_line seems a little slower for the smaller file, 
it is faster for the larger one. bufread_lines might be slower since 
it is using an iterator for each line.
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let email = get_email_bufread_read_line(filename); // dropping the `mail_search::` part if you don't have src/lib.rs
    print!("{email}");
}

fn condition(email: &str) -> bool {
    false
}

fn email_from_str(mbox: &str) -> String {
    let mut email = String::new();
    let mut collecting = false;

    for line in mbox.lines() {
        if line.starts_with("From ") {
            if collecting && !email.is_empty() {
                if condition(&email) {
                    return email;
                }
                email.clear();
            }
            collecting = true;
        } else if collecting {
            email.push_str(revert_from_munge(line));
        }
    }

    if collecting && !email.is_empty() && condition(&email) {
        return email;
    } else {
        panic!();
    }
}

fn revert_from_munge(line: &str) -> &str {
    if line.starts_with(">From") {
        &line[1..]
    } else {
        line
    }
}

// is this function getting a singular email and converting it to a string?
// or the whole mail box and converting
fn get_email_read_to_string(filename: &str) -> String {
    let mbox: String = fs::read_to_string(filename).expect("Failed to read file");
    email_from_str(&mbox)
}

// Need to implement iterator over the lines and check for From-lines
fn get_email_bufread_lines(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file");
    let buf_reader = BufReader::new(file);

    let mut email = String::new();
    let mut collecting = false;

    for line in buf_reader.lines() {
        let line = line.expect("Failed ot read line");


        if line.starts_with("From ") {
            if collecting && !email.is_empty() {
                if condition(&email) {
                    return email;
                }
                email.clear();
            }
            collecting = true;
        }
        if collecting {
            email.push_str(revert_from_munge(&line));
        }
    }

    if condition(&email) {
        return email;
    } else {
        panic!();
    }
}

fn get_email_bufread_read_line(filename: &str) -> String {
    let file = File::open(filename).expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);

    let mut buf = String::new();
    let mut email = String::new();
    let mut collecting  = false;

    while buf_reader.read_line(&mut buf).expect("Failed to read line") > 0 {
        if buf.starts_with("From ") {
            if collecting && !email.is_empty() {
                if condition(&email) {
                    return email;
                }
                email.clear();
            }
            collecting = true;
        }
        if collecting {
            email.push_str(revert_from_munge(&buf));
        }
        buf.clear();
        
    }

    if collecting && !email.is_empty() && condition(&email) {
        return email;
    }
    panic!();
}

