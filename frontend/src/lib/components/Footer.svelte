<script lang="ts">
  const year = new Date().getFullYear();
</script>

<footer class="mt-12 w-full border-t border-border">
  <div class="mx-auto flex w-full max-w-5xl flex-col gap-8 px-4 py-10 sm:flex-row sm:justify-between">
    <div class="flex flex-col gap-2">
      <span class="text-2xl font-bold tracking-tight text-card-foreground">Konsfekt</span>
      <p class="max-w-xs text-sm text-muted-foreground">
        Konsulatets godisskåp. Köp godis och fyll på ditt saldo direkt i appen.
      </p>
    </div>

    <div class="flex flex-col gap-2 text-sm">
      <span class="font-semibold text-card-foreground">Navigera</span>
      <a href="/" class="text-muted-foreground hover:text-foreground">Butik</a>
      <a href="/profil" class="text-muted-foreground hover:text-foreground">Min profil</a>
      <a href="/om" class="text-muted-foreground hover:text-foreground">Om denna sida</a>
    </div>

    <div class="flex flex-col gap-2 text-sm">
      <span class="font-semibold text-card-foreground">Kontakt</span>
      <a href="mailto:konsfekt@fysiksektionen.se" class="text-muted-foreground hover:text-foreground">
        konsulatet@fysiksektionen.se
      </a>
      <span class="text-muted-foreground">Fysiksektionen, KTH</span>
    </div>
  </div>

  <div class="border-t border-border px-4 py-4 text-center text-xs text-muted-foreground">
    Konsfekt | Fysiksektionens godisskåp
  </div>
</footer>
