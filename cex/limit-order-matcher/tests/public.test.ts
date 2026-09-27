import { expect, test } from "bun:test";
import { OrderBook } from "../src/orderbook";
import type { Order, Side } from "../src/types";

function order(id: number, userId: number, side: Side, price: number, quantity: number): Order {
  return { id, userId, side, price, remainingQuantity: quantity };
}

test("a buy fills an eligible ask at its resting price", () => {
  const book = new OrderBook();
  book.placeLimitOrder(order(1, 10, "SELL", 100, 3));
  const result = book.placeLimitOrder(order(2, 20, "BUY", 105, 3));
  expect(result).toEqual({
    fills: [{ makerOrderId: 1, buyerId: 20, sellerId: 10, price: 100, quantity: 3 }],
    filledQuantity: 3,
    remainingQuantity: 0,
  });
  expect(book.getAsks()).toEqual([]);
  expect(book.getBids()).toEqual([]);
});

test("a sell can match a resting bid", () => {
  const book = new OrderBook();
  book.placeLimitOrder(order(1, 10, "BUY", 105, 2));
  const result = book.placeLimitOrder(order(2, 20, "SELL", 100, 2));
  expect(result.fills).toEqual([
    { makerOrderId: 1, buyerId: 10, sellerId: 20, price: 105, quantity: 2 },
  ]);
  expect(result.remainingQuantity).toBe(0);
});
