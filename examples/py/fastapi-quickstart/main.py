import os

from fastapi import FastAPI
from agntcy_dir_fastapi import create_directory_router

app = FastAPI(title="hop-top/agntcy fastapi-quickstart")

app.include_router(
    create_directory_router(
        endpoint=os.environ.get("AGNTCY_ENDPOINT", "http://localhost:8888"),
    )
)


@app.get("/")
def root() -> dict[str, str]:
    return {"hello": "hop-top/agntcy"}
