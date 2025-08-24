import type { Actions, PageServerLoad } from "./$types";
import { urlSchema } from "$schemas/url";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { fail } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";

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

    const apiUrl = env.API_URL;
    if (!apiUrl) {
      throw new Error("Missing API_URL in .env file");
    }

    const res = await fetch(`${apiUrl}/shorten`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        url: form.data.url
      })
    });

    if (!res.ok) {
      return fail(res.status, { form, error: await res.text() });
    }

    const result = await res.json();
    return { form, short_url: result.short_url };
  }
} satisfies Actions;
