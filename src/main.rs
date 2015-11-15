extern crate regex;

use std::env;

use regex::Regex;

mod http {
    extern crate hyper;
    use std::io::Read;

    fn get(path: &str) -> self::hyper::client::Response {
        hyper::Client::new().get(path).header(hyper::header::Connection::close()).send().unwrap()
    }

    pub fn download(path: &str) -> String {
        let mut body = String::new();

        let mut res = self::get(path);
        res.read_to_string(&mut body).unwrap();
        return body.to_string();
    }

    pub fn encode_uri(path: &str) -> String {
        hyper::Url::parse(path).unwrap().serialize()
    }
}


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 5 {
        println!("invalid args");
        return;
    }

    let anime = &args[1];
    let base_uri = &args[4];

    let content = http::download(&*format!("{}/{}", base_uri, anime));

    let reg = Regex::new("<a href=\"(.*?)\">(.*?) - ([0-9]{1,}(v[0-9]{1,}){0,1})\\.mkv</a>").unwrap();

    println!("kyou best gril");
    for cap in reg.captures_iter(&*content) {
        let ep_name = cap.at(3).unwrap_or("invalid");
        assert!(ep_name != "invalid");
        println!("{} ||| {} ||| {}", ep_name, http::encode_uri(&*format!("{}/{}/{} - {}.mkv", base_uri, anime, anime, ep_name)), format!("{} - {}.mkv", anime, ep_name));
    }
}
