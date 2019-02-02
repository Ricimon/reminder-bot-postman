#[macro_use] extern crate mysql;

extern crate dotenv;
extern crate reqwest;
extern crate threadpool;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use std::env;
use std::thread;
use std::time::Duration;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    content: String,
    embed: Option<Embed>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    content: String,
    username: String,
    avatar_url: String,
    embeds: Vec<Embed>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    description: String,
    color: u32,
}

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").unwrap();
    let sql_url = env::var("SQL_URL").unwrap();
    let interval = env::var("INTERVAL").unwrap().parse::<u64>().unwrap();
    let threads = env::var("THREADS").unwrap().parse::<usize>().unwrap();

    const URL: &str = "https://discordapp.com/api/v6";

    let mysql_conn = mysql::Pool::new(sql_url).unwrap();
    let req_client = reqwest::Client::new();
    let pool = threadpool::ThreadPool::new(threads);

    loop {
        pool.join();

        let mut my = mysql_conn.get_conn().unwrap().unwrap();
        let q = my.query("SELECT id, message, channel, time, position, webhook, username, avatar, embed, UNIX_TIMESTAMP() FROM reminders WHERE time < UNIX_TIMESTAMP() AND time >= 0").unwrap();

        for res in q {
            let (id, mut message, channel, mut time, position, webhook, username, avatar, color, seconds) = mysql::from_row::<(u32, String, u64, u64, Option<u8>, Option<String>, String, String, Option<u32>, u64)>(res.unwrap());

            let mut req;

            if let Some(url) = webhook {
                let mut m;

                if let Some(color_int) = color {
                    m = Webhook {
                        content: String::new(),
                        username: username,
                        avatar_url: avatar,
                        embeds: vec![Embed { description: message, color: color_int }]
                    };
                }
                else {
                    m = Webhook {
                        content: message,
                        username: username,
                        avatar_url: avatar,
                        embeds: vec![]
                    };
                }

                req = send(url, serde_json::to_string(&m).unwrap(), &token, &req_client);
            }
            else {
                let mut m;

                if let Some(color_int) = color {
                    m = Message {
                        content: String::new(),
                        embed: Some(Embed { description: message, color: color_int }),
                    };
                }
                else {
                    m = Message {
                        content: message,
                        embed: None
                    };
                }

                req = send(format!("{}/channels/{}/messages", URL, channel), serde_json::to_string(&m).unwrap(), &token, &req_client);
            }

            let mut c = mysql_conn.clone();
            pool.execute(move || {
                match req.send() {
                    Err(e) => {
                        println!("{:?}", e);
                    },

                    Ok(mut r) => {
                        println!("{:?}", r);
                        println!("{:?}", r.text());

                        match position {
                            Some(pos) => {
                                let mut maxq = c.prep_exec("SELECT COUNT(*) FROM intervals WHERE reminder = :id", params!{"id" => id}).unwrap();

                                match maxq.next() {
                                    Some(row) => {
                                        let max = mysql::from_row::<(u8)>(row.unwrap());

                                        if max > 0 {
                                            while time < seconds {
                                                let mut q = c.prep_exec("SELECT (period) FROM intervals WHERE reminder = :id AND position = :p", params!{"id" => id, "p" => pos % max}).unwrap();

                                                let period = mysql::from_row::<(u64)>(q.next().unwrap().unwrap());
                                                time += period;

                                                c.prep_exec("UPDATE reminders SET position = :p, time = :t WHERE id = :id", params!{"p" => pos + 1, "t" => time, "id" => id}).unwrap();
                                            }
                                        }
                                        else {
                                            c.prep_exec("DELETE FROM reminders WHERE id = :id OR time < 0", params!{"id" => id}).unwrap();
                                        }
                                    },

                                    None => {
                                        c.prep_exec("DELETE FROM reminders WHERE id = :id OR time < 0", params!{"id" => id}).unwrap();
                                    },
                                }

                            },

                            None => {
                                c.prep_exec("DELETE FROM reminders WHERE id = :id OR time < 0", params!{"id" => &id}).unwrap();
                            }
                        }
                    }
                }
            });
        }

        thread::sleep(Duration::from_secs(interval));
    }
}

fn send(url: String, m: String, token: &str, client: &reqwest::Client) -> reqwest::RequestBuilder {
    client.post(&url)
        .body(m)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {}", token))
}
