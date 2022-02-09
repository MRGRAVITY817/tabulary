# Tabulary

A library to convert html table to Rust object and string, and vice versa.

## Work in progress. Try example first!

Use `html_to_table_string()` function to extract `table` element in html body and print it out.

```rust
/* examples/tabular.rs */

use tabulary::html_to_table_string;

pub fn main() {
    let html = include_str!("tabular.html");
    let table_string = html_to_table_string(html).unwrap_or("".to_string());
    println!("\n\n{table_string}");
}
```

Run this command to see the result.
```bash
$ cargo run --example tabular

## Estimated result

Contacts
╭────────────┬───────────┬────────────────────────╮
│ First Name ┆ Last Name ┆ Email Address          │
╞════════════╪═══════════╪════════════════════════╡
│ Hoon       ┆ Wee       ┆ mrgravity817@gmail.com │
├╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Eunbee     ┆ Cho       ┆ nicevil917@naver.com   │
╰────────────┴───────────┴────────────────────────╯


╭───┬──────────────────────┬─────────────╮
│   ┆ Semester             ┆ Grade       │
╞═══╪══════════════════════╪═════════════╡
│ 1 ┆ Jan - April          ┆ Credit      │
├╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2 ┆ May - August         ┆ Pass        │
├╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 2 ┆ September - December ┆ Distinction │
╰───┴──────────────────────┴─────────────╯
```