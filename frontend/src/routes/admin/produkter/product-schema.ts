import { z } from "zod";

const imageFileValidator = (file: File | undefined) => {
    if (file) {
        const acceptedTypes = ['image/webp', 'image/jpeg', 'image/png'];
        return acceptedTypes.includes(file.type) 
    }
    return true;
};
 
export const productFormSchema = z.object({
    name: z.string().nonempty(),
    price: z.number(),
    description: z.string(),
    stock: z.number().optional(),
    image: z.instanceof(File).optional().refine(imageFileValidator, {
      message: "The file must be an image of format WebP, JPEG, or PNG"
    })
});
 
export type ProductFormSchema = typeof productFormSchema;
