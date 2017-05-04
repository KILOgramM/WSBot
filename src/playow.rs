/////////
extern crate reqwest;
extern crate regex;

use std::io::Read;
use regex::Regex;

fn load_overwatch_rating(name: &str, id: &str) -> String {
    let url = &format!("https://playoverwatch.com/en-us/career/pc/eu/{}-{}", name, id);

    let mut resp = reqwest::get(url).expect("Wrong url");
    let regex = Regex::new("<div class=\"u-align-center h6\">(\\d+)</div>").unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content).expect("Rating downloading error");

    let rating = regex.captures(&content).unwrap().get(1).expect("Rating not found").as_str();

    return rating.to_string();
}
fn slicebtag(cap: &str) {
    let btag_reg = Regex::new(r"^!wsreg\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").unwrap();

    if let Some(caps) = btag_reg.captures(&message.content) {
        println!("Любое сообщение");
        let re = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").unwrap();
        println!("Передали re значение");
        for cap in re.captures_iter(&message.content) {
            let msg = ("К вам привязан батлтаг - {}.\
            Для изменения бтага или обновления ролей еще раз используйте команду !wsreg", &cap[0]);
            let _ = discord.send_message(message.channel_id, msg, "", false);
        };
    };
}

fn actualrating() {
    let name = "KILOgramM";
    let id = "2947";

    let rating = load_overwatch_rating(name, id);

    println!("{}-{} rating: {}", name, id, rating);
}


////