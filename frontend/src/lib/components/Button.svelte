<script lang="ts">
export let type: 'button' | 'submit' = 'button';
export let variant: 'primary' | 'secondary' = 'primary';
export let disabled = false;
export let fullWidth = false;
export let loading = false;
export let onClick: (() => void) | undefined = undefined;

const variantClasses = {
  primary: 'bg-primary-600 hover:bg-primary-700 text-white',
  secondary: 'bg-gray-200 hover:bg-gray-300 text-gray-800',
};
</script>

<button
{type}
class="px-4 py-2 rounded font-medium transition-colors {variantClasses[variant]} {fullWidth ? 'w-full' : ''} {disabled || loading ? 'opacity-70 cursor-not-allowed' : ''}"
on:click={onClick}
{disabled}
>
{#if loading}
<div class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite] mr-2"></div>
{/if}
<slot />
</button>