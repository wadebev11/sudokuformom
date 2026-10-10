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
    command: `trunk serve --port ${port} --address 127.0.0.1`,
    cwd: "../crates/app",
    url: `http://127.0.0.1:${port}`,
    reuseExistingServer: true,
    timeout: 300_000,
  },
});
