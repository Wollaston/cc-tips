import { z } from 'zod';

export const importSchema = z.object({
	importFile: z.custom<File>()
});

export type FormSchema = typeof importSchema;
