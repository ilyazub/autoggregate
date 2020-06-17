<script context="module">
  export async function preload({ params, query }) {
    make = query.make || "Opel";

    const response = await this.fetch(
      `http://localhost:8080/cars/${make.toLowerCase()}.json`
    );
    const json = await response.json();

    cars = json;

    return cars;
  }
</script>

<script>
  export let make = "Opel";
  export let cars = [];

  async function submit() {
    const response = await fetch(
      `http://localhost:8080/cars/${make.toLowerCase()}.json`
    );
    const json = await response.json();

    cars = json;

    return cars;
  }
</script>

<section class="bg-white px-8 pt-6 pb-8 mb-4">
  <form class="mb-4" on:submit|preventDefault={submit}>
    <input
      class="bg-white focus:outline-none focus:shadow-outline border
      border-gray-300 rounded-lg py-2 px-4 block w-full appearance-none
      leading-normal"
      type="text"
      placeholder="Filter car makes"
      bind:value={make} />
  </form>

  <ul class="grid gap-4 grid-cols-3">
    {#each cars as car}
      <li class="rounded overflow-hidden shadow-lg hover:bg-gray-300">
        <a href={car.link} target="_blank">
          <img class="w-full" src={car.thumbnail} alt={car.title} />

          <div class="px-6 py-4">
            <div class="font-bold text-xl mb-2">{car.title}</div>
            <p class="text-gray-700 text-base">{car.description}</p>
          </div>
        </a>
      </li>
    {/each}
  </ul>
</section>
