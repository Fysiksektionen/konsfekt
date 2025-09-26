<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Konsfekt</h1>
<p>Konsulatets egna konfektyr shop. Håll koll på ditt godisskåp saldo</p>

{data.user.email}
