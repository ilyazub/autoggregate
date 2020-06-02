<script context="module">
  export function preload({ params, query }) {
    return this.fetch(`insta.json`)
      .then((r) => r.json())
      .then((posts) => {
        return { posts };
      });
  }
</script>

<script>
  export let posts;
</script>

<svelte:head>
  <title>Insta</title>
</svelte:head>

<h1>Recent posts</h1>

<ul class="flex flex-wrap -mx-2 overflow-hidden">
  {#each posts as post}
    <!-- we're using the non-standard `rel=prefetch` attribute to
				tell Sapper to load the data for the page as soon as
				the user hovers over the link or taps it, instead of
				waiting for the 'click' event -->
    <li class="my-2 px-2 w-1/3 overflow-hidden">
      <a rel="prefetch" href="insta/{post.id}">
        <img src={post.image} alt={post.slug} />
      </a>
    </li>
  {/each}
</ul>
