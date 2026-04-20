const os = require("os");
const fs = require("fs");
const https = require("https");
const path = require("path");

const version =
  process.env.npm_package_version || require("../package.json").version;

const platform = os.platform();
const arch = os.arch();

const platformMap = {
  win32: "windows",
  darwin: "macos",
  linux: "linux",
};

const archMap = {
  x64: "x86_64",
  arm64: "aarch64",
};

const mappedPlatform = platformMap[platform] || platform;
const mappedArch = archMap[arch] || arch;
const extension = platform === "win32" ? ".exe" : "";

// The binary name as it will be uploaded to GitHub Releases
// Example: lget-linux-x86_64
const binName = `lget-${mappedPlatform}-${mappedArch}${extension}`;
const downloadUrl = `https://github.com/pencelheimer/lget/releases/download/v${version}/${binName}`;

const binDir = path.join(__dirname, "bin");
const binPath = path.join(binDir, `lget${extension}`);

function download(url, dest) {
  return new Promise((resolve, reject) => {
    https
      .get(url, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          return download(response.headers.location, dest)
            .then(resolve)
            .catch(reject);
        }
        if (response.statusCode >= 400) {
          return reject(
            new Error(`Failed to download: ${response.statusCode} - ${url}`),
          );
        }

        const file = fs.createWriteStream(dest, { mode: 0o755 });
        response.pipe(file);
        file.on("finish", () => {
          file.close();
          resolve();
        });
      })
      .on("error", (err) => {
        fs.unlink(dest, () => reject(err));
      });
  });
}

function verifyLocalBuildExists() {
  const localTargetBuild = path.join(
    __dirname,
    "..",
    "target",
    "release",
    `lget${extension}`,
  );
  if (fs.existsSync(localTargetBuild)) {
    return localTargetBuild;
  }
  const localDebugBuild = path.join(
    __dirname,
    "..",
    "target",
    "debug",
    `lget${extension}`,
  );
  if (fs.existsSync(localDebugBuild)) {
    return localDebugBuild;
  }
  return null;
}

async function main() {
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  // Since this is rust, handle local development
  // If there is an environment variable to skip download, or if we are inside a rust workspace,
  // we can attempt to just use the cargo built binary during development.
  const isDev =
    process.env.LGET_NPM_DEV === "1" ||
    fs.existsSync(path.join(__dirname, "..", "Cargo.toml"));

  if (isDev) {
    console.log(
      `[lget postinstall] Development mode detected. Checking for local cargo builds...`,
    );
    const localBinary = verifyLocalBuildExists();
    if (localBinary) {
      console.log(
        `[lget postinstall] Copying local build from ${localBinary} to ${binPath}`,
      );
      fs.copyFileSync(localBinary, binPath);
      return;
    } else {
      console.log(
        `[lget postinstall] No local build found in 'target/release' or 'target/debug'.`,
      );
      console.log(
        `[lget postinstall] You should run 'cargo build' or 'cargo build --release' if you want to use the wrapper locally.`,
      );
    }
  }

  console.log(
    `[lget postinstall] Downloading pre-compiled binary from GitHub Releases...`,
  );
  console.log(`[lget postinstall] URL: ${downloadUrl}`);

  try {
    await download(downloadUrl, binPath);
    console.log(
      `[lget postinstall] Successfully downloaded binary to ${binPath}`,
    );
  } catch (err) {
    console.error(
      `[lget postinstall] Warning: Could not download binary (${err.message}).`,
    );
    console.error(
      `[lget postinstall] If this is a new release, the binaries might still be building.`,
    );
    if (!isDev) {
      process.exit(1);
    }
  }
}

main();
