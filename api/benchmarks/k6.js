import http from "k6/http";
import { uuidv4 } from "https://jslib.k6.io/k6-utils/1.1.0/index.js";
import { check } from "k6";
import encoding from "k6/encoding";
import { Rate } from "k6/metrics";

export let errorRate = new Rate("errors");

export default function () {
  const username = `${__ENV.USER_MANUPULSE}`;
  const password = `${__ENV.PASSWORD_MANUPULSE}`;
  const environment = `${__ENV.ENV_PULSE}`;

  const credentials = `${username}:${password}`;
  const encodedCredentials = encoding.b64encode(credentials);

  var url = "http://localhost:8000/pulse/user_session";
  if (environment == "prod") {
    url =
      "https://manupulse.southeastasia.cloudapp.azure.com/pulse/user_session";
  }
  if (environment == "dev") {
    url = "https://dev.manupulse.com/pulse/user_session";
  }

  var payload = JSON.stringify({
    id: uuidv4(),
    has_consented: true,
    has_accepted_terms_conditions: true,
    has_been_printed: false,
    has_been_emailed: false,
    locale: "zh_Hans_HK",
    user_interactions: [
      {
        screen_id: "1-howitworks",
        started_at: "2021-10-21T17:28:26+00:00",
        ended_at: "2021-10-21T17:28:26+00:00",
        has_been_skipped: false,
      },
      {
        screen_id: "2-termsandconditions",
        started_at: "2021-10-22T17:28:26+00:00",
        ended_at: "2021-10-22T17:29:26+00:00",
        has_been_skipped: true,
        retakes: [
          { retaked_at: "2021-10-22T17:28:26+00:00" },
          { retaked_at: "2021-10-22T17:28:27+00:00" },
          { retaked_at: "2021-10-22T17:28:28+00:00" },
        ],
      },
    ],
    health_data: {
      user_gender: "male",
      user_weight: 0.123456789,
      user_height: 0.123456789,
      user_sys: 55,
      user_dia: 55,
      user_pulse: 55,
      spo2: 55,
      sugar: 55,
      fvc1: 0.123456789,
      fvc2: 0.123456789,
      fev1: 0.123456789,
      fev2: 0.123456789,
      kiosk_location: "HK Tower 1",
      invoice_number: "1234fGGH-HTYHGD",
      report_fee: 55,
      report_date: "2021-10-21T17:28:26+00:00",
      feedback: "happy",
      age: 55,
      status: true,
      insurance: false,
      bmc: 0.123456789,
      bmi: 0.123456789,
      mineral: 0.123456789,
      bfm: 0.123456789,
      bcm: 0.123456789,
      protein: 0.123456789,
      smm: 0.123456789,
      icw: 0.123456789,
      ecw: 0.123456789,
      vfa: 0.123456789,
      whr: 0.123456789,
      pbf: 0.123456789,
      bwa: 0.123456789,
      bmr: 0.123456789,
      dci: 0.123456789,
      ideal_weight: 0.123456789,
      ideal_pbf: 55,
      smm_percentage: 0.123456789,
      lbm_percentage: 0.123456789,
      ideal_protein: 0.123456789,
      ideal_icw: 0.123456789,
      ideal_ecw: 0.123456789,
      min_smm: 55,
      max_smm: 55,
      ideal_smm: 55,
      pulse_score: 0.123456789,
      weightlower: 55,
      weighthigher: 55,
      icllower: 0.123456789,
      iclhigher: 0.123456789,
      ecllower: 0.123456789,
      eclhigher: 0.123456789,
      fatlower: 0.123456789,
      fathigher: 0.123456789,
      bmclower: 0.123456789,
      proteinlower: 0.123456789,
      proteinhigher: 0.123456789,
      bmchigher: 0.123456789,
    },
  });

  var params = {
    headers: {
      "Content-Type": "application/json",
      Authorization: `Basic ${encodedCredentials}`,
    },
  };

  check(http.post(url, payload, params), {
    "status is 201": (r) => r.status == 201,
  }) || errorRate.add(1);
}
