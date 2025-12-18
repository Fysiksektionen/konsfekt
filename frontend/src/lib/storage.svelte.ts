export type ProductInCart = { id: string, amount: number };
export const cart: { products: ProductInCart[] } = $state({ products: [] });
