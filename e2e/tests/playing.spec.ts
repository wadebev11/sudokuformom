import { expect, test } from "@playwright/test";
import { autoSolved, board, button, cell, emptyCellAndAnswer, enter, nakedSingles, readBoard, solve, wrongDigit } from "./sudoku";

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

test("restarting clears the puzzle and the timer but keeps the givens", async ({ page }) => {
  await page.clock.install({ time: new Date("2026-01-01T10:00:00") });
  await page.goto("/");
  const givens = await readBoard(page);
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  await page.clock.runFor(5_000);
  await expect(page.getByLabel("Time")).not.toHaveText("0:00");

  await page.getByRole("button", { name: "New game" }).click();
  await page.getByRole("dialog", { name: "New game" }).getByRole("button", { name: "Restart this puzzle" }).click();
  expect(await readBoard(page)).toEqual(givens);
  await expect(page.getByLabel("Time")).toHaveText("0:00");
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

test("selecting a number highlights the notes that match it", async ({ page }) => {
  await page.goto("/");
  const grid = await readBoard(page);
  const givenIndex = grid.findIndex((digit) => digit !== 0);
  const given = grid[givenIndex];
  const emptyIndex = grid.indexOf(0);
  await cell(page, emptyIndex).click();
  await button(page, "Pencil").click();
  await button(page, String(given)).click();
  await button(page, String(wrongDigit(given))).click();

  const matchingNotes = cell(page, emptyIndex).locator(".pencil-marks .matching");
  await expect(matchingNotes).toHaveCount(0);
  await cell(page, givenIndex).click();
  await expect(matchingNotes).toHaveText(String(given));
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

test("clicking outside a dialog closes it, clicking inside does not", async ({ page }) => {
  await page.goto("/");
  const clickBackdrop = () => page.mouse.click(5, 5);

  for (const name of ["Stats", "Settings", "New game"]) {
    await page.getByRole("button", { name }).click();
    const dialog = page.getByRole("dialog", { name });
    await dialog.getByRole("heading", { name, exact: true }).click();
    await expect(dialog).toBeVisible();
    await clickBackdrop();
    await expect(dialog).toBeHidden();
  }
});

test("clicking outside the hard reset confirmation goes back to settings", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Settings" }).click();
  await button(page, "Hard reset").click();
  await page.mouse.click(5, 5);
  await expect(page.getByRole("dialog", { name: "Hard reset" })).toBeHidden();
  await expect(page.getByRole("dialog", { name: "Settings" })).toBeVisible();
});

test("finishing a row animates each of its cells", async ({ page }) => {
  await page.goto("/");
  const grid = await readBoard(page);
  const solution = solve(grid);
  const row = Array.from({ length: 9 }, (_, index) => index);
  for (const index of row) {
    if (grid[index] === 0) await enter(page, index, solution[index]);
  }
  for (const index of row) {
    await expect(cell(page, index)).toHaveClass(/\bcompleting-(even|odd)\b/);
  }
});

test("in pencil mode the Hint button fills in notes for cells without any", async ({ page }) => {
  await page.goto("/");
  const { index } = await emptyCellAndAnswer(page);
  await button(page, "Pencil").click();
  await expect(button(page, "Hint")).toBeHidden();

  await button(page, "Fill notes").click();
  await expect(cell(page, index)).toHaveAccessibleName(/notes/);
  await expect(button(page, "Fill notes")).toBeDisabled();

  await button(page, "Undo").click();
  await expect(cell(page, index)).toHaveAccessibleName(/, empty$/);
  await button(page, "Pencil").click();
  await expect(button(page, "Hint")).toBeVisible();
});

test("auto-solving naked singles fills them on Medium but leaves Easy alone", async ({ page }) => {
  await page.goto("/");
  const easyBoard = await readBoard(page);
  await page.getByRole("button", { name: "Settings" }).click();
  await page.getByLabel("Auto-solve naked singles").check();
  await button(page, "Done").click();
  expect(await readBoard(page)).toEqual(easyBoard);

  await page.getByRole("button", { name: "New game" }).click();
  await page.getByRole("dialog", { name: "New game" }).getByRole("button", { name: "Medium" }).click();
  await expect(page.getByText("Medium", { exact: true })).toBeVisible();
  expect(nakedSingles(await readBoard(page))).toEqual([]);
});

test("numbers auto-solving fills ink in, and undo or redo does not replay them", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Settings" }).click();
  await page.getByLabel("Auto-solve naked singles").check();
  await button(page, "Done").click();
  await page.getByRole("button", { name: "New game" }).click();
  await page.getByRole("dialog", { name: "New game" }).getByRole("button", { name: "Medium" }).click();
  await expect(page.getByText("Medium", { exact: true })).toBeVisible();
  const inking = board(page).locator(".ink-in-even, .ink-in-odd");

  // A move that sets off Auto-solve without finishing the puzzle, which would stop undo.
  const grid = await readBoard(page);
  const solution = solve(grid);
  const index = grid.findIndex((digit, index) => {
    const entered = grid.map((other, otherIndex) => (otherIndex === index ? solution[index] : other));
    const after = autoSolved(entered, solution);
    return digit === 0 && after.includes(0) && after.some((other, otherIndex) => other !== entered[otherIndex]);
  });
  await enter(page, index, solution[index]);
  await expect(inking).not.toHaveCount(0);
  await button(page, "Undo").click();
  await expect(inking).toHaveCount(0);
  await button(page, "Redo").click();
  await expect(inking).toHaveCount(0);
});
