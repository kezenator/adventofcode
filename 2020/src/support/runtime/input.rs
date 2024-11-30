use std::collections::HashMap;
use std::time::Instant;

pub struct InputCache
{
    cookie: String,
    cache: HashMap<(usize, usize), String>,
}

impl InputCache
{
    pub fn new() -> Self
    {
        let cwd = std::env::current_dir().unwrap();
        let input_path = cwd.join("inputs");

        if !input_path.is_dir()
        {
            println!("AoC INPUT CACHE: Expect \"inputs\" directory: {:?}", input_path);
            panic!("Missing \"inputs\" dir");
        }

        let cookie_path = input_path.join("cookie");
        if !cookie_path.is_file()
        {
            println!("AoC INPUT CACHE: Expect \"inputs/cookie\" file: {:?}", cookie_path);
            panic!("Missing \"inputs/cookie\" file");
        }

        let cookie = std::fs::read_to_string(cookie_path)
            .expect("Cannot read \"inputs/cookie\"")
            .trim()
            .to_string();

        InputCache { cookie, cache: HashMap::new() }
    }

    pub fn get(&mut self, year: usize, day: usize) -> String
    {
        if let Some(value) = self.cache.get(&(year, day))
        {
            // Already cached in memory
            return value.clone();
        }

        let dir = std::env::current_dir().unwrap().join("inputs").join(format!("y{:04}", year));
        let path = dir.join(format!("d{:02}.txt", day));
        let mut value = if path.is_file()
        {
            // Already cached in the local file system
            std::fs::read_to_string(path).expect("Cannot read existing input")
        }
        else
        {
            // Need to do a HTTP GET to download the input
            let value = load(year, day, &self.cookie);

            if (!dir.is_dir())
            {
                std::fs::create_dir(dir)
                    .expect("Cannot create year directory");
            }

            std::fs::write(path, &value)
                .expect("Cannot write downloaded file to local file-system cache");

            value
        };

        // If it's only one line, then trim it
        {
            let lines = crate::input::input_to_lines(&value);
            if lines.len() == 1
            {
                value = lines[0].clone();
            }
        }

        self.cache.insert((year, day), value.clone());
        value
    }
}

fn load(year: usize, day: usize, cookie: &str) -> String
{
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    println!("[ {:11} ] [             ] => [ {} ]", "Downloading", url);

    let start = Instant::now();

    let client = reqwest::blocking::Client::new();

    let request = client.get(url)
        .header(reqwest::header::COOKIE, cookie)
        .build()
        .expect("Cannot build request");

    let response = client.execute(request)
        .expect("Cannot get response");

    if response.status() != reqwest::StatusCode::OK
    {
        println!("   AoC INPUT CACHE: Status={}", response.status().as_str());
        println!("   Ensure the correct cookie has been installed in file \"{:?}\"",
            std::env::current_dir().unwrap().join("inputs/cookie"));
        panic!("Cannot download input");
    }

    let body = response.text()
        .expect("Cannot get response body");

    let duration = Instant::now().duration_since(start);

    println!("[ {:11} ] [ {:3}.{:06} s] => [ {:14} bytes ]",
            "Downloaded",
            duration.as_secs(),
            duration.subsec_micros(),
            body.as_bytes().len());

    body
}
