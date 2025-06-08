import { z } from "zod";

const schema = z.object({
  name: z.string().default("Hello world!"),
  email: z.string().email()
});
