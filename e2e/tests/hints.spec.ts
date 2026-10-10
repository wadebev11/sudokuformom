import { expect, test } from "@playwright/test";
import { button, readBoard } from "./sudoku";

const emptyCount = (grid: number[]) => grid.filter((digit) => digit === 0).length;

test("a hint explains the next step, then fills it in", async ({ page }) => {
  await page.goto("/");
  const before = emptyCount(await readBoard(page));

  await button(page, "Hint").click();
  const panel = page.getByRole("note", { name: "Hint" });
  await expect(panel).toContainText(/single/i);
  await expect(panel).toContainText("Tap Hint again to fill it in.");

  await button(page, "Hint").click();
  await expect(panel).toBeHidden();
  expect(emptyCount(await readBoard(page))).toBe(before - 1);
});
