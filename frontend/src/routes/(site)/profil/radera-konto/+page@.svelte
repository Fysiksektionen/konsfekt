<script lang="ts">
    import Button from "$lib/components/ui/button/button.svelte";
</script>

<div class="mt-20 gap-5 flex flex-col w-4/5 md:w-2/5 items-center">
  <div class="bg-primary relative text-background rounded-xl w-[250px] h-[100px] p-2">
    <p class="flex justify-center items-center text-5xl w-[280px] h-[75px] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-primary text-background rounded-xl text-shadow-2xs text-shadow-accent">
      Konsfekt
    </p>
  </div>
  <div class="w-full flex flex-col gap-3">
    <p>
      För att minimera risken att du förlorar befintligt saldo kan du själv inte ta bort ditt konto. 
    </p>
    <p>
      Om du verkligen inte vill ha kvar ditt konto kan du be att få det borttaget genom knappen nedan.
    </p>
  </div>
  <div class="flex gap-3">
    <Button variant="outline" href="/profil">Tillbaka</Button>
    <Button variant="secondary">Be om att få konto borttaget.</Button>
  </div>
</div>
