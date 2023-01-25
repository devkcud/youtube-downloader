use crate::pythonexec::download_video_mp3;
use args::Commands;
use clap::Parser;
use colored::Colorize;
use std::{
    io,
    process::{exit, Command},
};

#[allow(dead_code)]
mod args;
#[allow(dead_code)]
mod console;
#[allow(dead_code)]
mod pythonexec;
#[allow(dead_code)]
mod utils;

fn main() {
    console::info("Checking for python binary");

    let mut py_executable = "python3";

    if utils::try_command("python3") == false {
        py_executable = "python";

        if utils::try_command("python") == false {
            console::error("Cannot continue due to previous errors.");
            exit(1);
        }
    }

    console::info("Checking for pip binary");

    let mut pip_executable = "pip3";

    if utils::try_command("pip3") == false {
        pip_executable = "pip";

        if utils::try_command("pip") == false {
            console::error("Cannot continue due to previous errors.");
            exit(1);
        }
    }

    console::info("Adding pip dependencies");

    Command::new(pip_executable)
        .args(["install", "pytube==12.1.2", "--quiet"])
        .spawn()
        .unwrap();

    let args = args::YTDPArgs::parse();
    match args.command {
        Commands::Download(o) => {
            if args.skip {
                console::warn("SKIPPING CHECKS MAY CAUSE UNWANTED BEHAVIOUR");
            }

            let link = o.link;

            if !link.contains("youtube.com") && !link.contains("youtu.be") {
                console::error(&format!("Not a valid link: {}", link));
                exit(1);
            }

            if link.contains("playlist") {
                let playlist = match pythonexec::load_playlist(py_executable, &link) {
                    Ok(o) => o,
                    Err(_) => {
                        console::error("Something went wrong. Try again.");
                        exit(1);
                    }
                };

                let stdout = String::from_utf8_lossy(&playlist.stdout);
                let mut videos: Vec<&str> = Vec::new();

                console::info("Fetching videos");

                for video_url in stdout.lines() {
                    videos.push(video_url);
                    console::ok(&format!("Loaded: {}", video_url));
                }

                console::warn(&format!("Queue length: {} videos", videos.len()));
                console::warn(&format!("Output path: {}", args.output));

                if o.mp3 {
                    console::warn(&format!("Saving as: .mp3"));
                } else {
                    console::warn(&format!("Saving as: .mp4"));
                }

                console::info("Getting things ready");

                let mut confirmation = String::new();

                console::question("Proceed with download? (Y/n) ");

                if !args.skip {
                    io::stdin()
                        .read_line(&mut confirmation)
                        .expect("Failed to read line");
                } else {
                    println!();
                }

                if confirmation.trim() == "" {
                    confirmation = "y".to_string();
                }

                let confirmation = confirmation.trim();

                match &confirmation.to_lowercase()[..] {
                    "y" => {
                        console::warn("This may take a while depending on your resources.");
                    }
                    "n" => {
                        console::error("Aborting");
                        exit(1);
                    }
                    _ => {
                        console::error(&format!(
                            "Unexpected answer {}. Aborting",
                            confirmation.red().bold()
                        ));
                        exit(1);
                    }
                }

                if !o.mp3 {
                    for (i, video) in videos.iter().enumerate() {
                        console::info(&format!("{}/{} Downloading {}", i + 1, videos.len(), video));

                        let command =
                            pythonexec::download_video_mp4(py_executable, &video, &args.output);

                        match command {
                            Ok(_) => console::ok(&format!("Downloaded {}", video)),
                            Err(_) => console::error(&format!("Error downloading {}", video)),
                        }
                    }
                } else {
                    for (i, video) in videos.iter().enumerate() {
                        console::info(&format!("{}/{} Downloading {}", i + 1, videos.len(), video));

                        let command = download_video_mp3(py_executable, video, &args.output);

                        match command {
                            Ok(_) => console::ok(&format!("Downloaded {}", video)),
                            Err(_) => console::error(&format!("Error downloading {}", video)),
                        }
                    }
                }

                console::ok("Done");
                exit(0);
            }

            if !o.mp3 {
                console::info("MP4 selected");

                console::ok(&format!("Downloading {}", link));
                let command = pythonexec::download_video_mp4(py_executable, &link, &args.output);

                match command {
                    Ok(_) => console::ok(&format!("Downloaded {}", link)),
                    Err(_) => console::error(&format!("Error downloading {}", link)),
                }
            } else {
                console::info("MP3 selected");

                console::ok(&format!("Downloading {}", link));
                let command = download_video_mp3(py_executable, &link, &args.output);

                match command {
                    Ok(_) => console::ok(&format!("Downloaded {}", &link)),
                    Err(_) => console::error(&format!("Error downloading {}", &link)),
                }
            }

            console::ok("Done");
            exit(0);
        }
        Commands::Info(_) => console::warn(
            "This feature is not available right now. Be aware that I am working on it.",
        ),
    }
}
