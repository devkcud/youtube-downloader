use std::process::Command;

pub fn download_video_mp3(
    py_exec: &str,
    url: &str,
    output_path: &str,
) -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new(py_exec)
        .args(["-c", "import sys, pytube
try: yt_obj = pytube.YouTube(sys.argv[1]); video_name = yt_obj.title.title().replace(' ', '').replace('!', '').replace('?', '').replace('/', '').replace(',', '').replace('.', ''); yt_obj.streams.get_audio_only().download(output_path=sys.argv[2], filename=f'{video_name}.mp3')
except:
    try: yt_obj = pytube.YouTube(sys.argv[1]); video_name = yt_obj.title.title().replace(' ', '').replace('!', '').replace('?', '').replace('/', '').replace(',', '').replace('.', ''); yt_obj.streams.get_audio_only().download(output_path=sys.argv[2], filename=f'{video_name}.mp3')
    except: exit(1)", url, output_path]).status()
}

pub fn download_video_mp4(
    py_exec: &str,
    url: &str,
    output_path: &str,
) -> Result<std::process::ExitStatus, std::io::Error> {
    Command::new(py_exec)
        .args(["-c", "import sys, pytube
try: yt_obj = pytube.YouTube(sys.argv[1]); video_name = yt_obj.title.title().replace(' ', '').replace('!', '').replace('?', '').replace('/', '').replace(',', '').replace('.', ''); yt_obj.streams.get_highest_resolution().download(output_path=sys.argv[2], filename=f'{video_name}.mp4')
except:
    try: yt_obj = pytube.YouTube(sys.argv[1]); video_name = yt_obj.title.title().replace(' ', '').replace('!', '').replace('?', '').replace('/', '').replace(',', '').replace('.', ''); yt_obj.streams.get_highest_resolution().download(output_path=sys.argv[2], filename=f'{video_name}.mp4')
    except: exit(1)", url, output_path]).status()
}

pub fn load_playlist(py_exec: &str, url: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new(py_exec)
        .args([
            "-c",
            "import sys, pytube; print('\\n'.join(pytube.Playlist(sys.argv[1])))",
            url,
        ])
        .output()
}
