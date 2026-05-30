"""CLI entry point."""

import typer

from hop_top_kit.output import default_registry  # noqa: F401  (registers built-ins)
from hop_top_kit.output.cli import register_output_flags
from hop_top_kit.output.dispatch import dispatch
from hop_top_kit.output.formatter import ColumnSpec

app = typer.Typer(
    name="poly-agntcy",
    help="Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)",
    add_completion=False,
)

# Wire --format / --format-opt / --format-help / --cols / --columns /
# --template / --output|-o on every subcommand.
register_output_flags(app)


@app.callback(invoke_without_command=True)
def main(
    ctx: typer.Context,
    version: bool = typer.Option(
        False,
        "--version",
        "-V",
        help="Show version",
    ),
    verbose: bool = typer.Option(
        False,
        "--verbose",
        "-v",
        help="Verbose output",
    ),
) -> None:
    """Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)"""
    if version:
        from . import __version__

        typer.echo(f"poly-agntcy {__version__}")
        raise typer.Exit()


@app.command()
def hello(ctx: typer.Context, name: str = "World") -> None:
    """Say hello to *name*."""
    typer.echo(f"Hello, {name}!")


@app.command("list")
def list_items(ctx: typer.Context) -> None:
    """Demo list command — exercises the output flag suite.

    Try::

        poly-agntcy list
        poly-agntcy list --format json
        poly-agntcy list --format csv --format-opt delimiter=';'
        poly-agntcy list --cols name,status
        poly-agntcy list -o /tmp/out.json   # ext infers json
        poly-agntcy list --format-help
    """
    items = [
        {"name": "alpha", "count": "1", "status": "ok"},
        {"name": "beta", "count": "2", "status": "warn"},
    ]
    cols = [
        ColumnSpec(header="name", key="name", priority=9),
        ColumnSpec(header="count", key="count", priority=7),
        ColumnSpec(header="status", key="status", priority=5),
    ]
    dispatch(ctx, items, columns=cols)
