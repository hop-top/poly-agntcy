import os

from flask import Flask
from agntcy_dir_flask import create_directory_blueprint

app = Flask(__name__)
app.register_blueprint(
    create_directory_blueprint(
        endpoint=os.environ.get("AGNTCY_ENDPOINT", "http://localhost:8888"),
    )
)


@app.get("/")
def root() -> dict[str, str]:
    return {"hello": "hop-top/agntcy"}


if __name__ == "__main__":
    app.run(debug=True)
