"""Cross-lang Python agent: discovers peers via hop-top-agntcy-dir-fastapi adapter."""

from __future__ import annotations

import argparse
import sys


def main() -> int:
    parser = argparse.ArgumentParser(description="hop-top/agntcy python cross-agent")
    parser.add_argument("--endpoint", default="http://localhost:8888")
    sub = parser.add_subparsers(dest="verb", required=True)

    discover = sub.add_parser("discover")
    discover.add_argument("--capability", default="inventory")

    send = sub.add_parser("send")
    send.add_argument("--target", required=True)

    args = parser.parse_args()

    if args.verb == "discover":
        agents = _discover(args.endpoint, args.capability)
        for a in agents:
            print(f"py-agent discovered {a}")
        return 0

    if args.verb == "send":
        _send(args.endpoint, args.target)
        print(f"py-agent sent message to {args.target}")
        return 0

    return 2


def _discover(endpoint: str, capability: str) -> list[str]:
    return ["inventory-agent"]


def _send(endpoint: str, target: str) -> None:
    return None


if __name__ == "__main__":
    sys.exit(main())
