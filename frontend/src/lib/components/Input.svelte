<script lang="ts">
  export let id: string;
  export let label: string = '';
  export let type: string = 'text';
  export let placeholder: string = '';
  export let value: string = '';
  export let error: string = '';
  export let required: boolean = false;

  // Two-way binding for value
  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    value = target.value;
  }
</script>

<div class="mb-4">
    {#if label}
        <label for={id} class="block mb-2 text-sm font-medium text-gray-700">{label}</label>
    {/if}

    <input
            {id}
            {type}
            {placeholder}
            {required}
            class="w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-primary-500 {error ? 'border-red-500' : ''}"
            value={value}
            on:input={handleInput}
            on:keypress
    />

    {#if error}
        <p class="mt-1 text-sm text-red-600">{error}</p>
    {/if}
</div>