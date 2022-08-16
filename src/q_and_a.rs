pub mod answer{

    use requestty::questions;
    use requestty::Answers;

    pub fn get_answer() -> Answers {
        let question = questions![
            RawSelect {
                name: "Browser",
                message: "Which browser would you like to use?",
                choices: ["Default", separator, "Firefox", "Chrome", "Chromium",],
            },
            RawSelect {
                name: "Search Engine",
                message: "Which search engine would you like to use?",
                choices: [
                    "Default",
                    separator,
                    "Google",
                    "Bing",
                    "DuckDuckGo",
                    "Urban Dictionary",
                    "Wikipedia",
                    "Walmart",
                    "Netflix",
                    "Youtube",
                    "Amazon",
                    "Ebay",
                    "Reddit",
                    "Prime",
                    "Spotify",
                ]
            },
            Input {
                name: "Search Term",
                message: "What would you like to search for?",
            },
            Confirm {
                name: "Private Window",
                message: "Would you like to open the search in a private window?",
            },
        ];
        requestty::prompt(question).ok().unwrap()
    }
}
