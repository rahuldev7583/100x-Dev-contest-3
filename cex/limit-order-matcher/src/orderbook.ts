import type { MatchResult, Order } from "./types";

export class OrderBook {
  private bids: Order[] = [];
  private asks: Order[] = [];

  getBids(): Order[] {
    return this.bids.map((order) => ({ ...order }));
  }

  getAsks(): Order[] {
    return this.asks.map((order) => ({ ...order }));
  }

  private validate(order: Order): void {
    if (order.side !== "BUY" && order.side !== "SELL") {
      throw new Error("side must be BUY or SELL");
    }
    if (!Number.isSafeInteger(order.price) || order.price <= 0 ||
        !Number.isSafeInteger(order.remainingQuantity) || order.remainingQuantity <= 0) {
      throw new Error("price and quantity must be positive integers");
    }
  }

  // Supplied helper: higher bids and lower asks go first. Existing equal-price
  // orders remain before newly resting orders at that price.
  private rest(order: Order): void {
    const book = order.side === "BUY" ? this.bids : this.asks;
    let index = 0;
    while (index < book.length &&
        (order.side === "BUY" ? book[index].price >= order.price : book[index].price <= order.price)) {
      index++;
    }
    book.splice(index, 0, { ...order });
  }

  placeLimitOrder(order: Order): MatchResult {
    this.validate(order);
    // TODO: match against the opposite book and rest any incoming remainder.
    throw new Error("matching not implemented");
  }
}
