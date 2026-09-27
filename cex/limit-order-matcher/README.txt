Mini Exchange: Limit Orders and Fills

Complete the matching method for a single SOL/USD order book. Orders are kept in memory.
Each order has a unique ID supplied by the caller.

Implement

Complete placeLimitOrder in src/orderbook.ts. Types, input validation, sorted
resting-order insertion, and safe book snapshots are provided. Matching and fill
creation are your work.

Rules

- An incoming BUY matches the lowest-priced resting asks first, while their price is
  at most the buy limit.
- An incoming SELL matches the highest-priced resting bids first, while their price is
  at least the sell limit.
- Every fill executes at the resting maker's price. Its quantity is the smaller of
  the incoming and maker quantities still available.
- Continue across eligible resting orders until the incoming order is filled or no
  eligible maker remains.
- Remove a maker when fully consumed. Leave a partially consumed maker with its reduced
  quantity.
- If the incoming order has quantity left, rest that remainder on its own side at its
  original limit price. No match is a valid result.
- Return the fills in execution order, their total filledQuantity, and the incoming
  remainingQuantity.
- Existing orders at the same price match in their current order. The supplied insertion
  helper preserves that order.

Example: asks of 3 @ 98 and 5 @ 100, followed by a BUY 10 @ 100, produce fills of
3 @ 98 and 5 @ 100. The remaining 2 @ 100 rests as a bid. The fill records must
identify the maker order, buyer, and seller.

Prices and quantities are positive integers. Prices and initial quantities are at most
1,000,000, and at most 1,000 orders are submitted. The supplied inputs have unique
IDs and do not self-trade. There is no fractional amount or rounding rule.

Run

  bun test

No HTTP server, user balance, settlement, fee, cancellation, or second market is needed.
