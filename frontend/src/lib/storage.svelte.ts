export type Cart = { products: Record<string, number> };
export const cart: Cart = $state({ products: {} });
export type UndoablePurchases = { purchases: Record<number, string> };
export const undoablePurchases: UndoablePurchases = $state({ purchases: {}});
