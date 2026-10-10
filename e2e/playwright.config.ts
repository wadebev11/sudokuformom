import { defineConfig, devices } from "@playwright/test";

const port = 8123;

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  reporter: "list",
  use: {
    ...devices["Pixel 7"],
    baseURL: `http://127.0.0.1:${port}`,
  },
  webServer: {
    // A separate dist folder keeps test runs from overwriting a release build.
    command: `trunk serve --port ${port} --address 127.0.0.1 --dist ../../target/e2e-dist`,
    cwd: "../crates/app",
    url: `http://127.0.0.1:${port}`,
    reuseExistingServer: true,
    timeout: 300_000,
  },
});
