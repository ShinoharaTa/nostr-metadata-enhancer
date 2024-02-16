use anyhow::{Context, Result};
use job_scheduler::{Job, JobScheduler};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;

fn load_config(file_path: &str) -> Result<Config> {
    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("設定ファイル {} の読み込みに失敗しました", file_path))?;
    let config: Config = serde_json::from_str(&file_content)
        .with_context(|| format!("設定ファイル {} の展開に失敗しました", file_path))?;
    Ok(config)
}

#[derive(Serialize, Deserialize)]
struct Config {
    keys: Vec<String>,
    relays: Vec<String>,
    exec_time: String,
}

async fn nostr_process(client: &Client, key: String) -> Result<()> {
    let keys = Keys::from_sk_str(&key)
        .with_context(|| format!("次のキーは正常に使用できませんでした: {}", key))?;
    let npub: String = keys.public_key().to_bech32()?;
    let filters = Filter::new()
        .author(keys.public_key())
        .kind(Kind::Metadata)
        .limit(1);
    let events = client
        .get_events_of(vec![filters], Some(Duration::from_secs(10)))
        .await
        .with_context(|| format!("Kind: 0 の取得に失敗しました: key={}", npub))?;
    if let Some(latest_event) = events.get(0) {
        let content = &latest_event.content;
        let event = EventBuilder::new(Kind::Metadata, content, [])
            .to_event(&keys)
            .with_context(|| format!("イベントの生成に失敗しました key={}", npub))?;
        let _result = client
            .send_event(event)
            .await
            .with_context(|| format!("イベントの送信に失敗しました key={}", npub))?;
        println!("Kind: 0 の更新に成功しました: key={}", npub)
    } else {
        println!("Kind: 0 の取得に失敗しました: key={}", npub);
    }
    Ok(())
}

async fn process(client: Client, keys: Vec<String>) {
    for key in keys {
        if let Err(e) = nostr_process(&client, key).await {
            println!("{}", e);
            continue;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = "./config.json";
    let config = load_config(config_path)?;
    let client = Client::default();
    client
        .add_relays(config.relays)
        .await
        .with_context(|| format!("リレーの設定に失敗しました"))?;
    client.connect().await;

    let mut sched = JobScheduler::new();
    match config.exec_time.parse() {
        Ok(cron) => {
            sched.add(Job::new(cron, || {
                tokio::spawn(process(client.clone(), config.keys.clone()));
            }));
        }
        Err(e) => {
            eprintln!("failed: {}", e);
        }
    }

    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }
}
