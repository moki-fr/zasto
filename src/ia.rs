use reqwest::blocking::Client;
use serde_json::json;

// Big system prompt for the ai
const SYSTEM_PROMPT: &str = "You're in a tool called Zasto, it's an open source tool to free space, that scans a disks and uses AI to tell if heavy folders are worth keeping or not.

You will be given about a 100 paths sorted from heaviest to lightest with their sizes and you will have to output only the folders that you think are worth deleting, and a description of what it is.

## Important
You have 2 rules:

- Respond only with the paths, nothing else before or after, no 'Sure, here are the folders that are worth deleting..'
- You **HAVE** to use the correct formating, here are an example:

\"/home/user/.minecraft/\"|Minecraft directory|6.2GB
\"/home/user/download/heavy.txt\"|Heavy files in download|23GB
\"/home/user/Document/document.png\"|Heavy document|14GB


Your anwser has to be multiple lines, the three lines above is what you should answer, each line equals a file.

Here's an explaination:
first you link the path of the folder between quotes, then you add a @ to separate the path to the comment and the size, and if you want to add another path with a comment and size, separate it with |
it can be represented as this:

\"path\"|comment|size
\"path\"|comment|size
\"path\"|comment|size
...

It's better not to set a comment which contains more than 7 words but it's not really important
Remember that you HAVE to set a comment, even if you don't know where that file is pointing to, just say \"Unknown path\" or something like that
And also you HAVE to set a size, even if you're not sure just say \"? GB\"

It's better not to consider binary executable programs as something that's worth deleting because we assume that the user knows what programs they have installed. So for example avoid 'C:/ProgramFiles/ACoolProgram/program.exe'
If you have any problem (like not having the paths, or missunderstood something) please replace your whole reply with \"ai_thinking_error\" ";

pub fn check_api_key(api_key: &str) -> Result<bool, reqwest::Error> {
    let client = Client::new();

    let response = client
        .get("https://openrouter.ai/api/v1/auth/key")
        .bearer_auth(api_key)
        .send()?;

    Ok(response.status().is_success())
}

pub fn ai(api_key: &str, model: &str, scan_result: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    // Your API key should be set in the environment

    let request_body = json!({
        "model": model,
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": scan_result}
        ]
    });

    // Example API request
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()?;

    let output: serde_json::Value = response.json()?;

    let content = &output["choices"][0]["message"]["content"];

    let content = content.as_str().unwrap_or("").to_string();

    Ok(content)
}
