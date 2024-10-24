from pytubefix import YouTube
from pytubefix.streams import Stream


def downloadYoutubeAudio(url: str) -> str:
    yt = YouTube(url)
    yt.streams.filter(type="audio").filter(mime_type="audio/mp4").order_by("abr").last()
    elem : Stream = (
        yt.streams.filter(type="audio")
        .filter(mime_type="audio/mp4")
        .order_by("abr")
        .last()
    )
    filename = yt.title.replace(".", "").replace("|", "").replace("/", "").replace("\\", "") + ".mp3"
    elem.download(filename=filename, output_path=f"./mp3")
    return filename
