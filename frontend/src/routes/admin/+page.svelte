<script lang="ts">
	import SectionCards from "$lib/components/section-cards.svelte";
	// import ChartAreaInteractive from "$lib/components/chart-area-interactive.svelte";
  import type { PageProps } from "./$types.js";
  let { data }: PageProps = $props();
</script>

<div class="flex flex-1 flex-col">
	<div class="@container/main flex flex-1 flex-col gap-2">
		<div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
			<SectionCards />
      <!--
			<div class="px-4 lg:px-6">
				<ChartAreaInteractive />
			</div>
      -->
		</div>
	</div>
</div>
