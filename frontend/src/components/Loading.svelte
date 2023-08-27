<script>
</script>

<div class="loading-container">
    <span class="loading-text">Loading...</span>
</div>

<!-- CSS -->
<style>
    .loading-container {
        overflow: hidden;
        white-space: nowrap;
    }
    .loading-text {
        display: inline-block;
        animation: scrollText 1s linear infinite;
    }

    @keyframes scrollText {
        from {
            transform: translateX(100%);
        }
        to {
            transform: translateX(-100%);
        }
    }
</style>
