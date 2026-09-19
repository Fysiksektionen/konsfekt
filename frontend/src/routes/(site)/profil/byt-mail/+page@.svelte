<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
    import Logo from "$lib/components/Logo.svelte";
	  import type { PageProps } from './$types';

	  let { data }: PageProps = $props();
</script>

<div class="mt-20 gap-5 flex flex-col w-4/5 md:w-2/5 items-center">
  <Logo class="w-[250px] h-auto shrink-0"/>
  <div class="w-full flex flex-col gap-3">
    <p>Du är påväg att byta Gmail-address som är kopplat till Konsfekt.</p>
    <p>
      Din nuvarande Gmail-address är 
      <code class="font-mono font-semibold">
        {data.user.email}
      </code>
    </p>
    <p>
      Notera att du själv måste koppla ifrån Konsfekt från <a class="text-primary font-medium underline underline-offset-4" href="https://myaccount.google.com/connections">tredjeparts-kopplingar</a>
      via Google om du vill fullständiga bytet. Detta är dock inte strikt nödvändigt.
    </p>
  </div>
  <div class="flex gap-3">
    <Button variant="outline" href="/profil">Avbryt</Button>
    <form method="POST" action="/api/auth/change_email">
      <Button type="submit" variant="secondary">Logga in med ny Gmail-address</Button>
    </form>
  </div>
</div>
