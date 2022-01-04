use scraper::{Html, Selector};
use std::collections::HashMap;

fn solution(s: String) -> String {
    format!("Thank you, {}, and farewell!", s)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for t in TestEngine::new(solution, "thelastproblem".to_string()).await {
        println!("{}", t);
    }
    Ok(())
}

struct TestEngine {
    platform: String,
    problem_id: String,
    func: fn(String) -> String,
    test_cases: HashMap<String, String>,
}

impl TestEngine {
    async fn new(func: fn(String) -> String, problem_id: String) -> Self {
        let test_cases = Self::scrape_test_cases(problem_id.clone()).await.unwrap();
        TestEngine {
            platform: String::from("kattis"),
            problem_id,
            func,
            test_cases,
        }
    }

    async fn scrape_test_cases(
        problem_id: String,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
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
}

impl Iterator for TestEngine {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        match self.test_cases.iter().next() {
            Some((k, v)) => {
                let result = (self.func)(k.clone()) == *v;
                let k = k.clone();
                self.test_cases.remove(&k);
                Some(result)
            }
            None => None,
        }
    }
}
