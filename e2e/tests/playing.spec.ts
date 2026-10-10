import { expect, test } from "@playwright/test";
import { cell, emptyCellAndAnswer, enter, padButton, readBoard, solve, wrongDigit } from "./sudoku";

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
  await padButton(page, "Erase").click();
  await expect(cell(page, index)).not.toHaveClass(/\bmistake\b/);
});

test("choosing a difficulty starts a new puzzle at that level", async ({ page }) => {
  await page.goto("/");
  const before = await readBoard(page);
  await page.getByRole("button", { name: "New game" }).click();
  await page.getByRole("dialog", { name: "New game" }).getByRole("button", { name: "Hard" }).click();
  await expect(page.getByText("Hard", { exact: true })).toBeVisible();
  expect(await readBoard(page)).not.toEqual(before);
});
