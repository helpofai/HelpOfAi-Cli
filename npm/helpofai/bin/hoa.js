#!/usr/bin/env node

const { run } = require("../scripts/run");

run("helpofai").catch((error) => {
  console.error("Failed to start helpofai:", error.message);
  process.exit(1);
});
