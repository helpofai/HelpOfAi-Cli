#!/usr/bin/env node

const { runHelpOfAiTui } = require("../scripts/run");

runHelpOfAiTui().catch((error) => {
  console.error("Failed to start helpofai-tui:", error.message);
  process.exit(1);
});
