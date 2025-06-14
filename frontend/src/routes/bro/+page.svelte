<script lang="ts">
  import { enhance } from "$app/forms";
  import type { PageProps } from "./$types";
  let { form }: PageProps = $props();
</script>

<form method="POST" use:enhance></form>
