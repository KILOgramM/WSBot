extern crate discord;
extern crate regex;
extern crate reqwest;

use discord::Discord;
use discord::model::Event;
use regex::Regex;
use std::io::Read;


fn main() {
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token(
        "MzA4MDQ4NzQ0NzgyMzY0Njcy.C-qiOw.YG8nGwogD7FTRxNTotFhLWCr3Dg"
    ).expect("толи сервер толи токен");
    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect()
        .expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                match message.content {
                    "!help" => println!("два"), //discord.send_message (message.channel_id, "Яша не балуйся с !help", "", false),
                    _ => println!("{} says: {}", message.author.name, message.content),
                };
                if message.content == "!help" {
                    let _ = discord.send_message(
                        message.channel_id, "Яша не балуйся с !help", "", false
                    );
                    let name = "KILOgramM";
                    let id = "2947";
                    let mut resp = reqwest::get(&format!("https://playoverwatch.com/en-us/career/pc/eu/{}-{}", name, id)).unwrap();
                    let mut content = String::new();
                    if resp.status().is_success() {
                        resp.read_to_string(&mut content);
                        println!("ok? - ");
                    } else {
                        println!("error");
                    };
                } else {
                    let btag_reg = Regex::new(r"^!wsreg\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").unwrap();

                    if let Some(caps) = btag_reg.captures(&message.content) {
                        println!("Любое сообщение");
                        let re = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").unwrap();
                        println!("Передали re значение");
                        for cap in re.captures_iter(&message.content) {
                            let _ = discord.send_message(
                                message.channel_id, "Для изменения бтага или обновления ролей еще раз используйте команду !wsreg ", "", false);
                            println!("Ваш бтаг - {:?}", &cap[0]);
                        }
                    }
                }
            }
            Ok(Event::ServerMemberAdd(serverid, member)) => {
                let welcome = "Добропожаловать на сервер уважаемый";
                println!("{:?} {:?} - cthdth bl {:?}", &welcome, &member.nick, &serverid);
                break
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}
