use anyhow::Result;
use nanohtml2text::html2text;
use std::io::Write;

use crate::db::models::*;
use crate::{db::database::establish_connection, Config};
use diesel::RunQueryDsl;

pub fn standard_site_name(site_address: &str) -> Result<&str> {
    let standard_name = site_address
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');

    Ok(standard_name)
}

pub async fn send_request_to_wordpress(config: Config) -> Result<()> {
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();
    let wordpress_site_name = standard_site_name(config.sitename.as_str())?;

    let mut page = 1;

    loop {
        let url = format!(
            "{}/wp-json/wp/v2/posts/?page={}",
            config.sitename.as_str(),
            page
        );

        let server_response = reqwest::get(&url).await?;
        if page == 1 && server_response.status() == 404 {
            writeln!(stdout_lock, "REST API is not enable on this site")?;
            break;
        }
        if server_response.status() != 200 {
            break;
        }
        let response_text = server_response.text().await?;
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

        let number_of_posts = response_json.as_array().unwrap().iter().count();

        for i in 0..number_of_posts {
            let post_id = response_json[i]["id"].as_i64().unwrap();
            let post_url = format!("{}/?p={post_id}", wordpress_site_name);
            let post_title = response_json[i]["title"]["rendered"].as_str().unwrap();

            let post_in_html = response_json[i]["content"]["rendered"].as_str().unwrap();

            let content = if config.plain_text {
                html2text(post_in_html)
            } else {
                post_in_html.to_string()
            };

            let post = NewPost {
                title: post_title,
                content: content.as_str(),
                url: post_url.as_str(),
            };

            if config.insert_to_db {
                // Save post to database
                use crate::db::schema::posts::dsl::*;

                let connect = &mut establish_connection();

                let _ = diesel::insert_into(posts)
                    .values(&post)
                    .on_conflict_do_nothing()
                    .execute(connect);
            }

            writeln!(stdout_lock, "{:?}", post)?;
        }
        page += 1;
        if number_of_posts == 0 {
            break;
        }
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
