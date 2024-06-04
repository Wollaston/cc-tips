import { z } from 'zod';

export const formSchema = z.object({
	date: z.string().date(),
	totalSales: z.string(),
	gotabTips: z.string(),
	cashTips: z.string(),
	laborReport: z.custom<File>()
});

export type FormSchema = typeof formSchema;
