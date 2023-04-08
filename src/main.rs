use std::error::Error;

fn send_message_to_chat(bot_username: &str) -> Result<(), Box<dyn Error>> {
    // Set up your bot's API token and the chat ID to send a message to
    let api_token = "YOUR_API_TOKEN";
    let chat_id = "YOUR_CHAT_ID";

    // Set up the message you want to send
    let message = "Hello from my bot!";

    // Set up the request URL
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
        api_token, chat_id, message
    );

    // Send the request using ureq
    let response = ureq::get(&url).call()?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        println!("Error sending message: {:?}", response);
    }

    Ok(())
}

fn main() {
    let bot_username = "YOUR_BOT_USERNAME";
    send_message_to_chat(bot_username).unwrap();
}
