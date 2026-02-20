#!/usr/bin/env node

import { readFile } from "node:fs/promises";
import { WASI } from "node:wasi";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const wasmPath = join(__dirname, "..", "wasm", "agenticenti.wasm");

const wasi = new WASI({
  version: "preview1",
  args: ["agenticenti", ...process.argv.slice(2)],
  env: process.env,
  preopens: { "/": "/" },
  returnOnExit: true,
});

try {
  const wasmBuffer = await readFile(wasmPath);
  const wasmModule = await WebAssembly.compile(wasmBuffer);
  const instance = await WebAssembly.instantiate(wasmModule, wasi.getImportObject());
  const exitCode = wasi.start(instance);
  process.exit(exitCode);
} catch (err) {
  if (err.code !== undefined && typeof err.code === "number") {
    process.exit(err.code);
  }
  if (err instanceof WebAssembly.RuntimeError && err.message.includes("unreachable")) {
    // WASI process.exit() triggers unreachable in some runtimes — check exitCode
    process.exit(1);
  }
  console.error("agenticenti:", err.message);
  process.exit(1);
}
