import type { Actions, PageServerLoad } from "./$types";
import { urlSchema } from "$schemas/url";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { fail } from "@sveltejs/kit";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(zod(urlSchema))
  };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod(urlSchema));

    if (!form.valid) {
      return fail(400, { form });
    }

    return { form };
  }
};
