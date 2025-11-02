<script lang="ts">
	import NavMain from "./nav-main.svelte";
	import NavSecondary from "./nav-secondary.svelte";
	import NavUser from "./nav-user.svelte";
  import DashboardIcon from "@lucide/svelte/icons/layout-dashboard"
  import ProductIcon from "@lucide/svelte/icons/apple"
  import ArrowRightLeft from "@lucide/svelte/icons/arrow-right-left"
  import RoleIcon from "@lucide/svelte/icons/shield-user"
  import ChartIcon from "@lucide/svelte/icons/chart-no-axes-combined"

	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import type { ComponentProps } from "svelte";
  
	let { user, ...restProps }: any & ComponentProps<typeof Sidebar.Root> = $props();

	const sidebarMenu = {
		user: {
			name: user.name,
			email: user.email,
		},
		navMain: [
			{
				title: "Överblick",
				url: "/admin",
				icon: DashboardIcon,
			},
			{
				title: "Produkter",
				url: "/admin/produkter",
				icon: ProductIcon,
			},
			{
				title: "Transaktioner",
				url: "/admin/transaktioner",
				icon: ArrowRightLeft,
			},
			{
				title: "Statistik",
				url: "/admin/statistik",
				icon: ChartIcon,
			},
		],
		navSecondary: [
			{
				title: "Roller",
				url: "/admin/roller",
				icon: RoleIcon,
			},
		],
	};

</script>

<Sidebar.Root collapsible="offcanvas" {...restProps}>
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton class="data-[slot=sidebar-menu-button]:p-1.5!">
					{#snippet child({ props })}
						<a href="/admin" {...props}>
							<span class="text-base font-semibold">Konsfekt adminpanel</span>
						</a>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain items={sidebarMenu.navMain} />
		<NavSecondary items={sidebarMenu.navSecondary} class="mt-auto" />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavUser user={sidebarMenu.user} />
	</Sidebar.Footer>
</Sidebar.Root>
