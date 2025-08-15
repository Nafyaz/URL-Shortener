<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

{@render children()}

<style>
  :global(body) {
    background-color: #fdfbd4;
  }
</style>
