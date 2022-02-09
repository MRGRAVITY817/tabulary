mod error;
mod nodes;
mod table;
mod utils;

use {
    error::HtmlTableError,
    nodes::{get_body, get_caption, get_footer, get_header},
    select::{document::Document, predicate::Name},
    table::Table,
};

/// Extract tables in give html string and return formatted string.
pub fn html_to_table_string(html: &str) -> Result<String, HtmlTableError> {
    if let Ok(document) = Document::from_read(&mut html.as_bytes())
        .map_err(|e| HtmlTableError::CannotParseStringToHtml(e.to_string()))
    {
        let tables = document.find(Name("table")).collect::<Vec<_>>();
        if tables.is_empty() {
            return Err(HtmlTableError::TableNotFoundInHtmlDocument);
        }

        return Ok(tables
            .iter()
            .map(|table| {
                Table::from(
                    get_caption(table),
                    get_header(table),
                    get_body(table),
                    get_footer(table),
                )
            })
            .map(|table| table.to_string())
            .collect::<Vec<_>>()
            .join("\n\n"));
    }
    Err(HtmlTableError::CannotParseStringToHtml(html.to_owned()))
}
