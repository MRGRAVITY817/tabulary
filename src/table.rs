use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table as ComfyTable};

/// Elements in <tr>
pub struct Row {
    header: Option<String>,
    data: Vec<String>,
}

impl Row {
    /// Create an empty row
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
    /// Get row header
    pub fn header(&self) -> Option<&str> {
        self.header.as_ref().map(String::as_str)
    }
    /// Get row data
    pub fn data(&self) -> Vec<&str> {
        self.data.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
    }
    /// Get row total
    pub fn total(&self) -> Vec<&str> {
        match self.header() {
            Some(header) => [vec![header], self.data()]
                .into_iter()
                .flatten()
                .collect::<Vec<&str>>(),
            None => self.data(),
        }
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
    /// Create an empty table
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
    /// Get table caption
    pub fn caption(&self) -> Option<&str> {
        self.caption.as_ref().map(String::as_str)
    }
    /// Get table header
    pub fn header(&self) -> Option<&[String]> {
        self.header.as_ref().map(|h| h.as_slice())
    }
    /// Get table body
    pub fn body(&self) -> Option<&[Row]> {
        self.body.as_ref().map(|h| h.as_slice())
    }
    /// Get table footer
    pub fn footer(&self) -> Option<&Row> {
        self.footer.as_ref()
    }
}

impl ToString for Table {
    /// Convert table object into prettified string
    fn to_string(&self) -> String {
        let caption = format!(
            "{:-^15}\n{: ^15}\n{:-^15}",
            "",
            self.caption().unwrap_or("Table"),
            ""
        );
        // Draw a table
        let mut table = ComfyTable::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
            .set_table_width(80);
        if let Some(header) = self.header() {
            table.set_header(header);
        }
        if let Some(body) = self.body() {
            for row in body.into_iter() {
                table.add_row(row.total());
            }
        }
        if let Some(footer) = self.footer() {
            table.add_row(footer.total());
        }

        format!("{caption}\n{}", table.to_string())
    }
}
