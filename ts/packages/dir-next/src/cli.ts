#!/usr/bin/env node
import { Command } from "commander";

const program = new Command("agntcy")
  .version("0.1.0-alpha.0")
  .option("--endpoint <url>", "DIR endpoint")
  .description("AGNTCY Directory Service CLI");

program
  .command("discover")
  .requiredOption("--capability <cap>", "Capability to search for")
  .action(async (opts: { capability: string }) => {
    const endpoint = program.opts<{ endpoint?: string }>().endpoint;
    if (!endpoint) {
      console.error("--endpoint required");
      process.exit(1);
    }
    console.log(`discover ${opts.capability} @ ${endpoint}`);
  });

program.parse();
