pub mod utils {
    use std::process::Command;

    pub fn perform_search(browser: String, url: String, incognito: String) {
        let browser_executable = if browser == "chrome" {
            "google-chrome-stable".to_string()
        } else {
            browser
        };
        if incognito.is_empty() {
            Command::new(browser_executable)
                .arg(url)
                .spawn()
                .expect("Failed to open browser");
        } else {
            Command::new(browser_executable)
                .args(&[incognito, url])
                .spawn()
                .expect("Failed to open browser");
        }
    }
    pub fn build_search_url(search_engine: String, query: String) -> String {
        search_engine + query.as_str()
    }

    pub fn get_search_engine(search_option: Option<String>) -> String {
        match search_option.unwrap_or("".to_string()).as_str() {
            "ddg" => "https://duckduckgo.com/?q=".to_string(),
            "bing" => "https://www.bing.com/search?q=".to_string(),
            "ud" => "https://www.urbandictionary.com/define.php?term=".to_string(),
            "wiki" => "https://en.wikipedia.org/w/index.php?search=".to_string(),
            "walmart" => "https://www.walmart.ca/search?q=".to_string(),
            _ => "https://www.google.com/search?q=".to_string(),
        }
    }
}
