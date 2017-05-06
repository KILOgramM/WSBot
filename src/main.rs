extern crate discord;
extern crate regex;
extern crate reqwest;

use discord::Discord;
use discord::model::Event;
use discord::model::ChannelId;
use regex::Regex;
use std::io::Read;


fn load_overwatch_rating(name: &str, id: &str) -> String {
    let url = &format!("https://playoverwatch.com/en-us/career/pc/eu/{}-{}", name, id);
    let mut resp = reqwest::get(url).expect("Wrong url");
    let regex = Regex::new("<div class=\"u-align-center h6\">(\\d+)</div>").unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content).expect("Rating downloading error");
    let rating = regex.captures(&content).unwrap().get(1).expect("Rating not found").as_str();
    return rating.to_string();
}

fn actualrating(discord: &Discord, name: &str, id: &str, channel: ChannelId) {
    let rating = load_overwatch_rating(name, id);
    let acrat = format!("{}#{} ваш актуальный рейтинг: {}", name, id, rating);
    let _ = discord.send_message(channel, acrat.as_str(), "", false);
    println!("{}#{} actual rating: {}", name, id, rating);
}

fn main() {
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token("MzA4MDQ4NzQ0NzgyMzY0Njcy.C-qiOw.YG8nGwogD7FTRxNTotFhLWCr3Dg").expect("толи сервер толи токен");
    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                match message.content.as_str() {
                    "!wshelp" => {
                        let wshelp = "Много разного очень помогающего размеченного текста";
                        let _ = discord.send_message(message.channel_id, wshelp, "", false);
                    }
                    "!wsmix" => {
                        let wsmix = "Вы собрались поиграть миксы";
                        let _ = discord.send_message(message.channel_id, wsmix, "", false);
                    }
                    _ => {
                        let btag_reg = Regex::new(r"^!wsreg\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").unwrap();
                        if let Some(_) = btag_reg.captures(&message.content) {
                            let re = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").unwrap();
                            for cap in re.captures_iter(&message.content) {
                                let btmsg = format!("К вам привязан батлтаг - {}. Для изменения бтага или обновления ролей еще раз используйте команду !wsreg", &cap[0]);
                                let _ = discord.send_message(message.channel_id, &btmsg, "", false);
                                let fullbtag = cap;
                                let mut s = fullbtag.get(0).unwrap().as_str().splitn(2, '#');
                                let (name, id) = (s.next().unwrap(), s.next().unwrap());
                                println!("{}#{}", name, id);
                                actualrating(&discord, name, id, message.channel_id);
                            };
                        };
                    }
                };
            }
            Ok(Event::ServerMemberAdd(serverid, member)) => {
                let welcome = "Добропожаловать на сервер уважаемый";
                println!("{:?} {:?} - вы на планете № {:?}", &welcome, &member.nick, &serverid);
                break
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err),
        };
    };
}

