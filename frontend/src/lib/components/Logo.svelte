<script lang="ts">
  let { class: className = "" }: { class?: string } = $props();
</script>

<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1600 620" class={className}>
  <!-- Rounded label, themed with the site's primary color -->
  <rect x="60" y="122" width="1480" height="370" rx="60" fill="var(--primary)"/>
  <rect x="140" y="43" width="1330" height="530" rx="50" fill="var(--primary)"/>

  <!-- Shadow, themed with the site's secondary color -->
  <text
    x="810"
    y="420"
    text-anchor="middle"
    font-family="Arial, Helvetica, sans-serif"
    font-size="310"
    font-weight="400"
    fill="var(--secondary)"
    transform="translate(6,8)">
    Konsfekt
  </text>

  <!-- Main text -->
  <text
    x="810"
    y="420"
    text-anchor="middle"
    font-family="Arial, Helvetica, sans-serif"
    font-size="310"
    font-weight="400"
    fill="#fafafa">
    Konsfekt
  </text>
</svg>
