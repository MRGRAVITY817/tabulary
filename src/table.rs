/// Elements in <tr>
pub struct Row {
    header: Option<String>,
    data: Vec<String>,
}

impl Row {
    /// Create new row with no contents
    pub fn new() -> Self {
        Self {
            header: None,
            data: vec![],
        }
    }
    /// Create new row with args
    pub fn from(header: Option<String>, data: Vec<String>) -> Self {
        Self { header, data }
    }
}

/// Elements in <table>
pub struct Table {
    caption: Option<String>,
    header: Option<Vec<String>>,
    body: Option<Vec<Row>>,
    footer: Option<Row>,
}

impl Table {
    /// Create new table with no contents
    pub fn new() -> Self {
        Self {
            caption: None,
            header: None,
            body: None,
            footer: None,
        }
    }
    /// Create new table with args
    pub fn from(
        caption: Option<String>,
        header: Option<Vec<String>>,
        body: Option<Vec<Row>>,
        footer: Option<Row>,
    ) -> Self {
        Self {
            caption,
            header,
            body,
            footer,
        }
    }
}
