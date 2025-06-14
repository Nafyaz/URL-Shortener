import type { PageServerLoad } from "./$types";
import { urlSchema } from "$schemas/url";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(zod(urlSchema))
  };
};
