// import { fail } from "@sveltejs/kit";
// import type { Actions } from "./$types";
//
// export const actions = {
//   login: async ({ cookies, request }) => {
//     const data = await request.formData();
//     const email = data.get("email");
//     const password = data.get("password");
//
//     if (!email) {
//       return fail(400, { email, missing: true });
//     }
//
//     const user = await db.getUser(email);
//
//     if (!user || user.password !== db.hash(password)) {
//       return fail(400, { email, incorrect: true });
//     }
//
//     cookies.set("sessionid", await db.createSession(user), { path: "/" });
//
//     return { success: true };
//   },
//   register: async (event) => {
//     // TODO register the user
//   }
// } satisfies Actions;
