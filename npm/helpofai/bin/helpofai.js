#!/usr/bin/env node

const { runHelpOfAi } = require("../scripts/run");

runHelpOfAi().catch((error) => {
  console.error("Failed to start helpofai:", error.message);
  process.exit(1);
});
