import typer

app = typer.Typer(name="agntcy", help="AGNTCY Directory Service CLI")


@app.command()
def discover(
    endpoint: str = typer.Option(..., "--endpoint"),
    capability: str = typer.Option(..., "--capability"),
) -> None:
    typer.echo(f"discover {capability} @ {endpoint}")


@app.command()
def register(
    endpoint: str = typer.Option(..., "--endpoint"),
    record: str = typer.Option(..., "--record"),
) -> None:
    typer.echo(f"register {record} @ {endpoint}")


@app.command()
def describe(
    endpoint: str = typer.Option(..., "--endpoint"),
    cid: str = typer.Option(..., "--cid"),
) -> None:
    typer.echo(f"describe {cid} @ {endpoint}")


@app.command()
def publish(
    endpoint: str = typer.Option(..., "--endpoint"),
    cid: str = typer.Option(..., "--cid"),
) -> None:
    typer.echo(f"publish {cid} @ {endpoint}")


@app.command()
def verify(
    endpoint: str = typer.Option(..., "--endpoint"),
    cid: str = typer.Option(..., "--cid"),
) -> None:
    typer.echo(f"verify {cid} @ {endpoint}")


if __name__ == "__main__":
    app()
