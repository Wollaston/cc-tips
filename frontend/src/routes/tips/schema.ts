import { z } from 'zod';

export const tipsSchema = z.object({
	startDate: z.string().refine((v) => v, { message: 'A start date is required.' }),
	endDate: z.string().refine((v) => v, { message: 'An end date is required.' })
});

export type FormSchema = typeof tipsSchema;
