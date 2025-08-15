import { z } from "zod";

export const urlSchema = z.object({
  url: z.string().url({ message: "Please enter a valid URL" }).max(2192)
});

export type UrlSchema = typeof urlSchema;
