use anyhow::Result;
use nanohtml2text::html2text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    content: String,
    url: String,
    id: i64,
}

pub fn standard_site_name(site_address: &str) -> Result<&str> {
    let standard_name = site_address
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');

    Ok(standard_name)
}

pub async fn send_request_to_wordpress(wordpress_site: &str, plain_text: bool) -> Result<()> {
    let wordpress_site_name = standard_site_name(wordpress_site)?;

    let mut page = 1;

    loop {
        let url = format!("{}/wp-json/wp/v2/posts/?page={}", wordpress_site, page);

        print!("{:?}", page);

        let server_response = reqwest::get(&url).await?;
        if server_response.status() != 200 {
            break;
        }
        let response_text = server_response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        let number_of_posts = response_json.as_array().unwrap().iter().count();

        for i in 0..number_of_posts {
            let post_id = response_json[i]["id"].as_i64().unwrap();
            let post_url = format!("{}/?p={post_id}", wordpress_site_name.replace('_', "."));
            let post_title = response_json[i]["title"]["rendered"].as_str().unwrap();

            let post_in_html = response_json[i]["content"]["rendered"].as_str().unwrap();

            let content = if plain_text {
                html2text(post_in_html)
            } else {
                post_in_html.to_string()
            };

            let post = Post {
                title: post_title.to_string(),
                content,
                url: post_url,
                id: post_id,
            };
            println!("{post:?}");
        }
        page += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_site_name() {
        let site_address = "https://www.example.com/";
        let standard_name = standard_site_name(site_address).unwrap();
        assert_eq!(standard_name, "www.example.com");
    }
}
