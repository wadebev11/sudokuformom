import { expect, test } from "@playwright/test";
import { board, button, cell, emptyCellAndAnswer, enter, readBoard, wrongDigit } from "./sudoku";

test("the game in progress survives closing and reopening the app", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  const before = await readBoard(page);

  await page.reload();
  expect(await readBoard(page)).toEqual(before);
});

test("the time survives closing and reopening the app", async ({ page }) => {
  await page.clock.install({ time: new Date("2026-01-01T10:00:00") });
  await page.goto("/");
  await page.clock.pauseAt(new Date("2026-01-01T10:00:01"));
  await page.clock.runFor(64_000);
  await expect(page.getByLabel("Time")).toHaveText("1:04");

  await page.reload();
  await expect(page.getByLabel("Time")).toHaveText("1:04");
});

test("mistakes are not shown in red when that setting is off, even after reopening", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Settings" }).click();
  await page.getByLabel("Show mistakes in red").uncheck();
  await button(page, "Done").click();
  await page.reload();

  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, wrongDigit(answer));
  await expect(cell(page, index)).toHaveAccessibleName(new RegExp(`, ${wrongDigit(answer)}$`));
  await expect(cell(page, index)).not.toHaveClass(/\bmistake\b/);
});

test("hard reset erases the saved game and settings after confirming", async ({ page }) => {
  await page.goto("/");
  await page.evaluate(() => localStorage.setItem("other-app", "kept"));
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  const before = await readBoard(page);
  await page.getByRole("button", { name: "Settings" }).click();
  await page.getByLabel("Show mistakes in red").uncheck();

  await button(page, "Hard reset").click();
  await Promise.all([page.waitForEvent("load"), button(page, "Erase everything").click()]);

  expect(await readBoard(page)).not.toEqual(before);
  await page.getByRole("button", { name: "Settings" }).click();
  await expect(page.getByLabel("Show mistakes in red")).toBeChecked();
  expect(await page.evaluate(() => localStorage.getItem("other-app"))).toBe("kept");
});

test("cancelling a hard reset keeps everything", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  const before = await readBoard(page);

  await page.getByRole("button", { name: "Settings" }).click();
  await button(page, "Hard reset").click();
  await button(page, "Cancel").click();
  await button(page, "Done").click();

  expect(await readBoard(page)).toEqual(before);
  await page.reload();
  expect(await readBoard(page)).toEqual(before);
});

test.describe("before the app loads", () => {
  // Requests a service worker answers skip page.route, so the worker is kept out of the way.
  test.use({ serviceWorkers: "block" });

  test("the saved theme is applied", async ({ page }) => {
    await page.goto("/");
    await page.getByRole("button", { name: "Settings" }).click();
    await page.getByLabel("Dark").check();
    await button(page, "Done").click();

    await page.route("**/*.wasm", (route) => route.abort());
    await page.reload();
    await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
    await expect(board(page)).toHaveCount(0);
  });
});
