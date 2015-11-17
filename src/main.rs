extern crate regex;

use std::env;

use regex::Regex;

mod http {
    extern crate hyper;
    use std::io::Read;

    fn get(path: &str) -> Result<self::hyper::client::response::Response, self::hyper::error::Error> {
        hyper::Client::new().get(path).header(hyper::header::Connection::close()).send()
    }

    pub fn download(path: &str, content: &mut String) -> Result<(), String> {
        let mut res = match self::get(path) {
            Ok(res) => res,
            Err(..) => return Err("could not download file".to_string()),
        };
        match res.read_to_string(content) {
            Err(..) => Err("could not read content".to_string()),
            Ok(..) => Ok(()),
        }
    }

    pub fn encode_uri(path: &str) -> String {
        hyper::Url::parse(path).unwrap().serialize()
    }
}


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 5 {
        panic!("invalid args");
    }

    let anime = &args[1];
    let base_uri = &args[4];
    let mut content = String::new();
    if let Err(..) = http::download(&format!("{}/{}", base_uri, anime), &mut content) {
        panic!("could not download file");
    }

    let reg = Regex::new("<a href=\"(.*?)\">(.*?) - ([0-9]{1,}(v[0-9]{1,}){0,1})\\.mkv</a>").unwrap();

    println!("http search");
    println!("kyou best gril");
    for cap in reg.captures_iter(&content) {
        let ep_name = cap.at(3).unwrap_or("?");
        println!("{} ||| {} ||| {}", ep_name, http::encode_uri(&format!("{}/{}/{} - {}.mkv", base_uri, anime, anime, ep_name)), format!("{} - {}.mkv", anime, ep_name));
    }
}
