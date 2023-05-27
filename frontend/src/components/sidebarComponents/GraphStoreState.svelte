<script>
  import { graphStore } from "stores/graphStore";

  let graphState;

  $: {
    graphState = $graphStore;

    console.log("graphState is now: ", graphState);
  }
</script>
