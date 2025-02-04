fn main() {
    println!("Hello, world!");
}

fn mail_search(email: &str) -> bool {
    false
}

fn email_from_str(mbox: &str) -> String {
    let mut email = String::new();

    // will need to implement going over emails in mbox and checking for From-line

    for email in mbox {
        if mail_search(email) {
            email
        }
        else {
            panic
        }
    }
}

fn revert_from_munge(line: &str) -> &str {
    if line.starts_with(">From") {
        &line[1..]
    }
    else {
        line
    }
}

// is this function getting a singular email and converting it to a string?
// or the whole mail box and converting
fn get_email_read_to_string(filename: &str) -> String {
    let mbox: String = fs::read_to_string(filename)?;
    email_from_str(&mbox)
}

// Need to implement iterator over the lines and check for From-lines
fn get_email_bufread_lines(filename: &str) -> String {
    let file = File::open(filename);
    let mut buf_reader = BufReader::new(file);
   



}