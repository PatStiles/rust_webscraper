use chrono;
use regex::Regex;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

mod models;
mod utils;

fn save_raw_html(raw_html: &str, domain_name: &str) {
    let dt = chrono::Local::now();
    let filename = format!("{}_{}.html", domain_name, dt.format("%Y-%m-%d_%H.%M.%S"));
    let mut writer = File::create(&filename).unwrap();
    write!(&mut writer, "{}", &raw_html).unwrap();
}

fn save_article_list(article_list: &Vec<models::ArticleData>, domain_name: &str) {
    let dt = chrono::Local::now();
    let filename = format!("{}_{}.json", domain_name, dt.format("%Y-%m-%d_%H.%M.%S"));
    let mut writer = File::create(&filename).unwrap();
    write!(
        &mut writer,
        "{}",
        &serde_json::to_string(&article_list).unwrap()
    )
    .unwrap();
}
#[tokio::main]
async fn main() {
    //create client instance
    let client = utils::get_client();
    let domain_name = "finance.yahoo.com";
    let url = format!("https://{}", domain_name);
    //send GET request to yahoo finance return RESPONSE Object
    let result = client.get(url).send().await.unwrap();
    let raw_html = match result.status() {
        //check if status code waws OK or if we got an error
        //If OK, retrieve text as String by calling text() function
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    save_raw_html(&raw_html, domain_name);

    let document = Html::parse_document(&raw_html);
    //selector instance for a tag
    //a indicate the tag that should be selected for dot . means that what comes after is a class
    let article_selector = Selector::parse("a.js-content-viewer").unwrap();
    let h2select = Selector::parse("h2").unwrap();
    let h3select = Selector::parse("h3").unwrap();
    //regex statements
    //gets anything matchain -> then any number of characters with .* ending on <
    let get_text_re = Regex::new(r"->.*<").unwrap();
    //removes any -> and < parts
    let find_replace_re = Regex::new(r"[-><]").unwrap();
    //Article list
    let mut article_list: Vec<models::ArticleData> = Vec::new();
    //loop through selected elements by passing ref to .select() which produces a list of ElementRef instances if match selector criteria
    for element in document.select(&article_selector) {
        //get elements inner HTML on line 21
        //function results in a String containing the HTML between the opening and closing tags
        let inner = element.inner_html().to_string();
        //call select on element object
        let mut h2el = element.select(&h2select);
        let mut h3el = element.select(&h3select);
        //uses match to make sure it can deal with errors in case it doesn't have href attribute
        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };

        //.next() returns first matching element
        //if found print result
        match h2el.next() {
            Some(elref) => {
                let title = elref.inner_html().to_string();
                println!("H2: {}", &elref.inner_html().to_string());
                println!("Link: {}", &href);

                article_list.push(models::ArticleData {
                    article_title: title,
                    url_link: href.to_string(),
                    domain_name: domain_name.to_string(),
                });
                continue;
            }
            _ => {}
        }

        match h3el.next() {
            Some(elref) => {
                let title = elref.inner_html().to_string();
                println!("H3: {}", &elref.inner_html().to_string());
                println!("Link: {}", &href);

                article_list.push(models::ArticleData {
                    article_title: title,
                    url_link: href.to_string(),
                    domain_name: domain_name.to_string(),
                });
                continue;
            }
            _ => {}
        }

        //get first match of regex with .next()
        match get_text_re.captures_iter(&inner).next() {
            Some(cap) => {
                let replaced = find_replace_re.replace_all(&cap[0], "");
                println!("Regex: {}", &replaced);
                println!("Link: {}", &href);

                article_list.push(models::ArticleData {
                    article_title: replaced.to_string(),
                    url_link: href.to_string(),
                    domain_name: domain_name.to_string(),
                });
            }
            _ => {
                println!("Nothing found");
            }
        }
    }

    println!("Number of articles titles scraped: {}", article_list.len());
    save_article_list(&article_list, &domain_name);
}
