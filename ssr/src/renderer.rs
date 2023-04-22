use super::*;

use std::collections::HashMap;

pub async fn render(url: String, queries: HashMap<String, String>) -> String {
    let renderer =
        yew::ServerRenderer::<client::ServerApp>::with_props(move || client::ServerAppProps {
            url: url.into(),
            queries,
        });

    let generated = generate();

    let index_html_s = std::str::from_utf8(
        generated
            .get("index.html")
            .expect("index.html not found")
            .data,
    )
    .expect("index.html could not be parsed");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let index_html_before = index_html_before.clone();
    let index_html_after = index_html_after.clone();

    let mut html = String::new();

    html.push_str(&index_html_before);
    html.push_str(&renderer.render().await);
    html.push_str(&index_html_after);

    html
}
