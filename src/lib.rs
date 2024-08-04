use std::fs;

pub struct Config {
    query: String,
    search_path: String,
}

pub struct SearchResult {
    line: u16,
    content: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not found enough args");
        }

        let query = args[1].clone();
        let search_path = args[2].clone();
        let c = Config { query, search_path };

        return Ok(c);
    }
}

fn search(config: Config) -> Vec<SearchResult> {
    let contents = fs::read_to_string(config.search_path).expect("read file failed");
    let mut result: Vec<SearchResult> = vec![];
    let mut index: u16 = 1;
    for line in contents.to_lowercase().lines() {
        if line.contains(&config.query) {
            result.push(SearchResult {
                line: index,
                content: line.to_string(),
            })
        }
        index += 1;
    }

    return result;
}

fn display_search_result(result: Vec<SearchResult>) {
    for r in result {
        let line = r.line;
        let c = r.content;
        println!("line: {line}");
        println!("\t{c}");
    }
}

pub fn run(config: Config) {
    let result = search(config);
    display_search_result(result);
}
