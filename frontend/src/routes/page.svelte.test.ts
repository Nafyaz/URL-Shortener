import { describe, test, expect } from 'vitest';
import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/svelte';
import Page from './(app)/+page.svelte';

describe('/+page.ts', () => {
  test('should render h1', () => {
    render(Page);
    expect(screen.getByRole('heading', { level: 1 })).toBeInTheDocument();
  });
});
