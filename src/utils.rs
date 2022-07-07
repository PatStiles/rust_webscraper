use reqwest::Client;

//user agent name is defined as str built from package name and package version as the user client
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

//way for websites to identify our HTTP client
pub fn get_client() -> Client {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    client
}
