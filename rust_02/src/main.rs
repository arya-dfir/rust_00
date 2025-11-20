fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_help();
        return;
    }

    println!("{:?}", args);
}

fn print_help() {
    println!("Usage: hextool [OPTIONS]
Read and write binary files in hexadecimal

Options:
    -f, --file    Target file
    -r, --read    Read mode (display hex)
    -w, --write   Write mode (hex string to write)
    -o, --offset  Offset in bytes (decimal or 0x hex)
    -s, --size    Number of bytes to read
    -h, --help    Print help
");
}

