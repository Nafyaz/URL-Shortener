import { z } from "zod";

export const urlSchema = z.object({
  url: z.string().url().min(2).max(50),
});

export type UrlSchema = typeof urlSchema;