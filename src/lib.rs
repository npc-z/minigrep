use std::fs;

pub struct Config {
    query: String,
    search_path: String,
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

pub fn run(config: Config) {
    let contents = fs::read_to_string(config.search_path).expect("read file failed");
    let result = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&config.query));

    for r in result {
        println!("found: {r}");
    }
}
