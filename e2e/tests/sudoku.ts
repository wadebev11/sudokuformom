import { expect, type Page } from "@playwright/test";

export const board = (page: Page) => page.getByRole("group", { name: "Sudoku board" });

export const cell = (page: Page, index: number) =>
  board(page).getByRole("button", { name: new RegExp(`^Row ${Math.floor(index / 9) + 1}, column ${(index % 9) + 1},`) });

export const button = (page: Page, name: string) => page.getByRole("button", { name, exact: true });

/** Reads the board the way a player sees it: one entry per cell, 0 when empty. */
export async function readBoard(page: Page): Promise<number[]> {
  const labels = await board(page).getByRole("button").evaluateAll((cells) =>
    cells.map((cell) => cell.getAttribute("aria-label") ?? ""),
  );
  expect(labels).toHaveLength(81);
  return labels.map((label) => Number(label.split(", ")[2]) || 0);
}

export function solve(grid: number[]): number[] {
  const solved = [...grid];
  const fits = (index: number, digit: number) => {
    const row = Math.floor(index / 9);
    const column = index % 9;
    for (let other = 0; other < 81; other++) {
      const otherRow = Math.floor(other / 9);
      const otherColumn = other % 9;
      const sameBox =
        Math.floor(row / 3) === Math.floor(otherRow / 3) && Math.floor(column / 3) === Math.floor(otherColumn / 3);
      if ((otherRow === row || otherColumn === column || sameBox) && solved[other] === digit) return false;
    }
    return true;
  };
  const search = (): boolean => {
    const index = solved.indexOf(0);
    if (index === -1) return true;
    for (let digit = 1; digit <= 9; digit++) {
      if (fits(index, digit)) {
        solved[index] = digit;
        if (search()) return true;
      }
    }
    solved[index] = 0;
    return false;
  };
  search();
  return solved;
}

export async function enter(page: Page, index: number, digit: number) {
  await cell(page, index).click();
  await button(page, String(digit)).click();
}

export async function emptyCellAndAnswer(page: Page): Promise<{ index: number; answer: number }> {
  const grid = await readBoard(page);
  const solution = solve(grid);
  const index = grid.indexOf(0);
  return { index, answer: solution[index] };
}

export const wrongDigit = (answer: number) => (answer % 9) + 1;
