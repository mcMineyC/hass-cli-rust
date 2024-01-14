use std::fs;
use toml::Table;
use reqwest;
use reqwest::header::*;
use std::time::Duration;
use tokio::runtime::Runtime;
use std::collections::HashMap;

fn main(){
    println!("Hello, world!");
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(async {light("light.lower", "on").await });
        tokio::time::sleep(Duration::from_secs(1)).await;
        tokio::spawn(async {light("light.lower", "off").await });
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Sleep 2");
        tokio::spawn(async {light("light.lower", "on").await });

    });
}
async fn light(name: &str, state: &str){
    let mut data = HashMap::new();
    data.insert("entity_id", name);
    let url = format!("http://192.168.30.36:8123/api/serivces/light/{}", state);
    let client = reqwest::Client::new();
    let response = client
                                            .post(url)
                                            .json(&data)
                                            .header(AUTHORIZATION, "Bearer ")
                                            .header(CONTENT_TYPE, "application/json")
                                            .send()
                                            .await.unwrap().text().await.unwrap();
    println!("Response: {:?}", response);
}

fn load_config() -> Result<Table, toml::de::Error> {
    let contents = fs::read_to_string("config.toml").unwrap();
    toml::from_str(&contents)
}