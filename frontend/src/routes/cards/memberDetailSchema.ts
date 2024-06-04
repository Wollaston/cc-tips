import { z } from 'zod';

export const memberDetailSchema = z.object({
	eid: z.string()
});

export type FormSchema = typeof memberDetailSchema;
