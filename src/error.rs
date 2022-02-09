use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum HtmlTableError {
    #[error("cannot parse given string to html: {0}")]
    CannotParseStringToHtml(String),
    #[error("table not found in html document")]
    TableNotFoundInHtmlDocument,
}
