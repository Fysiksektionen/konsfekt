<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
  import Footer from '$lib/components/Footer.svelte';
</script>

<svelte:head>
	<title>Om Konsfekt</title>
</svelte:head>

<main class="mx-auto max-w-4xl px-6 py-12">
	<!-- Links -->
  <div class="space-y-1.5 mb-12">
      <div class="flex gap-3">
          <Button href="/login" variant="default">Logga in</Button>
          <Button href="/" variant="outline">Huvudsida</Button>
          <Button href="/profil" variant="outline">Min profil</Button>
          <Button href="/faq" variant="outline">FAQ</Button>
      </div>
  </div>

	<div class="space-y-12">
		<div>
			<h1 class="text-3xl font-bold">Välkommen till Konsfekt</h1>
			<p class="mt-2 text-muted-foreground">
				Konsfekt är en app för att hålla reda på ditt godisskåpssaldo.
			</p>
		</div>

		<!-- About -->
		<div class="space-y-8">
			<div>
				<h2 class="text-xl font-semibold">Hur använder jag Konsfekt?</h2>
				<p class="mt-2 text-muted-foreground">
					När du har loggat in visas alla produkter som för närvarande finns till försäljning i godisskåpet. För att köpa en produkt, välj den och klicka sedan på ”<b>Köp</b>”.

                    För att fylla på ditt saldo klickar du på ”<b>+ Lägg till pengar</b>” och följer instruktionerna på skärmen. När du har genomfört betalningen via Swish uppdateras ditt saldo automatiskt.
				</p>
			</div>

			<div>
				<h2 class="text-xl font-semibold">Varför finns Konsfekt?</h2>
				<p class="mt-2 text-muted-foreground">
                    Under året 2025–2026 saknades det <strong>1 501 241 kr</strong> i godisskåpet.
                    Vi tror att en bidragande faktor är att det saknas en enkel och tydlig metod
                    för att hålla koll på sitt saldo.
                </p>
                <p class="mt-2 text-muted-foreground">
                    Därför finns <strong>Konsfekt</strong> – för att göra det enklare att hålla
                    koll på sitt saldo och sina köp.
                </p>
			</div>

			<div>
				<h2 id="anonyma-transaktioner" class="text-xl font-semibold">Vilken information sparas om mig?</h2>
				<p class="mt-2 text-muted-foreground">
                    Normalt kopplas varje köp till ditt konto, så att det syns i din
                    transaktionshistorik att du gjort köpet.
                </p>
                <p class="mt-2 text-muted-foreground">
                    Om du aktiverar <strong>anonyma köp</strong> i din profil sparas köpet
                    istället utan koppling till ditt konto – varken andra användare eller
                    administratörer kan se att köpet var ditt. Produkt, pris och tidpunkt
                    sparas fortfarande, men inte vem som handlade.
                </p>
			</div>
		</div>

		<!-- Features -->
		<div>
			<h2 class="text-xl font-semibold">Konsfekt Features</h2>

			<div class="mt-4 grid gap-4 sm:grid-cols-3">
				<Card>
					<CardHeader>
						<CardTitle>Saldospårning</CardTitle>
					</CardHeader>
					<CardContent>
						<p class="text-sm text-muted-foreground">
							När du handlar genom Konsfekt uppdateras ditt saldo automatiskt. Du slipper hålla koll på varje köp och kan istället fokusera på det viktiga. Som att plugga!
						</p>
					</CardContent>
				</Card>

				<Card>
					<CardHeader>
						<CardTitle>Lagerstatus</CardTitle>
					</CardHeader>
					<CardContent>
                        <p class="text-sm text-muted-foreground">
                            Är Julabollen slut? Med Konsfekt kan du enkelt se aktuell lagerstatus på dina favoritvaror från godisskåpet innan du går dit.
                        </p>
					</CardContent>
				</Card>

				<Card>
					<CardHeader>
						<CardTitle>Swishintegration</CardTitle>
					</CardHeader>
					<CardContent>
						<p class="text-sm text-muted-foreground">
							Slipp uppdatera saldot manuellt. När du fyller på ditt saldo via Swish uppdateras det automatiskt i Konsfekt.
						</p>
					</CardContent>
				</Card>
			</div>
		</div>

        <!-- <div>
			<h2 class="text-xl font-semibold">AAAAAAAAa</h2>
			<p class="mt-2 text-muted-foreground">
				Just nu är Konsfekt i alpha testing.
			</p>
		</div> -->

	</div>
</main>

<Footer/>
