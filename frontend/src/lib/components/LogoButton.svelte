<script>
    import { invalidateAll } from "$app/navigation";
    import Logo from "./Logo.svelte";
</script>

<a href="/" onclick={() => invalidateAll()} class="flex items-center rounded-md p-1 transition-opacity hover:opacity-80">
    <Logo class="h-14 w-auto"/>
</a>
