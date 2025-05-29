import { z } from "zod";

export const formSchema = z.object({
  url: z.url().min(2).max(50),
});

export type FormSchema = typeof formSchema;