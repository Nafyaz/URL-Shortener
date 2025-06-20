<script lang="ts">
  import { type UrlSchema, urlSchema } from "$schemas/url";
  import { zodClient } from "sveltekit-superforms/adapters";
  import { superForm, type SuperValidated, type Infer } from "sveltekit-superforms";
  import * as Form from "$lib/components/ui/form";
  import { Input } from "$lib/components/ui/input";

  let { data }: { data: { form: SuperValidated<Infer<UrlSchema>> } } = $props();

  const form = superForm(data.form, {
    validators: zodClient(urlSchema)
  });

  const { form: formData, enhance } = form;
</script>

<div class="oklch(0.9798 0.0505 104.88) flex h-screen items-center justify-center">
  <form method="POST" class="w-1/3 space-y-6 text-center" use:enhance>
    <Form.Field {form} name="url">
      <Form.Control>
        {#snippet children({ props })}
          <!--          <Form.Label>URL</Form.Label>-->
          <Input {...props} type="url" bind:value={$formData.url} />
        {/snippet}
      </Form.Control>
      <Form.Description>Paste the url you want to shorten</Form.Description>
      <Form.FieldErrors />
    </Form.Field>
    <Form.Button>Shorten</Form.Button>
  </form>
</div>
<!--<aside>-->
<!--  <p>Form data:</p>-->
<!--</aside>-->

<!--<SuperDebug data={$formData} />-->
