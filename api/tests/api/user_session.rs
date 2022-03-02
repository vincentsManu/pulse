use crate::helpers::spawn_app;
use pulstats::user_session::UserSession;

#[tokio::test]
async fn post_user_session_returns_a_201_for_valid_json() {
    // Arrange
    let app = spawn_app().await;
    let bodies = vec![
        r#"
        {
          "id": "79a5016e-fb57-4cf1-93f2-6eb323e65ae3",
          "has_consented": false,
          "has_accepted_terms_conditions": false,
          "has_been_printed": false,
          "has_been_emailed": false,
          "locale": "zh_Hans_HK"
        }
        "#,
        r#"
        {
            "id": "80a5016e-fb57-4cf1-93f2-6eb323e65ae3",
            "has_consented": true,
            "has_accepted_terms_conditions": true,
            "has_been_printed": false,
            "has_been_emailed": false,
            "locale": "zh_Hans_HK",
            "user_interactions": [
                {
                    "screen_id": "1-howitworks",
                    "started_at": "2021-10-21T17:28:26+00:00",
                    "ended_at": "2021-10-21T17:28:26+00:00",
                    "has_been_skipped": false,
                    "retakes": null
                },
                {
                    "screen_id": "2-termsandconditions",
                    "started_at": "2021-10-22T17:28:26+00:00",
                    "ended_at": "2021-10-22T17:29:26+00:00",
                    "has_been_skipped": false,
                    "retakes": [
                        { "retaked_at": "2021-10-22T17:28:26+00:00" },
                        { "retaked_at": "2021-10-22T17:28:27+00:00" },
                        { "retaked_at": "2021-10-22T17:28:28+00:00" }
                    ]
                }
            ],
            "health_data": {
              "user_gender": "male",
              "user_weight": 0.123456789,
              "user_height": 0.123456789,
              "user_sys": 55,
              "user_dia": 55,
              "user_pulse": 55,
              "spo2": 55,
              "sugar": 55,
              "fvc1": 0.123456789,
              "fvc2": 0.123456789,
              "fev1": 0.123456789,
              "fev2": 0.123456789,
              "kiosk_location": "HK Tower 1",
              "invoice_number": "1234fGGH-HTYHGD",
              "report_fee": 55,
              "report_date": "2021-10-21T17:28:26+00:00",
              "feedback": "veryhappy",
              "age": 55,
              "status": true,
              "insurance": false,
              "bmc": 0.123456789,
              "bmi": 0.123456789,
              "mineral": 0.123456789,
              "bfm": 0.123456789,
              "bcm": 0.123456789,
              "protein": 0.123456789,
              "smm": 0.123456789,
              "icw": 0.123456789,
              "ecw": 0.123456789,
              "vfa": 0.123456789,
              "whr": 0.123456789,
              "pbf": 0.123456789,
              "bwa": 0.123456789,
              "bmr": 0.123456789,
              "dci": 0.123456789,
              "ideal_weight": 0.123456789,
              "ideal_pbf": 55,
              "smm_percentage": 0.123456789,
              "lbm_percentage": 0.123456789,
              "ideal_protein": 0.123456789,
              "ideal_icw": 0.123456789,
              "ideal_ecw": 0.123456789,
              "min_smm": 55,
              "max_smm": 55,
              "ideal_smm": 55,
              "pulse_score": 0.123456789,
              "weightlower": 55,
              "weighthigher": 55,
              "icllower": 0.123456789,
              "iclhigher": 0.123456789,
              "ecllower": 0.123456789,
              "eclhigher": 0.123456789,
              "fatlower": 0.123456789,
              "fathigher": 0.123456789,
              "bmclower": 0.123456789,
              "proteinlower": 0.123456789,
              "proteinhigher": 0.123456789,
              "bmchigher": 0.123456789
            }
          }"#,
    ];

    // Act
    for body in bodies {
        let response = app.post_user_session(body.into()).await;

        // Assert
        assert_eq!(201, response.status().as_u16(), "found response {:?}", response.text().await);
    }
}

// #[tokio::test]
// async fn post_user_session_test() {
//     // Arrange
//     let app = spawn_app().await;
//     let bodies = vec![
//         r#"
// {"id":"6a0ba276-d85a-b488-28b6-89ed85ec22ad","has_consented":false,"has_accepted_terms_conditions":false,"health_data":{"user_gender":"male","user_weight":0.0,"user_height":0.0,"user_sys":0,"user_dia":0,"user_pulse":0,"spo2":0,"sugar":0,"fvc1":0.0,"fvc2":0.0,"fev1":0.0,"fev2":0.0,"kiosk_location":"HK Tower 1","invoice_number":"1234fGGH - HTYHGD","report_fee":0,"report_date":"2021-12-07T05:24:27.9533707+05:30","feedback":"satisfactory","age":39,"status":false,"insurance":false,"bmc":0.0,"bmi":0.0,"mineral":0.0,"bfm":0.0,"bcm":0.0,"protein":0.0,"smm":0.0,"icw":0.0,"ecw":0.0,"vfa":0.0,"whr":0.0,"pbf":0.0,"bwa":0.0,"bmr":0.0,"dci":0.0,"ideal_weight":0.0,"ideal_pbf":0,"smm_percentage":0.0,"lbm_percentage":0.0,"ideal_protein":0.0,"ideal_icw":0.0,"ideal_ecw":0.0,"min_smm":0,"max_smm":0,"ideal_smm":0,"pulse_score":0.0,"weightlower":0,"weighthigher":0,"icllower":0.0,"iclhigher":0.0,"ecllower":0.0,"eclhigher":0.0,"fatlower":0.0,"fathigher":0.0,"bmclower":0.0,"proteinlower":0.0,"proteinhigher":0.0,"bmchigher":0.0},"user_interactions":[{"screen_id":"1-howitworks","started_at":"2021-12-07T05:23:41.295118+05:30","ended_at":"2021-12-07T05:23:41.3081084+05:30","has_been_skipped":false},{"screen_id":"1-howitworks","started_at":"2021-12-07T05:23:41.4010502+05:30","ended_at":"2021-12-07T05:23:44.2399157+05:30","has_been_skipped":false},{"screen_id":"2-termsandconditions","started_at":"2021-12-07T05:23:44.2409147+05:30","ended_at":"2021-12-07T05:23:47.3298855+05:30","has_been_skipped":false},{"screen_id":"3-privacy","started_at":"2021-12-07T05:23:47.3298855+05:30","ended_at":"2021-12-07T05:23:47.7324581+05:30","has_been_skipped":false},{"screen_id":"4-enter-name","started_at":"2021-12-07T05:23:47.7324581+05:30","ended_at":"2021-12-07T05:23:51.0451708+05:30","has_been_skipped":false},{"screen_id":"5-enter-dob","started_at":"2021-12-07T05:23:51.0451708+05:30","ended_at":"2021-12-07T05:23:56.7047631+05:30","has_been_skipped":false},{"screen_id":"6-gender","started_at":"2021-12-07T05:23:56.7057613+05:30","ended_at":"2021-12-07T05:23:58.3660079+05:30","has_been_skipped":false},{"screen_id":"7-exercise","started_at":"2021-12-07T05:23:58.3669987+05:30","ended_at":"2021-12-07T05:23:59.9747112+05:30","has_been_skipped":false},{"screen_id":"80-beforeheightweight","started_at":"2021-12-07T05:23:59.9747112+05:30","ended_at":"2021-12-07T05:24:00.7302578+05:30","has_been_skipped":false},{"screen_id":"8-heightweightstart","started_at":"2021-12-07T05:24:00.7302578+05:30","ended_at":"2021-12-07T05:24:01.0680533+05:30","has_been_skipped":false},{"screen_id":"9-displayheightweight","started_at":"2021-12-07T05:24:01.0680533+05:30","ended_at":"2021-12-07T05:24:01.8444129+05:30","has_been_skipped":false},{"screen_id":"10-startbca","started_at":"2021-12-07T05:24:01.8444129+05:30","ended_at":"2021-12-07T05:24:21.8789198+05:30","has_been_skipped":false},{"screen_id":"101-step2bca","started_at":"2021-12-07T05:24:21.8789198+05:30","ended_at":"2021-12-07T05:24:27.9693496+05:30","has_been_skipped":false},{"screen_id":"102-displaybca","started_at":"2021-12-07T05:24:27.9693496+05:30","ended_at":"2021-12-07T05:24:31.5968606+05:30","has_been_skipped":false},{"screen_id":"103-afterbcacompleted","started_at":"2021-12-07T05:24:31.5978613+05:30","ended_at":"2021-12-07T05:24:33.5081707+05:30","has_been_skipped":false},{"screen_id":"11-startbp","started_at":"2021-12-07T05:24:33.5081707+05:30","ended_at":"2021-12-07T05:24:35.1110509+05:30","has_been_skipped":true},{"screen_id":"12-startspo2","started_at":"2021-12-07T05:24:35.1120517+05:30","ended_at":"2021-12-07T05:24:35.8143619+05:30","has_been_skipped":true},{"screen_id":"122-displayspo2","started_at":"2021-12-07T05:24:35.8143619+05:30","ended_at":"2021-12-07T05:24:37.0118603+05:30","has_been_skipped":false},{"screen_id":"0-generatereport","started_at":"2021-12-07T05:24:37.0118603+05:30","ended_at":"2021-12-07T05:24:41.4145841+05:30","has_been_skipped":false},{"screen_id":"13-displaypulsescore","started_at":"2021-12-07T05:24:41.4155819+05:30","ended_at":"2021-12-07T05:24:43.2460292+05:30","has_been_skipped":false},{"screen_id":"14-provide-email-address","started_at":"2021-12-07T05:24:43.2460292+05:30","ended_at":"2021-12-07T05:24:47.2487408+05:30","has_been_skipped":false}],"has_been_printed":false,"has_been_emailed":true,"locale":"en_US"}
//         "#,
//     ];

//     // Act
//     for body in bodies {
//         let response = app.post_user_session(body.into()).await;

//         // Assert
//         assert_eq!(201, response.status().as_u16(), "found response {:?}", response.text().await);
//     }
// }

#[tokio::test]
async fn post_user_session_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let faulty_bodies = vec![
        (
            r#"{
                "has_consented":true,
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK"
            }"#,
            "missing id",
        ),
        (
            r#"{
                "id":"79a5016e-fb57-4cf1-93f2-6eb323e65ae3",
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK"
            }"#,
            "missing consent",
        ),
        (
            r#"{
                "id":"79a5016e-fb57-4cf1-93f2-6eb323e65ae3",
                "has_consented":true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK"
            }"#,
            "missing t&c",
        ),
        (
            r#"{
              "id":"79a5016e-fb57-4cf1-93f2-6eb323e65ae3",
              "has_accepted_terms_conditions": true,
              "has_consented":true,
              "has_been_printed": false,
              "has_been_emailed": false,
          }"#,
            "missing locale",
        ),
        (
            r#"{
                "id":"79a5016e-fb57-4cf1-93f2-6eb323e65ae4",
                "has_consented: true,
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK"
            }"#,
            "malformed json",
        ),
        ("", "missing body"),
        (
            r#"{
                "id":"79a5016e-fb57-4cf1-93f2-6eb323e65ae5",
                "has_consented":true,
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK"
            }"#,
            "consented, accepted t&c but missing data",
        ),
        (
            r#"{
                "id": "79a5016e-fb57-4cf1-93f2-6eb323e65ae6",
                "has_consented": false,
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK",
                "health_data": {
                  "user_gender": "male",
                  "user_weight": 0.123456789,
                  "user_height": 0.123456789,
                  "user_sys": 55,
                  "user_dia": 55,
                  "user_pulse": 55,
                  "spo2": 55,
                  "sugar": 55,
                  "fvc1": 0.123456789,
                  "fvc2": 0.123456789,
                  "fev1": 0.123456789,
                  "fev2": 0.123456789,
                  "kiosk_location": "HK Tower 1",
                  "invoice_number": "1234fGGH-HTYHGD",
                  "report_fee": 55,
                  "report_date": "2021-10-21T17:28:26+00:00",
                  "feedback": "happy",
                  "age": 55,
                  "status": true,
                  "insurance": false,
                  "bmc": 0.123456789,
                  "bmi": 0.123456789,
                  "mineral": 0.123456789,
                  "bfm": 0.123456789,
                  "bcm": 0.123456789,
                  "protein": 0.123456789,
                  "smm": 0.123456789,
                  "icw": 0.123456789,
                  "ecw": 0.123456789,
                  "vfa": 0.123456789,
                  "whr": 0.123456789,
                  "pbf": 0.123456789,
                  "bwa": 0.123456789,
                  "bmr": 0.123456789,
                  "dci": 0.123456789,
                  "ideal_weight": 0.123456789,
                  "ideal_pbf": 55,
                  "smm_percentage": 0.123456789,
                  "lbm_percentage": 0.123456789,
                  "ideal_protein": 0.123456789,
                  "ideal_icw": 0.123456789,
                  "ideal_ecw": 0.123456789,
                  "min_smm": 55,
                  "max_smm": 55,
                  "ideal_smm": 55,
                  "pulse_score": 0.123456789,
                  "weightlower": 55,
                  "weighthigher": 55,
                  "icllower": 0.123456789,
                  "iclhigher": 0.123456789,
                  "ecllower": 0.123456789,
                  "eclhigher": 0.123456789,
                  "fatlower": 0.123456789,
                  "fathigher": 0.123456789,
                  "bmclower": 0.123456789,
                  "proteinlower": 0.123456789,
                  "proteinhigher": 0.123456789,
                  "bmchigher": 0.123456789
                }
              }"#,
            "not consented with health data",
        ),
        (
            r#"{
                "id": "79a5016e-fb57-4cf1-93f2-6eb323e65ae6",
                "has_consented": true,
                "has_accepted_terms_conditions": false,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK",
                "health_data": {
                  "user_gender": "male",
                  "user_weight": 0.123456789,
                  "user_height": 0.123456789,
                  "user_sys": 55,
                  "user_dia": 55,
                  "user_pulse": 55,
                  "spo2": 55,
                  "sugar": 55,
                  "fvc1": 0.123456789,
                  "fvc2": 0.123456789,
                  "fev1": 0.123456789,
                  "fev2": 0.123456789,
                  "kiosk_location": "HK Tower 1",
                  "invoice_number": "1234fGGH-HTYHGD",
                  "report_fee": 55,
                  "report_date": "2021-10-21T17:28:26+00:00",
                  "feedback": "happy",
                  "age": 55,
                  "status": true,
                  "insurance": false,
                  "bmc": 0.123456789,
                  "bmi": 0.123456789,
                  "mineral": 0.123456789,
                  "bfm": 0.123456789,
                  "bcm": 0.123456789,
                  "protein": 0.123456789,
                  "smm": 0.123456789,
                  "icw": 0.123456789,
                  "ecw": 0.123456789,
                  "vfa": 0.123456789,
                  "whr": 0.123456789,
                  "pbf": 0.123456789,
                  "bwa": 0.123456789,
                  "bmr": 0.123456789,
                  "dci": 0.123456789,
                  "ideal_weight": 0.123456789,
                  "ideal_pbf": 55,
                  "smm_percentage": 0.123456789,
                  "lbm_percentage": 0.123456789,
                  "ideal_protein": 0.123456789,
                  "ideal_icw": 0.123456789,
                  "ideal_ecw": 0.123456789,
                  "min_smm": 55,
                  "max_smm": 55,
                  "ideal_smm": 55,
                  "pulse_score": 0.123456789,
                  "weightlower": 55,
                  "weighthigher": 55,
                  "icllower": 0.123456789,
                  "iclhigher": 0.123456789,
                  "ecllower": 0.123456789,
                  "eclhigher": 0.123456789,
                  "fatlower": 0.123456789,
                  "fathigher": 0.123456789,
                  "bmclower": 0.123456789,
                  "proteinlower": 0.123456789,
                  "proteinhigher": 0.123456789,
                  "bmchigher": 0.123456789
                }
              }"#,
            "t&c not accepted with health data",
        ),
        (
            r#"{
                "id": "79a5016e-fb57-4cf1-93f2-6eb323e65ae6",
                "has_consented": false,
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK",
                "user_interactions": [
                    {
                      "screen_id": "1-howitworks",
                      "started_at": "2021-10-21T17:28:26+00:00",
                      "ended_at": "2021-10-21T17:28:26+00:00",
                      "has_been_skipped": false,
                    },
                    {
                      "screen_id": "2-termsandconditions",
                      "started_at": "2021-10-22T17:28:26+00:00",
                      "ended_at": "2021-10-22T17:29:26+00:00",
                      "has_been_skipped": false,
                      "retakes": [
                        { "retaked_at": "2021-10-22T17:28:26+00:00" },
                        { "retaked_at": "2021-10-22T17:28:27+00:00" },
                        { "retaked_at": "2021-10-22T17:28:28+00:00" }
                      ]
                    }
                  ]
              }"#,
            "not consented with user interaction data",
        ),
        (
            r#"{
                "id": "79a5016e-fb57-4cf1-93f2-6eb323e65ae6",
                "has_consented": true,
                "has_accepted_terms_conditions": false,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK",
                "user_interactions": [
                    {
                      "screen_id": "1-howitworks",
                      "started_at": "2021-10-21T17:28:26+00:00",
                      "ended_at": "2021-10-21T17:28:26+00:00",
                      "has_been_skipped": false
                    },
                    {
                      "screen_id": "2-termsandconditions",
                      "started_at": "2021-10-22T17:28:26+00:00",
                      "ended_at": "2021-10-22T17:29:26+00:00",
                      "has_been_skipped": false,
                      "retakes": [
                        { "retaked_at": "2021-10-22T17:28:26+00:00" },
                        { "retaked_at": "2021-10-22T17:28:27+00:00" },
                        { "retaked_at": "2021-10-22T17:28:28+00:00" }
                      ]
                    }
                  ]
              }"#,
            "t&c not accepted with user interaction data",
        ),
        (
            r#"
            {
                "id": "79a5016e-fb57-4cf1-93f2-6eb323e65ae7",
                "has_consented": true,
                "has_accepted_terms_conditions": true,
                "has_been_printed": false,
                "has_been_emailed": false,
                "locale": "zh_Hans_HK",
                "user_interactions": [
                    {
                      "screen_id": "1-howitworks",
                      "started_at": "2021-10-21T17:28:26+00:00",
                      "ended_at": "2021-10-21T17:28:26+00:00",
                      "has_been_skipped": false,
                    },
                    {
                      "screen_id": "2-termsandconditions",
                      "started_at": "2021-10-22T17:28:26+00:00",
                      "ended_at": "2021-10-22T17:29:26+00:00",
                      "has_been_skipped": false,
                      "retakes": [
                        { "retaked_at": "2021-10-22T17:28:26+00:00" },
                        { "retaked_at": "2021-10-22T17:28:27+00:00" },
                        { "retaked_at": "2021-10-22T17:28:28+00:00" }
                      ]
                    }
                ],
                "health_data": {
                  "user_gender": "male",
                  "user_weight": 0.123456789,
                  "user_height": 0.123456789,
                  "user_sys": 55,
                  "user_dia": 55,
                  "user_pulse": 55,
                  "spo2": 55,
                  "sugar": 55,
                  "fvc1": 0.123456789,
                  "fvc2": 0.123456789,
                  "fev1": 0.123456789,
                  "fev2": 0.123456789,
                  "kiosk_location": "HK Tower 1",
                  "invoice_number": "1234fGGH-HTYHGD",
                  "report_fee": 55,
                  "report_date": "2021-10-21T17:28:26+00:00",
                  "feedback": "happy",
                  "age": 55,
                  "status": true,
                  "insurance": false,
                  "bmc": 0.123456789,
                  "bmi": 0.123456789,
                  "mineral": 0.123456789,
                  "bfm": 0.123456789,
                  "bcm": 0.123456789,
                  "protein": 0.123456789,
                  "smm": 0.123456789,
                  "icw": 0.123456789,
                  "ecw": 0.123456789,
                  "vfa": 0.123456789,
                  "whr": 0.123456789,
                  "pbf": 0.123456789,
                  "bwa": 0.123456789,
                  "bmr": 0.123456789,
                  "dci": 0.123456789,
                  "ideal_weight": 0.123456789,
                  "ideal_pbf": 55,
                  "smm_percentage": 0.123456789,
                  "lbm_percentage": 0.123456789,
                  "ideal_protein": 0.123456789,
                  "ideal_icw": 0.123456789,
                  "ideal_ecw": 0.123456789,
                  "min_smm": 55,
                  "max_smm": 55,
                  "ideal_smm": 55,
                  "pulse_score": 0.123456789,
                  "weightlower": 55,
                  "weighthigher": 55,
                  "icllower": 0.123456789,
                  "iclhigher": 0.123456789,
                  "ecllower": 0.123456789,
                  "eclhigher": 0.123456789,
                  "fatlower": 0.123456789,
                  "fathigher": 0.123456789,
                  "bmclower": 0.123456789,
                  "proteinhigher": 0.123456789,
                  "bmchigher": 0.123456789
                }
              }"#,
            "missing health data - proteinlower",
        ),
    ];

    for (invalid_body, error_message) in faulty_bodies {
        // Act
        let response = app.post_user_session(invalid_body.into()).await;

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn get_user_session_returns_a_list_of_user_sessions() {
    // Arrange
    let app = spawn_app().await;

    // insert 2 user sessions
    let res = app
        .post_user_session(
            r#"{
            "id":"79a7016e-fb57-4cf1-93f2-6eb323e65ae4",
            "has_consented": false,
            "has_accepted_terms_conditions": true,
            "has_been_printed": false,
            "has_been_emailed": false,
            "locale": "zh_Hans_HK"
        }"#
            .into(),
        )
        .await;
    assert_eq!(res.status().as_u16(), 201, "found response {:?}", res.text().await);

    let res = app
        .post_user_session(
            r#"{
            "id":"78a6016e-fb57-4cf1-93f2-6eb323e65ae3",
            "has_consented":true,
            "has_accepted_terms_conditions": false,
            "has_been_printed": false,
            "has_been_emailed": false,
            "locale": "zh_Hans_HK"
          }"#
            .into(),
        )
        .await;
    assert_eq!(201, res.status().as_u16(), "found response {:?}", res.text().await);

    let res = app
        .post_user_session(
            r#"
        {
            "id": "79a5066e-fb57-4cf1-93f2-6eb323e65ae3",
            "has_consented": true,
            "has_accepted_terms_conditions": true,
            "has_been_printed": false,
            "has_been_emailed": false,
            "locale": "zh_Hans_HK",
            "user_interactions": [
                {
                  "screen_id": "1-howitworks",
                  "started_at": "2021-10-21T17:28:26+00:00",
                  "ended_at": "2021-10-21T17:28:26+00:00",
                  "has_been_skipped": false
                },
                {
                  "screen_id": "2-termsandconditions",
                  "started_at": "2021-10-22T17:28:26+00:00",
                  "ended_at": "2021-10-22T17:29:26+00:00",
                  "has_been_skipped": true,
                  "retakes": [
                    { "retaked_at": "2021-10-22T17:28:26+00:00" },
                    { "retaked_at": "2021-10-22T17:28:27+00:00" },
                    { "retaked_at": "2021-10-22T17:28:28+00:00" }
                  ]
                }
            ],
            "health_data": {
              "user_gender": "male",
              "user_weight": 0.123456789,
              "user_height": 0.123456789,
              "user_sys": 55,
              "user_dia": 55,
              "user_pulse": 55,
              "spo2": 55,
              "sugar": 55,
              "fvc1": 0.123456789,
              "fvc2": 0.123456789,
              "fev1": 0.123456789,
              "fev2": 0.123456789,
              "kiosk_location": "HK Tower 1",
              "invoice_number": "1234fGGH-HTYHGD",
              "report_fee": 55,
              "report_date": "2021-10-21T17:28:26+00:00",
              "feedback": "happy",
              "age": 55,
              "status": true,
              "insurance": false,
              "bmc": 0.123456789,
              "bmi": 0.123456789,
              "mineral": 0.123456789,
              "bfm": 0.123456789,
              "bcm": 0.123456789,
              "protein": 0.123456789,
              "smm": 0.123456789,
              "icw": 0.123456789,
              "ecw": 0.123456789,
              "vfa": 0.123456789,
              "whr": 0.123456789,
              "pbf": 0.123456789,
              "bwa": 0.123456789,
              "bmr": 0.123456789,
              "dci": 0.123456789,
              "ideal_weight": 0.123456789,
              "ideal_pbf": 55,
              "smm_percentage": 0.123456789,
              "lbm_percentage": 0.123456789,
              "ideal_protein": 0.123456789,
              "ideal_icw": 0.123456789,
              "ideal_ecw": 0.123456789,
              "min_smm": 55,
              "max_smm": 55,
              "ideal_smm": 55,
              "pulse_score": 0.123456789,
              "weightlower": 55,
              "weighthigher": 55,
              "icllower": 0.123456789,
              "iclhigher": 0.123456789,
              "ecllower": 0.123456789,
              "eclhigher": 0.123456789,
              "fatlower": 0.123456789,
              "fathigher": 0.123456789,
              "bmclower": 0.123456789,
              "proteinlower": 0.123456789,
              "proteinhigher": 0.123456789,
              "bmchigher": 0.123456789
            }
          }"#
            .into(),
        )
        .await;
    assert_eq!(201, res.status().as_u16(), "found response {:?}", res.text().await);

    let response = app.list_user_sessions().await;

    // Assert
    assert_eq!(200, response.status().as_u16(), "found response {:?}", res.text().await);

    // json deser
    let uss = response.json::<Vec<UserSession>>().await.unwrap();

    assert_eq!(uss.len(), 3);
}
