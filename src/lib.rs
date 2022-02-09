use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum HtmlTableError {
    #[error("cannot parse given string to html")]
    CannotParseStringToHtml,
}

pub fn html_to_table(html: &str) -> Result<String, HtmlTableError> {
    Ok(html.to_string())
}
