use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn print_usage() {
    println!("del_lines [options] [file]");
    println!("options:");
    println!("-n         : List of space seperated line numbers");
    println!("-f         : File(s) to delete lines in          ");
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() < 3 {
        print_usage();
        panic!("Program was invoked incorrectly")
    }
    let n_pos = cli_args.iter().position(|q| q == "-n").unwrap();
    let f_pos = cli_args.iter().position(|q| q == "-f").unwrap();
    let mut del_lines: Vec<usize>;
    let file_s: &[String];
    if n_pos > f_pos {
        file_s = &cli_args[f_pos + 1..n_pos];
        del_lines = cli_args[n_pos + 1..]
            .iter()
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
    } else {
        file_s = &cli_args[f_pos + 1..];
        del_lines = cli_args[n_pos + 1..f_pos]
            .iter()
            .inspect(|l| println!("{}", l))
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
    }
    del_lines.sort();
    for f in file_s {
        let mut file_hndl = File::open(f).unwrap();
        let mod_file = f.clone() + "_del_lines";
        let buffer = BufReader::new(&mut file_hndl);
        let unwrap_lines: Vec<String> = buffer.lines().map(|l| l.unwrap()).collect();
        let mut new_lines: Vec<String> = Vec::new();
        'outer: for (i, l) in unwrap_lines.iter().enumerate() {
            for d in del_lines.iter() {
                if i == *d - 1 {
                    continue 'outer;
                }
            }
            new_lines.push(l.to_string())
        }
        let new_file = File::create(mod_file).unwrap();
        let mut write_buffer = BufWriter::new(new_file);
        new_lines.iter().for_each(|l| {
            writeln!(write_buffer, "{}", l).unwrap();
        });
    }
}
