import { z } from 'zod';

export const formSchema = z.object({
	name: z.string().min(2).max(50),
	card_id: z.string().length(10)
});

export type FormSchema = typeof formSchema;
