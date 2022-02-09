use tabulary::html_to_table_string;

pub fn main() {
    let html = include_str!("tabular.html");
    let table_string = html_to_table_string(html).unwrap_or("".to_string());
    println!("\n\n{table_string}");
}
