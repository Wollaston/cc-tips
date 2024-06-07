import { z } from 'zod';

export const commissionsSchema = z.object({
	eid: z.number(),
	product_id: z.number()
});

export type FormSchema = typeof commissionsSchema;
