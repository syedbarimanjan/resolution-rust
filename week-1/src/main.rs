use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    descendants: u64,
}


fn api_request() -> Result<Vec<Story>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    
    let top_ids: Vec<u64> = client.get("https://hacker-news.firebaseio.com/v0/topstories.json")
    .send()?
    // .expect("Failed to fetch top stories")
    .json()?;
    // .expect("Failed to parse stroy IDs");

    let args:Vec<String> = env::args().collect();
    
    
    let mut stories: Vec<Story> = Vec::with_capacity(args[1].parse().unwrap_or(10));
    
    for (i, id) in top_ids.iter().take(args[1].parse().unwrap_or(10)).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
        
        let story: Story = client
        .get(&url)
        .send()?
        // .expect("failed to fetch stroy")
        .json()?;
    // .expect("failed to parse story");
    
        let link = story.url.as_deref().unwrap_or("(no URL)");
        println!("{}. {} ({} points by {}) ({} descendants)", i+1, story.title, story.score, story.by, story.descendants);
        println!("   {}\n", link);
        
        stories.push(story);
    }
    let _ = Err::<Vec<Story>, Box<dyn std::error::Error>>("beep boop no data found".into());
    Ok(stories)
}

fn main(){
    let api_result: Result<Vec<Story>, Box<dyn std::error::Error>> = api_request();

    match api_result {
        Ok(response) => {
            println!("Data recieved: {}", response[0].title);
            // let link = response.url.as_deref().unwrap_or("(no URL)");
            // println!("{}. {} ({} points by {})", i+1, response.title, response.score, response.by);
            // println!("   {}\n", link);
        }
        Err(error) => {
            println!("Error {:?}", error);
        }
    }
}

// struct data{
//     name: String,
//     age: u32,
// }

// fn main() {
//     println!("Hello, world!");
//     fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }
    
//     println!("{}",add(2,2));
// }

// let name = "hacker news";// immutable var meaning vaalue cant be changes.
// let mut count = 0; // mutable var
// i32 signed int(can be neagtive) u32 unsigned int(zero or positive)
// STring normal string