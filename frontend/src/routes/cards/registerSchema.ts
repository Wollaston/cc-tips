import { z } from 'zod';

export const registerSchema = z.object({
	name: z.string().min(2).max(50),
	card_id: z.string().length(10),
	eid: z.string().length(4)
});

export type FormSchema = typeof registerSchema;
