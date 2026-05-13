pub fn get(cmd: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://cheat.sh/{}", cmd);
    let client = reqwest::blocking::Client::new();
    let res = client.get(url)
        .header("User-Agent", "curl") // cheat.sh returns ansi-colored text for curl
        .send()?
        .text()?;
    Ok(res)
}
