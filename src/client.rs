use reqwest::Client;

struct Apiclient {
    reqwest:String,
    base_url:String,
}

impl Apiclient {
   fn new(base_url: impl Into<String>) -> Self {
    Apiclient {
        client: Client::new()
        base_url: base_url.into(), 
    }
   }
    
}
fn main () {
    let api = Apiclient::new("https://api.example.com");
    println!("Base url: {}",api.base_url);
}