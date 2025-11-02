<script lang="ts">
	import * as Avatar from "$lib/components/ui/avatar/index.js";
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { goto } from "$app/navigation";

	let { user }: { user: { name: string; email: string } } = $props();

  const initials = user.email.split(".").slice(0, 2).map(n => n.at(0)?.toUpperCase()).join("")
  const emailName = user.email.split("@").at(0)
</script>

<Sidebar.Menu>
	<Sidebar.MenuItem>
		<Sidebar.MenuButton
			size="lg"
			class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
      onclick={() => goto("/profil")}
		>
			<Avatar.Root class="size-8 flex justify-center items-center rounded-lg bg-primary">
        {initials} 
			</Avatar.Root>
			<span class="truncate font-medium">{user.name ?? emailName}</span>
		</Sidebar.MenuButton>
	</Sidebar.MenuItem>
</Sidebar.Menu>
