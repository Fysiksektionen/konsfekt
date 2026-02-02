<script lang="ts">
	import { Button } from "$lib/components/ui/button/index.js";
	import { Separator } from "$lib/components/ui/separator/index.js";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { page } from '$app/state';
    import LogoButton from "./LogoButton.svelte";
</script>

<header
	class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear"
>
  <div class="flex justify-between w-full">
	  <div class="flex w-full items-center gap-1 px-4 lg:gap-2 lg:px-6">
	  	<Sidebar.Trigger class="-ml-1" />
	  	<Separator orientation="vertical" class="mx-2 data-[orientation=vertical]:h-4" />
	  	<h1 class="text-base font-medium">{page.url.pathname}</h1>
	  </div>
    <!-- <Button variant="secondary" href="/">Tillbaka till hemsidan</Button>
	  -->
	<div class="scale-75">
		<!-- <Button class="text-3xl bg-primary text-background rounded-md p-2 text-shadow-2xs text-shadow-accent" href="/">
			<p class="text-3xl bg-primary text-background rounded-md p-2 text-shadow-2xs text-shadow-accent">Konsfekt</p>
		</Button> -->
		<LogoButton />
	</div>
  </div>
</header>
