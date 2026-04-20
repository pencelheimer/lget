#!/usr/bin/env node

const { spawnSync } = require("child_process");
const path = require("path");
const os = require("os");
const fs = require("fs");

const extension = os.platform() === "win32" ? ".exe" : "";
const binPath = path.join(__dirname, "bin", `lget${extension}`);

if (!fs.existsSync(binPath)) {
  console.error(`[lget error] Missing binary at ${binPath}`);
  console.error(
    `[lget error] It seems the postinstall script failed to download or copy the binary.`,
  );
  console.error(
    `[lget error] Please run 'npm run postinstall' to try downloading it again.`,
  );
  process.exit(1);
}

const args = process.argv.slice(2);

const result = spawnSync(binPath, args, {
  stdio: "inherit",
});

if (result.error) {
  console.error(
    `[lget error] Failed to execute the binary: ${result.error.message}`,
  );
  process.exit(1);
}

process.exit(result.status ?? 0);
