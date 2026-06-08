import pytest
from flask import Flask
from agntcy_dir_flask import create_directory_blueprint


def test_blueprint_mounts() -> None:
    app = Flask(__name__)
    app.register_blueprint(create_directory_blueprint(endpoint="https://dir.example"))
    client = app.test_client()
    res = client.post("/agntcy")
    assert res.status_code == 200
    assert res.get_json()["ok"] is True


def test_endpoint_required() -> None:
    with pytest.raises(ValueError):
        create_directory_blueprint(endpoint="")
