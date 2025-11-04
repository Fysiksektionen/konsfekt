<script lang="ts">
    import Input from '$lib/components/ui/input/input.svelte';
    import { createSearchStore, searchData } from '$lib/utils';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
  let searchTerm = $state("");
  let searchStore = $state(createSearchStore(data.products, ["title", "description", "category"]));
  $effect(() => {
    searchData(searchStore, searchTerm);
  });
</script>

<Input bind:value={searchTerm}/>
{#each searchStore.filtered as product}
  <div class="flex gap-3">
    <p>{product.title}</p> 
    <p>{product.description}</p> 
    <p>{product.category}</p> 
  </div>
{/each}
