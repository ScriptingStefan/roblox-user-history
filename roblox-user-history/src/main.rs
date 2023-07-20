use reqwest;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write, Read};

const API_BASE_LINK: &str = "https://users.roblox.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let user_id = get_user_id_from_input();
    let endpoint = format!("{}/v1/users/{}/username-history", API_BASE_LINK, user_id);

    let response_data = fetch_data(&endpoint).await?;
    let usernames = parse_usernames(&response_data);

    match usernames {
        Ok(value) => {
            save_usernames_to_file("usernames.txt", &value).unwrap();
            println!("Usernames have been saved to 'usernames.txt'");
        },
        Err(_) => {
            println!("Couldn't get past usernames!");
        }
    }
    println!("Press enter to exit...");
    io::stdin().read_exact(&mut [1]).unwrap();
    Ok(())
}

async fn fetch_data(endpoint: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(endpoint).await?.text().await?;
    Ok(response)
}

fn parse_usernames(response_data: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let decoded: Value = serde_json::from_str(response_data)?;
    let data_map = extract_data_map(&decoded)?;
    let usernames = extract_usernames_from_list(&data_map)?;
    Ok(usernames)
}

fn extract_data_map(decoded: &Value) -> Result<&Value, Box<dyn Error>> {
    if let Value::Object(data_map) = decoded {
        if let Some(data) = data_map.get("data") {
            return Ok(data);
        }
    }
    Err("Invalid JSON response".into())
}

fn extract_usernames_from_list(data_map: &Value) -> Result<Vec<String>, Box<dyn Error>> {
    if let Value::Array(username_list) = data_map {
        let usernames = username_list
            .iter()
            .map(|entry| extract_username(entry))
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(usernames);
    }
    Err("Invalid JSON response".into())
}

fn extract_username(entry: &Value) -> Result<String, Box<dyn Error>> {
    if let Value::Object(username_map) = entry {
        if let Some(Value::String(name)) = username_map.get("name") {
            Ok(name.clone())
        } else {
            Err("Invalid name field in JSON".into())
        }
    } else {
        Err("Invalid object in JSON".into())
    }
}

fn save_usernames_to_file(filename: &str, usernames: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    for username in usernames {
        file.write_all(username.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

fn get_user_id_from_input() -> u64 {
    loop {
        print!("Please enter the user ID: ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<u64>() {
            Ok(id) => return id,
            Err(_) => println!("Invalid input. Please enter a valid user ID."),
        }
    }
}