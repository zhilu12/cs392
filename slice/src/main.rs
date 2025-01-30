use std::io;

fn main() {
    // use split
    let del = ">>>>>>>>>>>>>>>>>>>>";
    let mut del_exists: bool = false;

    let lines = io::stdin().lines();
    for line in lines {
        let mut line = line.unwrap();
        if line.contains(del) {
            del_exists = !del_exists;
            continue;
        }

        if !del_exists {
            println!("{}", line);
        }
    }
}
