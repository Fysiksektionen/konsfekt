import { z } from "zod";

const imageFileValidator = (file: File | undefined) => {
    if (file) {
        const acceptedTypes = ['image/webp', 'image/jpeg', 'image/png'];
        return acceptedTypes.includes(file.type) 
    }
    return true;
};
 
export const productFormSchema = z.object({
    id: z.number().positive().int().optional(),
    name: z.string().nonempty(),
    price: z.number(),
    description: z.string(),
    stock: z.number().optional().nullable(),
    image: z.instanceof(File).optional().refine(imageFileValidator, {
      message: "The file must be an image of format WebP, JPEG, or PNG"
    }),
    flags: z.object({
        modifiable: z.boolean(),
        new_product: z.boolean(),
        marked_sold_out: z.boolean(),
    }).optional().default({
        modifiable: true,
        new_product: false,
        marked_sold_out: false,
    }),
});
 
export type ProductFormSchema = typeof productFormSchema;
