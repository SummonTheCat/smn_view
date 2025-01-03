use super::Color;


pub fn log(msg: &str) {
    print!("{}", msg);
}

pub fn logln(msg: &str) {
    println!("{}", msg);
}

pub fn log_color(msg: &str, color: Color) {
    print!("{}", color.paint(msg));
}

pub fn logln_color(msg: &str, color: Color) {
    println!("{}", color.paint(msg));
}

pub fn log_line_header(msg: &str, color: Color, line_length: usize) {
    let header = format!("{:-^1$}", msg, line_length);
    logln_color(&header, color);
}

pub fn log_line(color: Color, line_length: usize) {
    let line = format!("{:-^1$}", "", line_length);
    log_color(&line, color);
    logln("");
}