import os

from fastapi import FastAPI
from poly_agntcy_dir_fastapi import create_directory_router

app = FastAPI(title="poly-agntcy fastapi-quickstart")

app.include_router(
    create_directory_router(
        endpoint=os.environ.get("AGNTCY_ENDPOINT", "http://localhost:8888"),
    )
)


@app.get("/")
def root() -> dict[str, str]:
    return {"hello": "poly-agntcy"}
