<script lang="ts">
  import { onMount } from 'svelte';
  import type { UrlResponse } from '$lib/types';
  import Button from './Button.svelte';

  export let url: UrlResponse;

  let copied = false;
  let timeoutId: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    return () => {
      if (timeoutId) clearTimeout(timeoutId);
    };
  });

  function copyToClipboard(): void {
    navigator.clipboard.writeText(url.short_url)
      .then(() => {
        copied = true;

        // Reset the "Copied" state after 2 seconds
        if (timeoutId) clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
          copied = false;
        }, 2000);
      })
      .catch(err => {
        console.error('Failed to copy:', err);
      });
  }
</script>

<div class="bg-white border border-gray-200 rounded-lg shadow-sm p-4 mb-4">
    <div class="flex flex-col">
        <h3 class="text-lg font-medium text-gray-900 mb-2">
            Shortened URL
        </h3>

        <div class="flex items-center mb-3">
            <a
                    href={url.short_url}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-primary-600 hover:underline mr-2 font-medium truncate"
            >
                {url.short_url}
            </a>

            <Button
                    variant="secondary"
                    onClick={copyToClipboard}
            >
                {copied ? 'Copied!' : 'Copy'}
            </Button>
        </div>

        <div class="text-sm text-gray-500">
            <span class="font-medium">Original URL:</span>
            <a
                    href={url.original_url}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:underline truncate block mt-1"
            >
                {url.original_url}
            </a>
        </div>
    </div>
</div>