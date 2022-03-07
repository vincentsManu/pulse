<script lang="ts">
  import { kioskLocations } from "./stats";
  import { selectedKiosk } from "./kiosk.js";

  let selected = "";
  console.log(kioskLocations);
</script>

<div class="wrapper">
  <select
    value={selected}
    on:change={(v) => {
      selectedKiosk.update((v) => v);
    }}
  >
    {#each $kioskLocations as kioskLocation}
      <option value={kioskLocation}>
        {kioskLocation == "" ? "All kiosks" : kioskLocation}
      </option>
    {/each}
  </select>
</div>

<style>
  .wrapper {
    max-width: 200px;
    margin: 0 20px;
    /* display: grid;*/
    grid-gap: 10px;
  }

  /* no grid support? */

  .wrapper {
    display: flex;
    flex-wrap: wrap;
  }

  .wrapper {
    display: grid;
    margin: 0 auto;
  }
</style>
