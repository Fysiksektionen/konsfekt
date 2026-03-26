<script>
    import { goto } from "$app/navigation";
    import Button from "$lib/components/ui/button/button.svelte";
    function googleLogin() {
        goto("/api/auth/google");
    }
</script>

<div class="mt-20 gap-5 flex flex-col w-2/5 items-center">
  <div class="bg-primary relative text-background rounded-xl w-[250px] h-[100px] p-2">
    <p class="flex justify-center items-center text-5xl w-[280px] h-[75px] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-primary text-background rounded-xl text-shadow-2xs text-shadow-accent">
      Konsfekt
    </p>
  </div>
  <div class="w-full">
    <p>Välkommen till konsulatets godisskåpsapp! Här kan du handla godis, se ditt saldo och hålla koll på dina köp. Logga in med ditt Google-konto för att komma igång.</p>
  </div>
  <Button variant="secondary" onclick={() => googleLogin()}>Logga in med Google</Button>
</div>



