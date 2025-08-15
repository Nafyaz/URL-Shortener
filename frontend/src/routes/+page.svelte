<script lang="ts">
  import { onMount } from 'svelte';
  import { z } from 'zod';
  import Input from '$components/Input.svelte';
  import Button from '$components/Button.svelte';
  import UrlCard from '$components/UrlCard.svelte';
  import { shortenUrl, fetchUrls } from '$lib/services/api';
  import type { UrlResponse } from '$lib/types';
  import { isApiError } from '$lib/types';

  // Form state
  let urlInput = '';
  let urlError = '';
  let isSubmitting = false;

  // Results
  let latestUrl: UrlResponse | null = null;
  let urlList: UrlResponse[] = [];
  let fetchError = '';

  // URL validation schema
  const urlSchema = z.string().trim().min(1, 'URL is required');

  onMount(async () => {
    await loadUrls();
  });

  async function loadUrls() {
    const response = await fetchUrls();

    if (isApiError(response)) {
      fetchError = response.error;
    } else {
      urlList = response;
      fetchError = '';
    }
  }

  async function handleSubmit() {
    // Reset states
    urlError = '';

    // Validate input
    try {
      urlSchema.parse(urlInput);
    } catch (err) {
      if (err instanceof z.ZodError) {
        urlError = err.errors[0].message;
        return;
      }
    }

    // Make sure URL has protocol
    if (!urlInput.startsWith('http://') && !urlInput.startsWith('https://')) {
      urlInput = `https://${urlInput}`;
    }

    // Submit the form
    isSubmitting = true;

    try {
      const response = await shortenUrl(urlInput);

      if (isApiError(response)) {
        urlError = response.error;
      } else {
        latestUrl = response;
        urlInput = '';

        // Refresh the URL list
        await loadUrls();
      }
    } finally {
      isSubmitting = false;
    }
  }
</script>

<svelte:head>
    <title>URL Shortener</title>
    <meta name="description" content="Create short, shareable links quickly and easily" />
</svelte:head>

<div class="min-h-screen bg-gray-50">
    <div class="max-w-3xl mx-auto pt-10 pb-20 px-4">
        <!-- Header -->
        <header class="text-center mb-10">
            <h1 class="text-4xl font-bold text-gray-900 mb-2">URL Shortener</h1>
            <p class="text-lg text-gray-600">Create short, shareable links in seconds</p>
        </header>

        <!-- URL Shortener Form -->
        <div class="bg-white rounded-xl shadow-md p-6 mb-8">
            <form
                    on:submit|preventDefault={handleSubmit}
                    class="space-y-4"
            >
                <Input
                        id="url-input"
                        label="Enter a long URL to shorten"
                        placeholder="https://example.com/very/long/url/that/needs/shortening"
                        bind:value={urlInput}
                        error={urlError}
                        required={true}
                        on:keypress={(e) => e.key === 'Enter' && handleSubmit()}
                />

                <Button
                        type="submit"
                        disabled={isSubmitting}
                        loading={isSubmitting}
                        fullWidth={true}
                >
                    {isSubmitting ? 'Shortening...' : 'Shorten URL'}
                </Button>
            </form>
        </div>

        <!-- Latest shortened URL -->
        {#if latestUrl}
            <div class="mb-10">
                <h2 class="text-xl font-semibold text-gray-800 mb-4">Your shortened URL</h2>
                <UrlCard url={latestUrl} />
            </div>
        {/if}

        <!-- URL List -->
        {#if urlList.length > 0}
            <div>
                <h2 class="text-xl font-semibold text-gray-800 mb-4">Recently shortened URLs</h2>

                <div class="space-y-4">
                    {#each urlList as url}
                        <UrlCard {url} />
                    {/each}
                </div>
            </div>
        {:else if fetchError}
            <div class="text-center p-6 bg-red-50 rounded-lg">
                <p class="text-red-600">{fetchError}</p>
            </div>
        {:else}
            <div class="text-center p-10">
                <p class="text-gray-500">No URLs have been shortened yet.</p>
            </div>
        {/if}
    </div>
</div>

<style lang="postcss">
    /* Global styles will be processed by PostCSS/Tailwind */
    :global(html) {
        @apply antialiased;
    }
</style>