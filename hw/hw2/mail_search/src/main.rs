fn main() {
    println!("Hello, world!");
}

fn condition(email: &str) -> bool {
    false
}

fn email_from_str(mbox: &str) -> String {
    let mut email = String::new();
    let mut collecting = false;

    // will need to implement going over emails in mbox and checking for From-line
    // iterate over each line, creating the email based on if From-line exists

    for line in mbox.lines() {
        if line.starts_with("From") {
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

    if condition(email) {
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
    let mbox: String = fs::read_to_string(filename)?;
    email_from_str(&mbox)
}

// Need to implement iterator over the lines and check for From-lines
fn get_email_bufread_lines(filename: &str) -> String {
    let file = File::open(filename);
    let mut buf_reader = BufReader::new(file);

    let email = String::new();
    let mut collecting = false;

    for line in buf_reader.lines() {
        if line.starts_with("From") {
            if collecting && !email.is_empty() {
                if condition(&email) {
                    return email;
                }
                email.clear();
            }
            collecting = true;
        }
        if collecting {
            email.push_str(revert_from_munge(line));
        }
    }

    if condition(email) {
        return email;
    } else {
        panic!();
    }
}

fn get_email_bufread_read_line(filename: &str) -> String {
    let file = File::open(filename);
    let mut buf_reader = BufRead::new(file);

    let mut buf = String::new();
    let mut email = String::new();

    // file not done not implemeted
    // when number of bytes is not 0?
    while (buf_reader.read_line(&mut buf) != 0) {
        // double reading?
        buf_reader.read_line(&mut buf);
        if buf.starts_with("From") {
            if condition(email) {
                return email;
            }
            email.clear();
        }
        email.push_str(revert_from_munge(buf))
    }

    if condition(email) {
        return email;
    }
    panic!();
}
