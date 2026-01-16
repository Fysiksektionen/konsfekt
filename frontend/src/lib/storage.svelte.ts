export type Cart = { products: Record<string, number> };
export const cart: Cart = $state({ products: {} });
