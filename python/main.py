from fastapi import FastAPI
from fastapi.responses import FileResponse

import Youtube

app = FastAPI()


@app.get("/")
def index(url : str):
    filename =  Youtube.downloadYoutubeAudio(url=url)
    return FileResponse(path=f"./mp3/{filename}", filename=filename)






