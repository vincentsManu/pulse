type Stats = {
  total_users: number;
  median_duration_s: number;
  users_per_hour: UsersPerHour[];
  users_per_day: UsersPerDay[];
  users_per_weekday: UsersPerWeekday[];
  completion_percent: number;
  consented_percent: number;
  agreed_percent: number;
  has_been_printed_percent: number;
  has_been_emailed_percent: number;
  median_duration_s_per_screen: MedianDurationSPerScreen[];
  median_retakes_per_screen: MedianRetakesPerScreen[];
  has_been_skipped_percent_per_screen: HasBeenSkippedPercentPerScreen[];
  users_per_feedback: UsersPerFeedback[];
  users_per_locale: UsersPerLocale[];
  users_per_gender: UsersPerGender[];
  users_per_weight_status: UsersPerWeightStatus[];
  users_per_age_group: UsersPerAgeGroup[];
};

type UsersPerHour = {
  hour: number;
  count: number;
};

type UsersPerDay = {
  date: string;
  count: number;
};

type UsersPerWeekday = {
  weekday: string;
  count: number;
};

type MedianDurationSPerScreen = {
  screen_id: number;
  median_duration_s: number;
};

type MedianRetakesPerScreen = {
  screen_id: number;
  median_retakes: number;
};

type HasBeenSkippedPercentPerScreen = {
  screen_id: number;
  has_been_skipped_percent: number;
};

type UsersPerFeedback = {
  feedback: string;
  count: number;
};

type UsersPerLocale = {
  locale: string;
  count: number;
};

type UsersPerGender = {
  gender: string;
  count: number;
};

type UsersPerWeightStatus = {
  weight_status: string;
  count: number;
};

type UsersPerAgeGroup = {
  age_group: string;
  count: number;
};

let domain_url_switch = "DOMAIN_URL_SWITCH";

let base_url;
if (domain_url_switch === "relative") {
  base_url = window.location.origin;
} else {
  base_url = "http://localhost:8000";
}

export async function getStats(): Promise<Stats> {
  const res = await fetch(base_url + `pulstats/stats`);
  const stats = await res.json();

  if (res.ok) {
    return stats;
  } else {
    throw new Error(stats);
  }
}

import { readable } from "svelte/store";

export const stats_store = readable(<Stats>{}, (set) => {
  const eventSource = new EventSource(base_url + `/pulstats/stats/listen`);

  eventSource.onmessage = (e) => {
    try {
      set(<Stats>JSON.parse(e.data));
    } catch (e) {
      console.log(e.data); // error in the above string (in this case, yes)!
    }
  };

  return function stop() {
    eventSource.close();
  };
});

type BarChartItem = {
  group: string;
  value: number;
};

// user per hour to graph
export function users_per_hour_map(
  users_per_hour: UsersPerHour[]
): BarChartItem[] {
  return users_per_hour.map(
    (uh) => <BarChartItem>{ group: String(uh.hour) + "h", value: uh.count }
  );
}

// user per weekday to graph
export function users_per_weekday_map(
  users_per_weekday: UsersPerWeekday[]
): BarChartItem[] {
  return users_per_weekday.map(
    (uwd) => <BarChartItem>{ group: uwd.weekday, value: uwd.count }
  );
}

// user per day to graph
export function users_per_day_map(
  users_per_day: UsersPerDay[]
): BarChartItem[] {
  return users_per_day.map(
    (ud) => <BarChartItem>{ group: ud.date, value: ud.count }
  );
}

// user per day to graph
export function users_per_feedback_map(
  users_per_feedback: UsersPerFeedback[]
): BarChartItem[] {
  return users_per_feedback.map(
    (ud) => <BarChartItem>{ group: ud.feedback, value: ud.count }
  );
}

// user per day to graph
export function users_per_locale_map(
  users_per_locale: UsersPerLocale[]
): BarChartItem[] {
  return users_per_locale.map(
    (ud) => <BarChartItem>{ group: ud.locale, value: ud.count }
  );
}

// user per gender
export function users_per_gender_map(
  users_per_gender: UsersPerGender[]
): BarChartItem[] {
  return users_per_gender.map(
    (ud) => <BarChartItem>{ group: ud.gender, value: ud.count }
  );
}

// user per weight status
export function users_per_weight_status_map(
  users_per_weight_status: UsersPerWeightStatus[]
): BarChartItem[] {
  return users_per_weight_status.map(
    (ud) => <BarChartItem>{ group: ud.weight_status, value: ud.count }
  );
}

// user per age groups
export function users_per_age_group_map(
  users_per_age_group: UsersPerAgeGroup[]
): BarChartItem[] {
  return users_per_age_group.map(
    (ud) => <BarChartItem>{ group: ud.age_group, value: ud.count }
  );
}

// screens
// user per day to graph
export function median_duration_s_per_screen_map(
  median_duration_s_per_screen: MedianDurationSPerScreen[]
): BarChartItem[] {
  return median_duration_s_per_screen.map(
    (m) =>
      <BarChartItem>{ group: String(m.screen_id), value: m.median_duration_s }
  );
}

export function median_retakes_per_screen_map(
  median_retakes_per_screen: MedianRetakesPerScreen[]
): BarChartItem[] {
  return median_retakes_per_screen.map(
    (m) => <BarChartItem>{ group: String(m.screen_id), value: m.median_retakes }
  );
}

export function has_been_skipped_percent_per_screen_map(
  has_been_skipped_percent_per_screen: HasBeenSkippedPercentPerScreen[]
): BarChartItem[] {
  return has_been_skipped_percent_per_screen.map(
    (m) =>
      <BarChartItem>{
        group: String(m.screen_id),
        value: m.has_been_skipped_percent,
      }
  );
}
