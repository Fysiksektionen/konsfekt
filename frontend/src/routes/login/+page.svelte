<script>
    import { goto } from "$app/navigation";
    function googleLogin() {
        goto("/api/auth/google");
    }
</script>
<button onclick={() => googleLogin()}>Logga in med Google</button>
