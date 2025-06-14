<script lang="ts">
  import { type UrlSchema, urlSchema } from "$schemas/url";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { superForm, type SuperValidated, type Infer } from "sveltekit-superforms";
  import { Field, Control, Label, Description, FieldErrors } from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";

  let { data }: { data: { form: SuperValidated<Infer<UrlSchema>> } } = $props();

  const form = superForm(data.form, {
    validators: zodClient(urlSchema)
  });

  const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
  <Field {form} name="url">
    <Control>
      {#snippet children({ props })}
        <Label>URL</Label>
        <Input {...props} type="url" bind:value={$formData.url} />
      {/snippet}
    </Control>
    <Description>Paste the url you want to shorten</Description>
    <FieldErrors />
  </Field>
</form>

<!--<aside>-->
<!--  <p>Form data:</p>-->
<!--</aside>-->

<!--<SuperDebug data={$formData} />-->
