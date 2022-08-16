mod args;
mod browser_utils;
mod q_and_a;
mod search_utils;

use args::args::parse_args;
use browser_utils::utils::{get_browser, get_private_flag};
use q_and_a::answer::get_answer;
use search_utils::utils::{build_search_url, get_search_engine, perform_search};
use std::collections::HashMap;

fn main() {
    let search_details = if std::env::args().count() < 2 {
        search_with_prompt()
    } else {
        search_with_args()
    };

    perform_search(
        &search_details["browser"],
        &search_details["url"],
        &search_details["incognito"],
    );
}

fn search_with_prompt() -> HashMap<&'static str, String> {
    let answer = get_answer();
    let browser_answer = answer.get("Browser").unwrap().as_list_item().unwrap();
    let search_engine_answer = answer.get("Search Engine").unwrap().as_list_item().unwrap();
    let search_term_answer = answer.get("Search Term").unwrap().as_string().unwrap();

    let query = search_term_answer.to_string();
    let browser = get_browser(Some(browser_answer.text.clone().to_lowercase()));
    let search_engine = get_search_engine(Some(search_engine_answer.text.clone().to_lowercase()));
    let private_window = answer.get("Private Window").unwrap().as_bool().unwrap();
    let incognito: String = if private_window {
        get_private_flag(&browser)
    } else {
        String::new()
    };

    HashMap::from([
        ("url", build_search_url(search_engine, query)),
        ("incognito", incognito),
        ("browser", browser),
    ])
}

fn search_with_args() -> HashMap<&'static str, String> {
    let args = parse_args();

    let query: String = args.query.join(" ");
    let browser = get_browser(args.browser);
    let search_engine = get_search_engine(args.search_engine);
    let incognito: String = if args.private {
        get_private_flag(&browser)
    } else {
        String::new()
    };
    HashMap::from([
        ("url", build_search_url(search_engine, query)),
        ("incognito", incognito),
        ("browser", browser),
    ])
}
