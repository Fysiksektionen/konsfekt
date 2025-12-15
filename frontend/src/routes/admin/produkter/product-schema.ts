import { z } from "zod";
import { optional } from "zod/v3";

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
    }).optional().default({
        modifiable: true,
        new_product: false
    }),
});
 
export type ProductFormSchema = typeof productFormSchema;
