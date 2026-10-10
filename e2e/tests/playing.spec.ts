import { expect, test } from "@playwright/test";
import { button, cell, emptyCellAndAnswer, enter, readBoard, solve, wrongDigit } from "./sudoku";

test("the timer counts up while playing", async ({ page }) => {
  await page.clock.install({ time: new Date("2026-01-01T10:00:00") });
  await page.goto("/");
  await page.clock.pauseAt(new Date("2026-01-01T10:00:01"));
  await page.clock.runFor(64_000);
  await expect(page.getByLabel("Time")).toHaveText("1:04");
});

test("solving a puzzle celebrates and counts toward stats", async ({ page }) => {
  await page.goto("/");
  const grid = await readBoard(page);
  const solution = solve(grid);
  for (let index = 0; index < 81; index++) {
    if (grid[index] === 0) await enter(page, index, solution[index]);
  }
  const celebration = page.getByRole("dialog", { name: "Puzzle solved" });
  await expect(celebration.getByRole("status")).toHaveText("Solved!");
  const time = (await celebration.innerText()).match(/Best time: (\d+:\d\d)/)![1];

  await celebration.getByRole("button", { name: "Close" }).click();
  await page.getByRole("button", { name: "Stats" }).click();
  const easyRow = page.getByRole("dialog", { name: "Stats" }).getByRole("row", { name: /Easy/ });
  await expect(easyRow).toHaveText(new RegExp(`Easy\\s*1\\s*${time}\\s*${time}`));
});

test("a wrong number shows in red until it is erased", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, wrongDigit(answer));
  await expect(cell(page, index)).toHaveClass(/\bmistake\b/);
  await button(page, "Erase").click();
  await expect(cell(page, index)).not.toHaveClass(/\bmistake\b/);
});

test("choosing a difficulty starts a new puzzle at that level", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  const before = await readBoard(page);
  await page.getByRole("button", { name: "New game" }).click();
  const menu = page.getByRole("dialog", { name: "New game" });
  await expect(menu).toContainText("Your current puzzle will be lost.");
  await menu.getByRole("button", { name: "Hard" }).click();
  await expect(page.getByText("Hard", { exact: true })).toBeVisible();
  expect(await readBoard(page)).not.toEqual(before);
});

test("pencil marks can be written, cleared by a number, and undone", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await cell(page, index).click();
  await button(page, "Pencil").click();
  await button(page, String(answer)).click();
  await button(page, String(wrongDigit(answer))).click();
  await expect(cell(page, index)).toHaveAccessibleName(/notes/);

  await button(page, "Pencil").click();
  await button(page, String(answer)).click();
  await expect(cell(page, index)).toHaveAccessibleName(new RegExp(`, ${answer}$`));

  await button(page, "Undo").click();
  await expect(cell(page, index)).toHaveAccessibleName(/notes/);
});

test("number keys fill the selected cell and Backspace erases it", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await cell(page, index).click();
  await page.keyboard.press(String(answer));
  await expect(cell(page, index)).toHaveAccessibleName(new RegExp(`, ${answer}$`));

  await page.keyboard.press("Backspace");
  await expect(cell(page, index)).toHaveAccessibleName(/, empty$/);
});

test("number keys write pencil marks in pencil mode", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await cell(page, index).click();
  await button(page, "Pencil").click();
  await page.keyboard.press(String(answer));
  await expect(cell(page, index)).toHaveAccessibleName(new RegExp(`notes ${answer}$`));
});

test("number keys do nothing while a dialog is open", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await cell(page, index).click();
  await page.getByRole("button", { name: "Stats" }).click();
  await page.keyboard.press(String(answer));
  await expect(cell(page, index)).toHaveAccessibleName(/, empty$/);
});
