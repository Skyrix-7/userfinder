use reqwest;
use std:: io;
use std::io::Stdout;
 use std::io::Write;
use colored::*;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    clearscreen::clear();

    logo();

    print!("{}?{} ", "[".bright_green(), "] Input Username:".bright_green());

    io::stdout().flush();

    let mut inp = String::new();


    io::stdin().read_line(&mut inp).expect("Failed");

    let urls = vec![
        ("Instagram".to_string(), "https://instagram.com/".to_string()),
        ("GitHub".to_string(), "https://github.com/".to_string()),
        ("Facebook".to_string(), "https://facebook.com/".to_string()),
        ("Twitter".to_string(), "https://twitter.com/".to_string()),
        ("X".to_string(), "https://x.com/".to_string()),
        ("LinkedIn".to_string(), "https://linkedin.com/in/".to_string()),
        ("Reddit".to_string(), "https://reddit.com/user/".to_string()),
        ("YouTube".to_string(), "https://youtube.com/@".to_string()),
        ("TikTok".to_string(), "https://tiktok.com/@".to_string()),
        ("Snapchat".to_string(), "https://snapchat.com/add/".to_string()),
        ("Pinterest".to_string(), "https://pinterest.com/".to_string()),
        ("Twitch".to_string(), "https://twitch.tv/".to_string()),
        ("Discord".to_string(), "https://discord.com/users/".to_string()),
        ("Telegram".to_string(), "https://t.me/".to_string()),
        ("Tumblr".to_string(), "https://tumblr.com/".to_string()),
        ("Medium".to_string(), "https://medium.com/@".to_string()),
        ("Dev.to".to_string(), "https://dev.to/".to_string()),
        ("Stack Overflow".to_string(), "https://stackoverflow.com/users/".to_string()),
        ("GitLab".to_string(), "https://gitlab.com/".to_string()),
        ("Bitbucket".to_string(), "https://bitbucket.org/".to_string()),
        ("SourceHut".to_string(), "https://sourcehut.org/~".to_string()),
        ("Codeberg".to_string(), "https://codeberg.org/".to_string()),
        ("crates.io".to_string(), "https://crates.io/users/".to_string()),
        ("npm".to_string(), "https://npmjs.com/~".to_string()),
        ("PyPI".to_string(), "https://pypi.org/user/".to_string()),
        ("Docker Hub".to_string(), "https://hub.docker.com/u/".to_string()),
        ("Mastodon".to_string(), "https://mastodon.social/@".to_string()),
        ("Bluesky".to_string(), "https://bsky.app/profile/".to_string()),
        ("Patreon".to_string(), "https://patreon.com/".to_string()),
        ("Kickstarter".to_string(), "https://kickstarter.com/profile/".to_string()),
        ("Spotify".to_string(), "https://open.spotify.com/user/".to_string()),
        ("SoundCloud".to_string(), "https://soundcloud.com/".to_string()),
        ("Bandcamp".to_string(), "https://bandcamp.com/".to_string()),
        ("Vimeo".to_string(), "https://vimeo.com/".to_string()),
        ("Steam".to_string(), "https://steamcommunity.com/id/".to_string()),
        ("itch.io".to_string(), "https://itch.io/profile/".to_string()),
        ("Quora".to_string(), "https://quora.com/profile/".to_string()),
        ("Behance".to_string(), "https://behance.net/".to_string()),
        ("Dribbble".to_string(), "https://dribbble.com/".to_string()),
    ];

    println!(" ");
    println!("{}*{} {} {} ", "[".bright_green(), "] Checking username".bright_green(), inp.trim(), "on:".bright_green());

    for base_url in urls {

        let url = format!("{}{}", base_url.1.trim(), inp.trim());

        let response = reqwest::get(&url).await?;
        let status = response.status();
        let body_text = response.text().await.unwrap_or_default();

        if status.is_success() && !body_text.contains("Uh oh!") && !body_text.contains("Log into Facebook") && !body_text.contains("wrong") {
            println!("[{}] {}: {} {}", "+".bright_green(), base_url.0.trim(), "Found!".bright_green(), url);
        } else {
            println!("[{}] {}: {}", "-".bright_red(), base_url.0.trim(), "Not Found!".bright_red());
        }
    }





    
    
    

    Ok(())

    
}


fn logo() {
    println!("");
    println!("██╗   ██╗███████╗███████╗██████╗       ███████╗██╗███╗   ██╗██████╗ ███████╗██████╗ ");
    println!("██║   ██║██╔════╝██╔════╝██╔══██╗      ██╔════╝██║████╗  ██║██╔══██╗██╔════╝██╔══██╗");
    println!("██║   ██║███████╗█████╗  ██████╔╝█████╗█████╗  ██║██╔██╗ ██║██║  ██║█████╗  ██████╔╝");
    println!("██║   ██║╚════██║██╔══╝  ██╔══██╗╚════╝██╔══╝  ██║██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗");
    println!("╚██████╔╝███████║███████╗██║  ██║      ██║     ██║██║ ╚████║██████╔╝███████╗██║  ██║");
    println!(" ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝      ╚═╝     ╚═╝╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝");
    let username = "@Skyrix-7";
    let url = "https://github.com/Skyrix-7/";
    
    let hyperlink = format!("\x1B]8;;{}\x1B\\{}\x1B]8;;\x1B\\", url, username);

    println!(
        "                        {} {} {} {} {}",
        "Skyrix".truecolor(181, 242, 169).bold(),
        "v1.0".truecolor(140, 140, 140),
        "│".truecolor(80, 80, 80),
        "Developed by".truecolor(180, 180, 180),
        hyperlink.truecolor(181, 242, 169).underline() 
    );
                                                      

                                                                                    
}