<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
	  import type { PageProps } from './$types';

	  let { data }: PageProps = $props();
</script>

<div class="mt-20 gap-5 flex flex-col w-4/5 md:w-2/5 items-center">
  <div class="bg-primary relative text-background rounded-xl w-[250px] h-[100px] p-2">
    <p class="flex justify-center items-center text-5xl w-[280px] h-[75px] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-primary text-background rounded-xl text-shadow-2xs text-shadow-accent">
      Konsfekt
    </p>
  </div>
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
    <Button variant="secondary" href="/api/auth/change_email">Logga in med ny Gmail-address</Button>
  </div>
</div>
