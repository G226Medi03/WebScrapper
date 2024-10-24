from fastapi import FastAPI
from fastapi.responses import FileResponse
import Youtube
import threading

app = FastAPI()



@app.get("/download")
def download(url : str):
    global _localFileName
  
    _localFileName = Youtube.downloadYoutubeAudio(url=url)
    return FileResponse(path=f"./mp3/{_localFileName}", filename=_localFileName)

@app.get("/filename")
def api_filename():
    return _localFileName






