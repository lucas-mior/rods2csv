use std::io;
use std::process::exit;
use regex::Regex;

fn main() {
    println!("============== rods2csv ==============");
    let mut row: u32 = 0;

    let mut string = String::new();
    // let mut content;
    // let mut fina;
    let table_rep = Regex::new("table-cell.*number-columns-repeated").unwrap();
    let table_span = Regex::new("table-cell.*number-columns-spanned").unwrap();
    let any_table = Regex::new("table-cell.*number-columns-(repeated|spanned)").unwrap();

    let trep_a = Regex::new(".*repeated=\"([[:digit:]]+)\"[^>]*/>").unwrap();
    let trep_b = Regex::new(".*repeated=\"([[:digit:]]+)\"[^>/]*>").unwrap();

    let table_row = Regex::new("table-row").unwrap();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        if table_rep.is_match(&line) {
            println!("repeated: {}", line);
            let caps = match trep_a.captures(&line) {
                Some(caps) => caps,
                _ => match trep_b.captures(&line) {
                    Some(caps) => caps,
                    _ => {println!("caps failed"); exit(1);},
                },
            };
            println!("caps: {}", line);
            let x = caps.get(1).map_or("", |m| m.as_str());
            println!("x: {}", x);
            let mut a = x.parse::<usize>().unwrap();
            if a > 30 {
                a = 30;
            }
            for _ in 0..a {
                string.push_str("<table-cell/>");
            }
            println!("String: {}", string);
        } else if table_span.is_match(&line) {
            println!("spanned: {}", line);
        } else if ! any_table.is_match(&line) {
            println!("not any table: {}", line);
            if table_row.is_match(&line) {
                println!("table-row: {}", line);
                row += 1;
            }
            println!("{}", line);
            if row > 100 {
                exit(0);
            }
        }
    }
}
