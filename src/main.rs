use std::io;
use std::process::exit;
use regex::Regex;

fn worka(caps: &regex::Captures) {
    let mut string = String::new();
    let x = caps.get(1).map_or("", |m| m.as_str());
    let mut a = x.parse::<usize>().unwrap();
    if a > 30 {
        a = 30;
    }
    for _ in 0..a {
        string.push_str("<table-cell/>");
    }
    print!("{}", string);
}


fn main() {
    // println!("============== rods2csv ==============");
    let mut row: u32 = 0;

    let table_rep = Regex::new("table-cell.*number-columns-repeated").unwrap();
    let table_span = Regex::new("table-cell.*number-columns-spanned").unwrap();
    let any_table = Regex::new("table-cell.*number-columns-(repeated|spanned)").unwrap();

    let trep_a = Regex::new(".*repeated=\"([[:digit:]]+)\"[^>]*/>").unwrap();
    let trep_b = Regex::new(".*repeated=\"([[:digit:]]+)\"[^>/]*>").unwrap();

    let tspan_a = Regex::new(".*columns-spanned=\"([[:digit:]]+)\"[^>]*/>").unwrap();
    let tspan_b = Regex::new(".*columns-spanned=\"([[:digit:]]+)\"[^>/]*>").unwrap();

    let table_row = Regex::new("table-row").unwrap();
    let table_cell = Regex::new("</table-cell>").unwrap();
    let cont = Regex::new("<[^>]*>").unwrap();

    let mut lines = io::stdin().lines();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if table_rep.is_match(&line) {
            match trep_a.captures(&line) {
                Some(caps) => worka(&caps),
                _ => match trep_b.captures(&line) {
                    Some(caps) => {
                        let mut string = String::new();
                        let mut content = String::new();
                        let mut fina = String::new();
                        let x = caps.get(1).map_or("", |m| m.as_str());
                        let mut b = x.parse::<usize>().unwrap();
                        if b > 50 {
                            b = 50;
                        }
                        while ! table_cell.is_match(&line) {
                            if let Some(line) = lines.next() {
                                let line = match line {
                                    Ok(line) => line,
                                    Err(_) => break,
                                };
                                content.push_str(&line);
                            } else {
                                break;
                            }
                        }
                        let content = cont.replace(&content, "");
                        for _ in 0..b {
                            print!("<table-cell>{}</table-cell>", content);
                        }
                    }
                    _ => {println!("caps failed"); exit(1);},
                },
            };
        } else if table_span.is_match(&line) {
            match tspan_a.captures(&line) {
                Some(caps) => worka(&caps),
                _ => match tspan_b.captures(&line) {
                    Some(caps) => {
                        let mut string = String::new();
                        let mut content = String::new();
                        let mut fina = String::new();
                        let x = caps.get(1).map_or("", |m| m.as_str());
                        let mut b = x.parse::<usize>().unwrap();
                        if b > 50 {
                            b = 50;
                        }
                        while ! table_cell.is_match(&line) {
                            if let Some(line) = lines.next() {
                                let line = match line {
                                    Ok(line) => line,
                                    Err(_) => break,
                                };
                                content.push_str(&line);
                            } else {
                                break;
                            }
                        }
                        let content = cont.replace(&content, "");
                        for _ in 0..b {
                            print!("<table-cell>{}</table-cell>", content);
                        }
                    }
                    _ => {println!("caps failed"); exit(1);},
                },
            };
        } else if ! any_table.is_match(&line) {
            if table_row.is_match(&line) {
                row += 1;
            }
            print!("{}", line);
            if row > 100 {
                exit(0);
            }
        }
    }
}
