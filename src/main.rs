extern crate discord;
extern crate regex;
extern crate reqwest;
//extern crate mysql;
//VH14.spaceweb.ru
use discord::Discord;
use discord::model::Event;
use regex::Regex;
use std::io::Read;

// Элемент очереди
struct Player {
    name: String,
    btag: String,
    rtg: String,
    rdy: bool,
    h: bool, // Хил?
    d: bool, // Дамагер?
    t: bool, // Танк?
    inv: String,
}

impl Player {
    fn new(name: &str, btag: &str, rtg: &str, rdy: bool, h: bool, d: bool, t: bool, inv: &str) -> Player {
        Player {
            name: name.to_string(),
            btag: btag.to_string(),
            rtg: rtg.to_string(),
            rdy: rdy,
            h: h,
            d: d,
            t: t,
            inv: inv.to_string(),
        }
    }
}

//fn startmix(name: &str) {
//    println!("{} заехал в очередь. В очереди людей", name);
//}

//fn stopmix(name: &str) {
//    println!("{} вышел из очереди", name);
//}


fn load_overwatch_rating(btag: &str) -> String {
    let mut s = btag.get(0).unwrap().as_str().splitn(2, '#');
    let (name, id) = (s.next().unwrap(), s.next().unwrap());
    let url = &format!("https://playoverwatch.com/en-us/career/pc/eu/{}-{}", name, id);
    println!("сам урл есть - {}", &url);
    let mut resp = reqwest::get(url).expect("Wrong url");
    println!("Запрос УРЛ успешен");
    let regex = Regex::new("<div class=\"u-align-center h6\">(\\d+)</div>").expect("не удалось найти строчку с рейтнгом");
    println!("Форма регекса сделана");
    let mut content = String::new();
    println!("новая строка?");
    resp.read_to_string(&mut content).expect("Rating downloading error");
    println!("весь контент страницы в строке?");
    let result = regex.captures(&content);
    if result.is_none() {
        return "Valera#228".to_string();
    };
    let rating = result.unwrap().get(1).expect("Rating not found").as_str();
    println!("нашли б таг в строке");
    return rating.to_string();
}

fn main() {
    let btag_reg = Regex::new(r"^!wsreg\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").expect("не найдена команда !wsreg btag"); //форма среза текста "!wsreg battletag#123"
    let re = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").expect("не найден баттл таг");//форма среза текста "battletag#123"
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token("MzA4MDQ4NzQ0NzgyMzY0Njcy.C_AEnQ.OOXAryqsK0YEBOSdHpBAV78KWOs").expect("толи сервер толи токен");
    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    let mut list = Vec::<Player>::new();
    let newplayer = Player::new("", "", "", false, true, true, true, "");
    list.push(newplayer);
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                match message.content.as_str() {

                    "!wshelp" => {
                        let wshelp = "Отчаяние =). Введите !wscmd KILOgramM достал уже все время перезагружать и обнулять списки";
                        let _ = discord.send_message(message.channel_id, wshelp, "", false);
                    }
                    "!wstake" => {
                        let _ = discord.send_message(message.channel_id, "https://discordapp.com/oauth2/authorize?&client_id=308048744782364672&scope=bot&permissions=0", "", false);
                    }
                    "!wsreg" => {
                        let wsreg = "Введите команду !wsreg вместе с батлтагом. Например: !wsreg Valera#228";
                        let _ = discord.send_message(message.channel_id, wsreg, "", false);
                    }
                    "!wscmd" => {
                        let wscmd = include_str!("cmd.ws");
                        let _ = discord.send_message(message.channel_id, wscmd, "", false);
                    }
                    "!wsmix" => {
                        let wsmix = "Вы собрались поиграть миксы?";
                        let _ = discord.send_message(message.channel_id, wsmix, "", false);
                    }
                    "!wsmixgo" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.rdy = true;
                                let gomsg = format!("Игрок {} встал в очередь для поиска миксов", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &gomsg, "", false);}
                            };
                    }
                    "!wsmixstop" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.rdy = false;
                                let stopmsg = format!("Игрок {} вышел из очереди для поиска миксов", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &stopmsg, "", false);}
                        };
                    }
                    "!wsmixlist" => {
                        let _ = discord.send_message(message.channel_id, "__**Список игроков которые ищут микс**__", "", false);
                        for j in list.iter() {
                            if j.rdy == true {
                                println!("игрок");
                                let listmsg = format!("{} | battletag - {} | рейтинг - {}", j.name, j.btag, j.rtg);
                                let _ = discord.send_message(message.channel_id, &listmsg, "", false);
                            };
                            };
                        }
                    _ => {
                        if let Some(_) = btag_reg.captures(&message.content) {
                            println!("Определен");
                            for btag in re.captures_iter(&message.content) {
                                let bt = format!("{}", &btag);
                                let newwplayer = Player::new(message.author.name.as_str(), &bt, "", false, true, true, true, "");
                                list.push(newwplayer);
                                let btmsg = format!("К игроку {} привязан батлтаг - {}", message.author, &btag[0]);
                                let _ = discord.send_message(message.channel_id, &btmsg, "", false);
                                let rating = load_overwatch_rating(&btag);
                                for i in list.iter_mut() {
                                    if i.name == message.author.name {
                                        i.rtg = &rating;
                                        let acrat = format!("Ваш актуальный рейтинг: {}", rating);
                                        let _ = discord.send_message(message.channel_id, &acrat, "", false);
                                    };
                                }

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
            Err(err) => {
                println!("Receive error: {:?}", err);
            }
        };
    };
}

