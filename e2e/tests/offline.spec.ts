import { expect, test } from "@playwright/test";
import { readBoard } from "./sudoku";

test("the app still opens with no connection after the first visit", async ({ page, context }) => {
  await page.goto("/");
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  const before = await readBoard(page);

  await context.setOffline(true);
  await page.reload();
  expect(await readBoard(page)).toEqual(before);
});
