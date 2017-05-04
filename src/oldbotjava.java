var fs = require('fs');
        var https = require('https');
        var cheerio = require('cheerio');
        var discordie = require('discordie');
        var srbot = new discordie({autoReconnect: true});

        var token = "Mjg2NDYxMzI2MDYwOTQ1NDA4.C5hDaQ.hJjMLabSgo7xysrXK0wjKjrI3sU",
        server = "287508326877757440",
        command = "@@sr",
        textchannel = "287508326877757440";

        roles = [
        {id : "287514615603003392",
        min : 0,
        max : 999},
        {id : "287514712118132738",
        min : 1000,
        max : 1999},
        {id : "287514758922502144",
        min : 2000,
        max : 2999},
        {id : "287514800743776256",
        min : 3000,
        max : 3999},
        {id : "287514844603613184",
        min : 4000,
        max : 9999}
        ]



        srbot.connect({ token: token });

        srbot.Dispatcher.on("GATEWAY_READY", e => {

        console.log("Fueled up, ready to go!");
        server = srbot.Guilds.get(server);

        });

        srbot.Dispatcher.on("GUILD_MEMBER_ADD", e => {
        console.log("someone joined");

        if(e.guild.id == server.id){
        srbot.Channels.get(textchannel).sendMessage("<@" + e.member.id + "> Добро пожаловать на наш сервер! Что-бы пройти дальше вам нужно ввести ваш battle.net Tag для получения вашей роли!");
        }

        });

        srbot.Dispatcher.on("MESSAGE_CREATE", e => {
        if(e.message.content.toLowerCase()=="@@help"){
        e.message.channel.sendMessage("ожалуйста введите свой Battle.net Tag для получения вашей роли.");
        }
        split = e.message.content.split(" ");
        if(split[0].toLowerCase()==command){
        btag = split[1].split("#");
        srlookup(btag[0],btag[1],e.message.member, e);
        }

        });

        function srlookup(name, id, member, e){
        var playerendpoint = '/en-us/career/pc/eu/' + name + '-'  + id;
        console.log(playerendpoint);
        var options = {
        hostname: 'playoverwatch.com',
        port: 443,
        path:  playerendpoint,
        method: 'GET'
        };

        var req = https.request(options, (res) => {
        var sr;
        if(res.statusCode == 200){
        res.on('data', (d) => {
        var $ = cheerio.load(d);
        $('.u-align-center.h6').filter(function(){
        var data = $(this);
        sr = data.text();
        console.log(sr);
        })
        });

        res.on('end', () => {
        console.log("assigning roles");
        var roleassigned = false;
        var srint = parseInt(sr);
        roles.forEach(function(role){
        if(role.min<=srint && role.max>=srint){
        roleassigned = true;
        e.message.member.assignRole(role.id);
        e.message.channel.sendMessage(":white_check_mark:");
        }
        })

        if(!roleassigned){
        e.message.channel.sendMessage("Что-то пошло не так, пожалуйста попробуйте снова."); //Sorry something went wrong, please try again
        }
        });
        }else if(res.statusCode == 404){
        e.message.channel.sendMessage("Данный Battletag не найден. Пожалуйста проверьте правильность его написания."); // No data found for that tag, please make sure you spelt it correctly
        }else{
        e.message.channel.sendMessage("Что-то пошло не так, пожалуйста попробуйте снова."); //Sorry something went wrong, please try again
        }
        console.log('statusCode:', res.statusCode);
        });

        req.on('error', (e) => {
        console.error(e);
        });
        req.end();
        }

// function rolepicker(sr){
// sr = parseInt(sr);
// roles.forEach(function(role){
//   if(role.min<=sr && role.max>=sr){
//     console.log(role.id);
//     var targetrole = server.roles.filter(function(irole){
//       return irole.id == role.id;
//     });
//     console.log(targetrole.length);
//     console.log(targetrole);
//     return targetrole;
//   }
// })
// }
