pub mod args {
    use clap::Parser;
    /// Simple program to search things online
    #[derive(Parser, Debug)]
    pub struct Args {
        /// The search query
        pub query: Vec<String>,

        /// The browser to use, defaults to your default browser
        #[clap(short, long, possible_values = &["chrome", "firefox", "chromium", "opera", "safari"])]
        pub browser: Option<String>,

        /// The search engine to use, defaults to Google
        #[clap(short, long, possible_values = &["google", "bing", "ddg", "ud", "wiki", "walmart", "netflix", "youtube", "amazon", "ebay", "reddit", "prime", "spotify"])]
        pub search_engine: Option<String>,

        /// Whether to use incognito mode, defaults to false
        #[clap(short, long)]
        pub private: bool,
    }
    pub fn parse_args() -> Args {
        Args::parse()
    }
}
