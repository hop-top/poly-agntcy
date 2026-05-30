from flask import Blueprint, Response, jsonify


def create_directory_blueprint(*, endpoint: str) -> Blueprint:
    if not endpoint:
        raise ValueError("endpoint required")
    bp = Blueprint("agntcy", __name__)

    @bp.route("/agntcy", methods=["POST"])
    def _handler() -> Response:
        return jsonify({"ok": True, "endpoint": endpoint})

    return bp
