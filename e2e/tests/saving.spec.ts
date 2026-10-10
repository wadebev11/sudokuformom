import { expect, test } from "@playwright/test";
import { button, cell, emptyCellAndAnswer, enter, readBoard, wrongDigit } from "./sudoku";

test("the game in progress survives closing and reopening the app", async ({ page }) => {
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  const before = await readBoard(page);

  await page.reload();
  expect(await readBoard(page)).toEqual(before);
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
