use std::env;
use std::fs;
use tiny_json as json;

fn print_help_info() {
    println!("Usage: tiny-json <options> filename");
    println!("example: tiny-json -f a.tiny-json\n");
    println!("Options");
    println!("    -f [indent]   format tiny-json file");
    println!("    -m            minify tiny-json file\n");
}

fn format_json(filename: &str, indent: u32) {
    let content = fs::read_to_string(filename)
        .expect(&format!("{}{}", "Read file error at:", filename));
    let node = json::parse(&content);
    let format_content = json::stringify(&node, indent);
    fs::write(filename, &format_content).expect("Write File Error");
}

pub fn json_commander() {
    let mut i = 0;
    let mut args: Vec<String> = vec![];
    for x in env::args() {
        if i >= 1 {
            args.push(x);
        }
        i += 1;
    }

    if args.len() == 0 {
        return print_help_info();
    }

    let mut args = args.iter();
    let op = args.next().unwrap();
    if op == "-f" {
        let indent: u32 = args.next().unwrap().parse().expect("The argument indent should be a number");
        let filename = args.next().expect("Missing argument filename");
        format_json(filename, indent);
    } else if op == "-m" {
        let filename = args.next().expect("Missing argument filename");
        format_json(filename, 0);
    } else {
        print_help_info();
    }
}