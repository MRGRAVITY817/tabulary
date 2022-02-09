pub mod error;
pub mod nodes;
pub mod table;

use {
    error::HtmlTableError,
    nodes::{get_body, get_caption, get_footer, get_header},
    select::{document::Document, node::Node, predicate::Name},
    table::Table,
};

/// Convert node to text string
pub fn node_to_text(node: Node) -> String {
    node.text().trim().to_string()
}

/// Extract tables in give html string and return formatted string.
pub fn html_to_table_string(html: &str) -> Result<String, HtmlTableError> {
    if let Ok(document) = Document::from_read(&mut html.as_bytes())
        .map_err(|e| HtmlTableError::CannotParseStringToHtml(e.to_string()))
    {
        let tables = document.find(Name("table")).collect::<Vec<_>>();
        if tables.is_empty() {
            return Err(HtmlTableError::TableNotFoundInHtmlDocument);
        }

        let _tables = tables
            .iter()
            .map(|table| {
                Table::from(
                    get_caption(table),
                    get_header(table),
                    get_body(table),
                    get_footer(table),
                )
            })
            .collect::<Vec<_>>();
        return Ok("Ok".to_string());
    }
    Err(HtmlTableError::CannotParseStringToHtml(html.to_owned()))
}
