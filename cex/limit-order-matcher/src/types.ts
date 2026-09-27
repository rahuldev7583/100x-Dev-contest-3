export type Side = "BUY" | "SELL";

export type Order = {
  id: number;
  userId: number;
  side: Side;
  price: number;
  remainingQuantity: number;
};

export type Fill = {
  makerOrderId: number;
  buyerId: number;
  sellerId: number;
  price: number;
  quantity: number;
};

export type MatchResult = {
  fills: Fill[];
  filledQuantity: number;
  remainingQuantity: number;
};
