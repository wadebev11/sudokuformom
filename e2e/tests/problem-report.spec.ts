import { expect, test } from "@playwright/test";
import { button, emptyCellAndAnswer, enter, readBoard } from "./sudoku";

declare global {
  interface Window {
    shared?: ShareData;
  }
}

test("pasting a shared problem report into the console brings back her game and settings", async ({ page }) => {
  // Headless browsers have no share sheet, so this records what would have been shared.
  await page.addInitScript(() => {
    navigator.share = async (data) => {
      window.shared = data;
    };
  });
  await page.goto("/");
  const { index, answer } = await emptyCellAndAnswer(page);
  await enter(page, index, answer);
  const herBoard = await readBoard(page);
  await button(page, "Hint").click();
  const hintTitle = await page.getByRole("note", { name: "Hint" }).locator("strong").textContent();
  await page.getByRole("button", { name: "Settings" }).click();
  await page.getByLabel("Show mistakes in red").uncheck();
  await button(page, "Send this game to Wade").click();
  const script = await page.evaluate(() => window.shared?.text ?? "");
  expect(script).toContain(`// Hint showing: ${hintTitle}: `);

  await page.evaluate(() => localStorage.clear());
  await page.reload();
  expect(await readBoard(page)).not.toEqual(herBoard);
  await Promise.all([page.waitForEvent("load"), page.evaluate(script)]);

  expect(await readBoard(page)).toEqual(herBoard);
  await page.getByRole("button", { name: "Settings" }).click();
  await expect(page.getByLabel("Show mistakes in red")).not.toBeChecked();
});

test.describe("on a computer", () => {
  test.use({ isMobile: false, hasTouch: false, permissions: ["clipboard-read", "clipboard-write"] });

  test("a problem report is copied instead of shared", async ({ page }) => {
    await page.addInitScript(() => {
      navigator.share = async (data) => {
        window.shared = data;
      };
    });
    await page.goto("/");
    await page.getByRole("button", { name: "Settings" }).click();
    await button(page, "Send this game to Wade").click();

    await expect(button(page, "Copied! Paste it in a message to Wade")).toBeVisible();
    const copied = await page.evaluate(() => navigator.clipboard.readText());
    expect(copied).toContain('localStorage.setItem("sudoku-for-mom:game", ');
    expect(await page.evaluate(() => window.shared)).toBeUndefined();
  });
});

test("the problem report button is hidden when the browser can neither share nor copy", async ({ page }) => {
  await page.addInitScript(() => {
    const prototype = Navigator.prototype as { share?: unknown; clipboard?: unknown };
    delete prototype.share;
    delete prototype.clipboard;
  });
  await page.goto("/");
  await page.getByRole("button", { name: "Settings" }).click();

  await expect(button(page, "Hard reset")).toBeVisible();
  await expect(button(page, "Send this game to Wade")).toBeHidden();
});
