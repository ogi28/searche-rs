pub mod utils {
    use std::process::Command;
    pub fn get_private_flag(browser: &String) -> String {
        match browser.as_str() {
            "chrome" => "--incognito".to_string(),
            "opera" => "--private".to_string(),
            "firefox" => "--private-window".to_string(),
            "chromium" => "--incognito".to_string(),
            _ => String::new(),
        }
    }

    pub fn get_browser(browser_options: Option<String>) -> String {
        match std::env::consts::OS {
            "linux" => {
                if let Some(browser) = browser_options {
                    browser
                } else {
                    get_linux_default_browser()
                }
            }
            _ => panic!("Unsupported OS"),
        }
    }

    pub fn get_linux_default_browser() -> String {
        let cmd = Command::new("xdg-settings")
            .arg("get")
            .arg("default-web-browser")
            .output()
            .expect("Failed to get default browser");

        let period_index: usize = cmd.stdout.iter().position(|&c| c == b'.').unwrap();
        String::from_utf8(cmd.stdout).unwrap().trim()[..period_index].to_string()
    }
}
