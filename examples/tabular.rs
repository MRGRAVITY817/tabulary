pub fn main() {
    let html = include_str!("tabular.html");
    let table_string = tabulary::html_to_table_string(html).unwrap_or("".to_string());
    println!("{table_string}");
}
