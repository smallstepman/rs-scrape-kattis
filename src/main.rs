use scraper::{Html, Selector};
use std::collections::HashMap;

async fn scrape(problem_id: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let url = format!("https://open.kattis.com/problems/{}", problem_id);
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let table_selector = Selector::parse("table").unwrap();
    let pre_selector = Selector::parse("pre").unwrap();
    let mut test_cases: HashMap<String, String> = HashMap::new();

    for element in document.select(&table_selector) {
        let mut pre = element.select(&pre_selector);
        let input = pre.next().unwrap().inner_html().clone();
        let output = pre.next().unwrap().inner_html().clone();
        test_cases.insert(input.trim().to_string(), output.trim().to_string());
        println!("{:?}", test_cases);
    }

    Ok(test_cases)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = scrape(String::from("thelastproblem")).await?;
    for t in TestEngine::new(solution, test_cases) {
        println!("{}", t);
    }
    Ok(())
}

struct TestEngine {
    func: fn(String) -> String,
    test_cases: HashMap<String, String>,
}

impl TestEngine {
    fn new(func: fn(String) -> String, test_cases: HashMap<String, String>) -> Self {
        TestEngine { func, test_cases }
    }
}

impl Iterator for TestEngine {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let (k, v) = self.test_cases.iter().next().unwrap();
        let result = (self.func)(k.clone()) == *v;
        let k = k.clone();
        let v = 3;
        self.test_cases.remove(&k);
        Some(result)
    }
}

fn solution(s: String) -> String {
    format!("Thank you, {}, and farewell!", s)
}
