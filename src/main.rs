extern crate discord;
extern crate regex;
extern crate reqwest;
//extern crate mysql;
//VH14.spaceweb.ru
use discord::{Discord, State};
use discord::model::Event;
use regex::Regex;
use std::io::Read;


// Элемент очереди
struct Player {
    did: String,
    name: String,
    disc: String,
    btag: String,
    rtg: String,
    mixrdy: bool,
    rtgrdy: bool,
    arcrdy: bool,
    h: bool, // Хил?
    d: bool, // Дамагер?
    t: bool, // Танк?
    inv: String,
}

impl Player {
    fn new(did: &str, name: &str, disc: &str, btag: &str, rtg: &str, mixrdy: bool, rtgrdy: bool, arcrdy: bool, h: bool, d: bool, t: bool, inv: &str) -> Player {
        Player {
            did: did.to_string(),
            name: name.to_string(),
            disc: disc.to_string(),
            btag: btag.to_string(),
            rtg: rtg.to_string(),
            mixrdy: mixrdy,
            rtgrdy: rtgrdy,
            arcrdy: arcrdy,
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

//fn identify_or_create_my_channel(discord: &Discord, server: LiveServer) -> Result<Channel> {
//    for channel in server.channels.into_iter() {
//        if &channel.name == MY_CHANNEL_NAME && channel.kind == ChannelType::Text {
//            return Ok(Channel::Public(channel));
//        }
//    }
//discord.create_channel(&server.id, MY_CHANNEL_NAME, ChannelType::Text)
//};





fn load_overwatch_rating(btag: &str) -> String {
    let mut s = btag.splitn(2, '#');
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
    let mut result_rtmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
    let mut result_mixmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
    let mut result_playmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
    let btag_reg = Regex::new(r"^!wsreg\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").expect("не найдена команда !wsreg btag"); //форма среза текста "!wsreg battletag#123"
    let btag_new = Regex::new(r"^!wsbt\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").expect("не найдена команда !wsbt btag"); //форма среза текста "!wsbt battletag#123"
    let bt = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").expect("не найден баттл таг");//форма среза текста "battletag#123"
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token("MzA4MDQ4NzQ0NzgyMzY0Njcy.C_AEnQ.OOXAryqsK0YEBOSdHpBAV78KWOs").expect("толи сервер толи токен");
    // Establish and use a websocket connection
    let (mut connection, ready) = discord.connect().expect("connect failed");
    println!("Ready.");
    let state = State::new(ready);
    let botdiscordid = format!("{}", state.user().id);
    let mut list = Vec::<Player>::new();
    //let disc = format!("{}", state.user.discriminator);
    let newplayer = Player::new(&botdiscordid, "", "", "", "", false, false, false, true, true, true, "");
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
                                i.mixrdy = true;
                                i.rtgrdy = false;
                                i.arcrdy = false;
                                let gomsg = format!("Игрок {} встал в очередь для поиска миксов", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &gomsg, "", false);
                            }
                        };
                    }
                    "!wsmixstop" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.mixrdy = false;
                                let stopmsg = format!("Игрок {} вышел из очереди для поиска миксов", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &stopmsg, "", false);
                            }
                        };
                    }
                    "!wsmixlist" => {
                        let _ = discord.send_message(message.channel_id, "__**Список игроков которые ищут микс**__", "", false);
                        for i in list.iter_mut() {
                            if i.mixrdy == true {
                                println!("игрок");
                                result_mixmsg.push_str(&format!("| {}#{} | {} | {} |\n", i.name, i.disc, i.btag, i.rtg));
                            };
                        };
                        result_mixmsg.push_str(&format!("```"));
                        let _ = discord.send_message(message.channel_id, &result_mixmsg, "", false);
                        let mut result_mixmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
                    }
                    "!wsrtgo" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.mixrdy = false;
                                i.rtgrdy = true;
                                i.arcrdy = false;
                                let gomsg = format!("Игрок {} встал в очередь для поиска совместной игры в рейтинг", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &gomsg, "", false);
                            }
                        };
                    }
                    "!wsrtstop" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.rtgrdy = false;
                                let stopmsg = format!("Игрок {} вышел из очереди совместной игры в рейтинг", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &stopmsg, "", false);
                            }
                        };
                    }
                    "!wsrtlist" => {
                        let _ = discord.send_message(message.channel_id, "__**Список игроков которые ищут c кем бы поиграть рейтинг**__", "", false);
                        let _ = discord.send_message(message.channel_id, "```Discord tag | Battletag | Актуальный рейтинг```", "", false);
                        for i in list.iter_mut() {
                            if i.rtgrdy == true {
                                println!("игрок");
                                result_rtmsg.push_str(&format!("| {}#{} | {} | {} |\n", i.name, i.disc, i.btag, i.rtg))
                            };
                        };
                        result_rtmsg.push_str(&format!("```"));
                        let _ = discord.send_message(message.channel_id, &result_rtmsg, "", false);
                        let mut result_rtmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
                    }
                    "!wsplaygo" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.mixrdy = false;
                                i.rtgrdy = false;
                                i.arcrdy = true;
                                let gomsg = format!("Игрок {} встал в очередь для просто поиграть. Быстрые катки, аркады. ", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &gomsg, "", false);
                            }
                        };
                    }
                    "!wsplaystop" => {
                        for i in list.iter_mut() {
                            if i.name == message.author.name {
                                i.arcrdy = false;
                                let stopmsg = format!("Игрок {} просто наигрался =)", &message.author.name);
                                let _ = discord.send_message(message.channel_id, &stopmsg, "", false);
                            }
                        };
                    }
                    "!wsplaylist" => {
                        let _ = discord.send_message(message.channel_id, "__**Список игроков которые ищут c кем бы поиграть рейтинг**__", "", false);
                        for i in list.iter_mut() {
                            if i.rtgrdy == true {
                                println!("игрок");
                                result_rtmsg.push_str(&format!("| {}#{} | {} | {} |\n", i.name, i.disc, i.btag, i.rtg))
                            };
                        };
                        result_rtmsg.push_str(&format!("```"));
                        let _ = discord.send_message(message.channel_id, &result_rtmsg, "", false);
                        let mut result_rtmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
                        let _ = discord.send_message(message.channel_id, "__**Список игроков которые ищут c кем бы поиграть микс**__", "", false);
                        for i in list.iter_mut() {
                            if i.mixrdy == true {
                                println!("игрок");
                                result_mixmsg.push_str(&format!("| {}#{} | {} | {} |\n", i.name, i.disc, i.btag, i.rtg))
                            };
                        };
                        result_mixmsg.push_str(&format!("```"));
                        let _ = discord.send_message(message.channel_id, &result_mixmsg, "", false);
                        let mut result_mixmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
                        let _ = discord.send_message(message.channel_id, "__**Список игроков которые ищут c кем бы поиграть**__", "", false);
                        for i in list.iter_mut() {
                            if i.arcrdy == true {
                                println!("игрок");
                                result_playmsg.push_str(&format!("| {}#{} | {} | {} |\n", i.name, i.disc, i.btag, i.rtg))
                            };
                        };
                        result_playmsg.push_str(&format!("```"));
                        let _ = discord.send_message(message.channel_id, &result_playmsg, "", false);
                        let mut result_playmsg = "```| Discord tag | Battletag | Актуальный рейтинг |\n".to_string();
                    }
 //                   "!wsmixroom" => {
 //                       let (sid, cid) = state.find_voice_user(message.author.id);
 //                   }
                    _ => {
                        if let Some(_) = btag_reg.captures(&message.content) {
                            println!("Определен");
                            'try: for btag in bt.captures_iter(&message.content) {
                                let did = format!("{}", message.author.id);
                                for i in list.iter_mut() {
                                    if i.name == message.author.name {
                                            let didmsg = format!("Игрок с именем {} уже зарегистрирован", message.author.name);
                                            let _ = discord.send_message(message.channel_id, &didmsg, "", false);
                                            let hz = format!("{}", &btag[0]);
                                            i.btag = hz;
                                            let gomsg = format!("К игроку {} привязан новый батлтаг - {}", message.author.name, &btag[0]);
                                            let _ = discord.send_message(message.channel_id, &gomsg, "", false);
                                            let rating = load_overwatch_rating(&btag[0]);
                                            i.rtg = rating.to_string();
                                            let acrat = format!("Ваш актуальный рейтинг: {}", &rating);
                                            let _ = discord.send_message(message.channel_id, &acrat, "", false);
                                        break 'try;
                                        }
                                    };
                                let btmsg = format!("К игроку {} привязан батлтаг - {}", message.author.name, &btag[0]);
                                let _ = discord.send_message(message.channel_id, &btmsg, "", false);
                                let rating = load_overwatch_rating(&btag[0]);
                                let rt = rating.to_string();
                                let disc = format!("{}", message.author.discriminator);
                                let newplayer = Player::new(&did, message.author.name.as_str(), &disc, &btag[0], &rt, false, false, false, true, true, true, "");
                                list.push(newplayer);
                                let acrat = format!("Ваш актуальный рейтинг: {}", &rating);
                                let _ = discord.send_message(message.channel_id, &acrat, "", false);
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

