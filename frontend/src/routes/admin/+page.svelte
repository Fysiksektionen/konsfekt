<script lang="ts">
  import type { PageProps } from "./$types.js";
	import * as Card from "$lib/components/ui/card/index.js";
  let { data }: PageProps = $props();
  
</script>

<div class="flex flex-1 flex-col">
	<div class="@container/main flex flex-1 flex-col gap-2">
		<div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
      <div
      	class="*:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card dark:*:data-[slot=card]:bg-card *:data-[slot=card]:shadow-xs @xl/main:grid-cols-2 @5xl/main:grid-cols-4 grid grid-cols-1 gap-4 px-4 *:data-[slot=card]:bg-gradient-to-t lg:px-6"
      >
      	<Card.Root class="@container/card">
      		<Card.Header>
      			<Card.Description>Mest köpta produkt</Card.Description>
      			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
      				{data.bestSellingProd?.name ?? "-"}
      			</Card.Title>
      		</Card.Header>
      		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
      			<div class="line-clamp-1 flex gap-2 font-medium">
      				Totala antalet sålda
      			</div>
      			<div class="text-muted-foreground">
      				{data.bestSellingProd?.total_sold ?? "-"}
            </div>
      		</Card.Footer>
      	</Card.Root>
      	<Card.Root class="@container/card">
      		<Card.Header>
      			<Card.Description>Omsättning</Card.Description>
      			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
              {data.purchasesInfo?.total ?? "-"}kr
      			</Card.Title>
      		</Card.Header>
      		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
      			<div class="line-clamp-1 flex gap-2 font-medium">
              Totala antalet köp
      			</div>
      			<div class="text-muted-foreground">{data.purchasesInfo?.count ?? "-"}</div>
      		</Card.Footer>
      	</Card.Root>
      	<Card.Root class="@container/card">
      		<Card.Header>
      			<Card.Description>Godisskåpskunder</Card.Description>
      			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
      				{data.customerInfo?.count ?? "-"}
      			</Card.Title>
      		</Card.Header>
      		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
      			<div class="line-clamp-1 flex gap-2 font-medium">
              Antalet registrerade på topplistan
      			</div>
      			<div class="text-muted-foreground">{data.customerInfo?.on_leaderboard ?? "-"}</div>
      		</Card.Footer>
      	</Card.Root>
      	<Card.Root class="@container/card">
      		<Card.Header>
      			<Card.Description>Intäkter</Card.Description>
      			<Card.Title class="@[250px]/card:text-3xl text-2xl font-semibold tabular-nums">
      				{data.depositsInfo?.total ?? "-"}kr
      			</Card.Title>
      		</Card.Header>
      		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
      			<div class="line-clamp-1 flex gap-2 font-medium">
      				Medelinsättning
      			</div>
      			<div class="text-muted-foreground">{Math.round(data.depositsInfo?.average) ?? "-"}kr</div>
      		</Card.Footer>
      	</Card.Root>
      </div>
      <!--
			<div class="px-4 lg:px-6">
				<ChartAreaInteractive />
			</div>
      -->
		</div>
	</div>
</div>
