pub struct Header(String);

impl Header {
    pub fn from(s: String) -> Self {
        Self(s)
    }
}

impl ToString for Header {
    fn to_string(&self) -> String {
        format!("[{: ^18}]", self.0)
    }
}

/// Elements in <tr>
pub struct Row {
    header: Option<Header>,
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
    pub fn from(header: Option<Header>, data: Vec<String>) -> Self {
        Self { header, data }
    }
    /// Get row header
    pub fn header(&self) -> Option<&Header> {
        self.header.as_ref()
    }
    /// Get row data
    pub fn data(&self) -> Vec<&str> {
        self.data.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
    }
}

impl ToString for Row {
    /// Make a string version of row
    fn to_string(&self) -> String {
        let header = match self.header() {
            Some(h) => h.to_string(),
            None => "".to_string(),
        };
        let data = self
            .data()
            .iter()
            .map(|&item| format!("{: ^20}", item))
            .collect::<Vec<_>>()
            .join("");
        format!("{header}{data}")
    }
}

/// Elements in <table>
pub struct Table {
    caption: Option<String>,
    header: Option<Vec<Header>>,
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
        header: Option<Vec<Header>>,
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
    pub fn header(&self) -> Option<&[Header]> {
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
            "{:-^20}\n||{: ^16}||\n{:-^20}",
            "",
            self.caption().unwrap_or("Table"),
            ""
        );
        let header = self
            .header()
            .unwrap_or(&[])
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("");
        let body = self
            .body()
            .unwrap_or(&[])
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        let footer = self.footer().unwrap_or(&Row::new()).to_string();

        format!("{caption}\n\n{header}\n{body}\n{footer}\n")
    }
}
