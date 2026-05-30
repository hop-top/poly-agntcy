import { Command } from "commander";
import { registerOutputFlags } from "@hop-top/kit/output";

export function createProgram(): Command {
  const program = new Command()
    .name("poly-agntcy")
    .description("Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)")
    .version("0.0.0")
    .option("-v, --verbose", "verbose output");

  // Output flag suite: --format, --format-opt, --format-help, --cols,
  // --template, -o/--output. Adopters render data via dispatch().
  registerOutputFlags(program);

  return program;
}
