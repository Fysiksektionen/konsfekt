<script>
    import { goto } from "$app/navigation";
    import Button from "$lib/components/ui/button/button.svelte";
    import Logo from "$lib/components/Logo.svelte";
    function googleLogin() {
        goto("/api/auth/google");
    }
</script>

<div class="mt-20 gap-5 flex flex-col w-2/5 items-center">
  <Logo class="w-[250px] h-auto shrink-0"/>
  <div class="w-full">
    <p>Välkommen till konsulatets godisskåpsapp! Här kan du handla godis, se ditt saldo och hålla koll på dina köp. Logga in med ditt Google-konto för att komma igång.</p>
  </div>
  <Button variant="secondary" onclick={() => googleLogin()}>Logga in med Google</Button>
</div>



