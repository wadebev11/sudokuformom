import { expect, test } from "@playwright/test";
import { button, cell, emptyCellAndAnswer, enter, readBoard, solve, wrongDigit } from "./sudoku";

test("filling in every number solves the puzzle", async ({ page }) => {
  await page.goto("/");
  const grid = await readBoard(page);
  const solution = solve(grid);
  for (let index = 0; index < 81; index++) {
    if (grid[index] === 0) await enter(page, index, solution[index]);
  }
  await expect(page.getByRole("status")).toHaveText(/Solved/);
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
