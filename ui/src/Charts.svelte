<script lang="ts">
  import {
    stats_store,
    users_per_hour_map,
    users_per_weekday_map,
    users_per_day_map,
    users_per_feedback_map,
    median_duration_s_per_screen_map,
    median_retakes_per_screen_map,
    has_been_skipped_percent_per_screen_map,
    users_per_locale_map,
    users_per_age_group_map,
    users_per_gender_map,
    users_per_weight_status_map,
  } from "./stats";

  import {
    GaugeChart,
    BarChartSimple,
    DonutChart,
  } from "@carbon/charts-svelte";
  import {
    LegendPositions,
    GaugeTypes,
    ScaleTypes,
  } from "@carbon/charts/interfaces";
  import "@carbon/charts/styles.min.css";
  import "carbon-components/css/carbon-components.min.css";

  export function convert2Hms(seconds: number): string {
    let left = seconds;
    let t = "";

    // hours
    const h: number = Math.floor(seconds / 3600);
    if (h !== 0) {
      left = seconds - h * 3600;
      t += String(h).padStart(2, "0") + "h";
    }

    // minutes
    const m = Math.floor(left / 60);
    if (m === 0) {
      return seconds + "s";
    }

    t += String(m).padStart(2, "0") + "mn";

    return t;
  }
</script>

<div class="wrapper">
  {#if $stats_store.total_users !== undefined}
    <div class="panel">
      <p class="big-text">{$stats_store.total_users}</p>
      <br />sessions
    </div>
    <div class="panel">
      <p class="big-text">{convert2Hms($stats_store.median_duration_s)}</p>
      <br />session time
    </div>
    <div class="panel">
      <GaugeChart
        data={[
          {
            group: "value",
            value: $stats_store.completion_percent,
          },
        ]}
        options={{
          title: "% completion",
          legend: {
            enabled: false,
          },
          gauge: {
            type: GaugeTypes.SEMI,
          },
        }}
      />
    </div>
    <div class="panel">
      <DonutChart
        data={users_per_gender_map($stats_store.users_per_gender)}
        options={{
          title: "gender",
          legend: {
            position: LegendPositions.LEFT,
          },
          donut: {
            center: {
              label: "gender",
            },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={users_per_age_group_map($stats_store.users_per_age_group)}
        options={{
          title: "users per age group",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={users_per_weight_status_map($stats_store.users_per_weight_status)}
        options={{
          title: "bmi status",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <DonutChart
        data={users_per_feedback_map($stats_store.users_per_feedback)}
        options={{
          title: "feedback",
          legend: {
            position: LegendPositions.LEFT,
          },
          donut: {
            center: {
              label: "feedbacks",
            },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <DonutChart
        data={users_per_locale_map($stats_store.users_per_locale)}
        options={{
          title: "locale",
          legend: {
            position: LegendPositions.LEFT,
          },
          donut: {
            center: {
              label: "locales",
            },
          },
        }}
      />
    </div>
    <div class="panel">
      <GaugeChart
        data={[
          {
            group: "value",
            value: $stats_store.agreed_percent,
          },
        ]}
        options={{
          title: "% agreed to t&c",
          legend: {
            enabled: false,
          },
          gauge: {
            type: GaugeTypes.SEMI,
          },
        }}
      />
    </div>
    <div class="panel">
      <GaugeChart
        data={[
          {
            group: "value",
            value: $stats_store.consented_percent,
          },
        ]}
        options={{
          title: "% consent",
          legend: {
            enabled: false,
          },
          gauge: {
            type: GaugeTypes.SEMI,
          },
        }}
      />
    </div>
    <div class="panel">
      <GaugeChart
        data={[
          {
            group: "value",
            value: $stats_store.has_been_printed_percent,
          },
        ]}
        options={{
          title: "% printed",
          legend: {
            enabled: false,
          },
          gauge: {
            type: GaugeTypes.SEMI,
          },
        }}
      />
    </div>
    <div class="panel">
      <GaugeChart
        data={[
          {
            group: "value",
            value: $stats_store.has_been_emailed_percent,
          },
        ]}
        options={{
          title: "% emailed",
          legend: {
            enabled: false,
          },
          gauge: {
            type: GaugeTypes.SEMI,
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={users_per_hour_map($stats_store.users_per_hour)}
        options={{
          title: "users per hour",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={users_per_weekday_map($stats_store.users_per_weekday)}
        options={{
          title: "users per weekday",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel full-wide-panel">
      <BarChartSimple
        data={users_per_day_map($stats_store.users_per_day)}
        options={{
          title: "users per day",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={median_duration_s_per_screen_map(
          $stats_store.median_duration_s_per_screen
        )}
        options={{
          title: "median duration (s) per screen",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={median_retakes_per_screen_map(
          $stats_store.median_retakes_per_screen
        )}
        options={{
          title: "median retakes per screen",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
    <div class="panel half-wide-panel">
      <BarChartSimple
        data={has_been_skipped_percent_per_screen_map(
          $stats_store.has_been_skipped_percent_per_screen
        )}
        options={{
          title: "% of skips per screen",
          legend: {
            enabled: false,
          },
          axes: {
            left: { mapsTo: "value" },
            bottom: { mapsTo: "group", scaleType: ScaleTypes.LABELS },
          },
        }}
      />
    </div>
  {/if}
</div>

<style>
  .wrapper {
    max-width: 1300px;
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
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    grid-auto-rows: minmax(300px, auto);
  }

  .panel {
    /* needed for the flex layout*/
    margin-left: 5px;
    margin-right: 5px;
    flex: 1 1 300px;
  }

  .tall-panel {
    grid-row-end: span 2;
  }

  .half-wide-panel {
    grid-column-end: span 2;
  }

  .full-wide-panel {
    grid-column-end: span 4;
  }

  .header,
  .footer {
    margin-left: 5px;
    margin-right: 5px;
    flex: 0 1 100%;
    grid-column: 1 / -1;
  }

  .wrapper > * {
    background-color: rgb(249, 251, 255);
    border-radius: 20px;
    border: 1px dashed grey;
    padding: 20px;
    font-size: 150%;
    margin-bottom: 10px;
  }

  .big-text {
    font-size: 250%;
  }

  /* We need to set the margin used on flex items to 0 as we have gaps in grid.  */

  @supports (display: grid) {
    .wrapper > * {
      margin: 0;
    }
  }
</style>
