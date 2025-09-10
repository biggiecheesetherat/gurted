mod file;
mod structs;

use colored::Colorize;
use macros_rs::fmt::{crashln, string};
use sqlx::{PgPool, Error};
use std::fs::write;
use structs::{Auth, Database, Discord, Server, Settings};

pub use structs::Config;

impl Config {
    pub fn new() -> Self {
        let default_offensive_words = vec!["nigg", "sex", "porn", "igg", "fag", "hitler"]
        let default_tld_list = vec!["com", "net", "org", "shit", "eefoc", "adrian", "67", "chat", "arsonflare", "gpt", "based", "delulu", "aura", "pmo", "sucks", "emo", "twin", "zorp", "clank", "web", "fent", "yeah", "slop", "job", "goat", "buss", "dawg", "opium", "gang", "ok", "wtf", "lol", "scam", "cat", "edge", "miku", "teto", "dumb", "balls", "yap", "attic", "ayo", "dev", "ac", "ad", "ae", "af", "ag", "ai", "al", "am", "ao", "aq", "ar", "as", "at", "au", "aw", "ax", "az", "ba", "bb", "bd", "be", "bf", "bg", "bh", "bi", "bj", "bl", "bm", "bn", "bo", "bq", "br", "bs", "bt", "bv", "bw", "by", "bz", "ca", "cc", "cd", "cf", "cg", "ch", "ci", "ck", "cl", "cm", "cn", "co", "cr", "cu", "cv", "cw", "cx", "cy", "cz", "de", "dj", "dk", "dm", "do", "dz", "ec", "ee", "eg", "eh", "er", "es", "et", "eu", "fi", "fj", "fk", "fm", "fo", "fr", "ga", "gb", "gd", "ge", "gf", "gg", "gh", "gi", "gl", "gm", "gn", "gp", "gq", "gr", "gs", "gt", "gu", "gw", "gy", "hk", "hm", "hn", "hr", "ht", "hu", "id", "ie", "il", "im", "in", "io", "iq", "ir", "is", "it", "je", "jm", "jo", "jp", "ke", "kg", "kh", "ki", "km", "kn", "kp", "kr", "kw", "ky", "kz", "la", "lb", "lc", "li", "lk", "lr", "ls", "lt", "lu", "lv", "ly", "ma", "mc", "md", "me", "mg", "mh", "mk", "ml", "mm", "mn", "mo", "mp", "mq", "mr", "ms", "mt", "mu", "mv", "mw", "mx", "my", "mz", "na", "nc", "ne", "nf", "ng", "ni", "nl", "no", "np", "nr", "nu", "nz", "om", "pa", "pe", "pf", "pg", "ph", "pk", "pl", "pm", "pn", "pr", "ps", "pt", "pw", "py", "qa", "re", "ro", "rs", "ru", "rw", "sa", "sb", "sc", "sd", "se", "sg", "sh", "si", "sj", "sk", "sl", "sm", "sn", "so", "sr", "ss", "st", "su", "sv", "sx", "sy", "sz", "tc", "td", "tf", "tg", "th", "tj", "tk", "tl", "tm", "tn", "to", "tr", "tt", "tv", "tw", "tz", "ua", "ug", "uk", "us", "uy", "uz", "va", "vc", "ve", "vg", "vi", "vn", "vu", "wf", "ws", "ye", "yt", "za", "zm", "zw"];

        Config {
            config_path: "config.toml".into(),
            server: Server {
                address: "127.0.0.1".into(),
                port: 8080,
                database: Database {
                    url: "postgresql://username:password@localhost/domains".into(),
                    max_connections: 10,
                },
                cert_path: "localhost+2.pem".into(),
                key_path: "localhost+2-key.pem".into(),
            },
            discord: Discord {
                bot_token: "".into(),
                channel_id: 0,
            },
            auth: Auth {
                jwt_secret: "your-secret-key-here".into(),
            },
            settings: Settings {
                tld_list: default_tld_list.iter().map(|s| s.to_string()).collect(),
                offensive_words: default_offensive_words.iter().map(|s| s.to_string()).collect(),
            },
        }
    }

    pub fn read(&self) -> Self { file::read(&self.config_path) }
    pub fn get_address(&self) -> String { format!("{}:{}", self.server.address.clone(), self.server.port) }
    pub fn tld_list(&self) -> Vec<&str> { self.settings.tld_list.iter().map(AsRef::as_ref).collect::<Vec<&str>>() }
    pub fn offen_words(&self) -> Vec<&str> { self.settings.offensive_words.iter().map(AsRef::as_ref).collect::<Vec<&str>>() }

    pub fn set_path(&mut self, config_path: &String) -> &mut Self {
        self.config_path = config_path.clone();
        return self;
    }

    pub fn write(&self) -> &Self {
        let contents = match toml::to_string(self) {
            Ok(contents) => contents,
            Err(err) => crashln!("Cannot parse config.\n{}", string!(err).white()),
        };

        if let Err(err) = write(&self.config_path, contents) {
            crashln!("Error writing config to {}.\n{}", self.config_path, string!(err).white())
        }

        log::info!("Created config: {}", &self.config_path,);

        return self;
    }

    pub async fn connect_to_db(&self) -> Result<PgPool, Error> {
        let pool = PgPool::connect(&self.server.database.url).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        log::info!("PostgreSQL database connected");
        Ok(pool)
    }
}
