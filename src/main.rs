mod args;
mod browser_utils;
mod search_utils;

use args::args::parse_args;
use browser_utils::utils::{get_browser, get_private_flag};
use search_utils::utils::{build_search_url, get_search_engine, perform_search};

fn main() {
    let args = parse_args();
    let query: String = args.query.join(" ");
    let browser = get_browser(args.browser);
    let search_engine = get_search_engine(args.search_engine);
    let url = build_search_url(search_engine, query);
    let incognito: String = if args.private {
        get_private_flag(&browser)
    } else {
        String::new()
    };

    perform_search(browser, url, incognito);
}
