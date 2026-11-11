#!/usr/bin/env node
/** Entry point of the `diracord` npm package. */
import { main } from "@diracord/cli";

process.exitCode = await main(process.argv.slice(2));
