#!/usr/bin/env zx
import "zx/globals";
import { workingDirectory } from "../utils.mjs";

// Start the local validator if it's not already running.
await $`pnpm validator:restart`;

// Build the client and run the tests.
cd(path.join(workingDirectory, "clients", "js-v1"));
await $`pnpm install`;
await $`pnpm build`;
await $`pnpm test ${argv._}`;
