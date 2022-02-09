use {
    crate::{table::Row, utils::node_to_text},
    select::{
        node::Node,
        predicate::{Name, Predicate},
    },
};

/// Get caption of the table
pub fn get_caption<'a>(table: &Node<'a>) -> Option<String> {
    table.find(Name("caption")).map(node_to_text).nth(0)
}

/// Get header part of the table
pub fn get_header<'a>(table: &Node<'a>) -> Option<Vec<String>> {
    let header = match table.find(Name("thead")).nth(0) {
        // If there's a <thead> tag, get all <th> inside it.
        Some(ref head) => head
            .find(Name("tr").descendant(Name("th")))
            .map(node_to_text)
            .collect::<Vec<_>>(),
        // If there ain't <thead> tag, filter out <tr> with <td>, since they are body part.
        None => {
            match table
                .find(Name("tr"))
                .filter(|tr| tr.find(Name("td")).collect::<Vec<_>>().is_empty())
                .nth(0)
            {
                Some(tr) => tr.find(Name("th")).map(node_to_text).collect::<Vec<_>>(),
                None => vec![],
            }
        }
    };

    if header.is_empty() {
        return None;
    }
    Some(header)
}

/// Get body part of the table
pub fn get_body<'a>(table: &Node<'a>) -> Option<Vec<Row>> {
    if let Some(body) = table.find(Name("tbody")).next() {
        let body = body
            .find(Name("tr"))
            .map(|row| {
                let mut headers = row.find(Name("th")).map(node_to_text);
                let header = headers.next().map(String::from);
                let data = row.find(Name("td")).map(node_to_text).collect::<Vec<_>>();
                Row::from(header, data)
            })
            .filter(|row| !row.data().is_empty())
            .collect::<Vec<_>>();
        if body.is_empty() {
            return None;
        }
        return Some(body);
    }
    None
}

/// Get body part of the footer
pub fn get_footer<'a>(table: &Node<'a>) -> Option<Row> {
    match table.find(Name("tfoot").descendant(Name("tr"))).nth(0) {
        Some(footer) => {
            let header = footer
                .find(Name("th"))
                .map(node_to_text)
                .nth(0)
                .map(String::from);
            let data = footer
                .find(Name("td"))
                .map(node_to_text)
                .collect::<Vec<_>>();
            Some(Row::from(header, data))
        }
        None => None,
    }
}
