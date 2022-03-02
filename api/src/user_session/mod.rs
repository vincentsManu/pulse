pub mod crud;
pub mod errors;
pub mod health_data;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    id: uuid::Uuid,
    pub has_consented: bool,
    pub has_accepted_terms_conditions: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_interactions: Option<Vec<UserInteraction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_data: Option<HealthData>,
    has_been_printed: bool,
    has_been_emailed: bool,
    locale: Locale,
    #[serde(default = "Utc::now")]
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "locale")]
pub enum Locale {
    #[serde(rename = "en_US")]
    #[sqlx(rename = "en_US")]
    EnUS,
    #[serde(rename = "zh_Hans_HK")]
    #[sqlx(rename = "zh_Hans_HK")]
    ZhHansHK,
    #[serde(rename = "zh_Hant_HK")]
    #[sqlx(rename = "zh_Hant_HK")]
    ZhHantHK,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "screen")]
pub enum Screen {
    #[serde(rename = "0-welcome")]
    #[sqlx(rename = "0-welcome")]
    Welcome,
    #[serde(rename = "1-importantnotice")]
    #[sqlx(rename = "1-importantnotice")]
    ImportantNotice,
    #[serde(rename = "1-howitworks")]
    #[sqlx(rename = "1-howitworks")]
    HowItWorks,
    #[serde(rename = "2-termsandconditions")]
    #[sqlx(rename = "2-termsandconditions")]
    TermsAndConditions,
    #[serde(rename = "3-privacy")]
    #[sqlx(rename = "3-privacy")]
    Privacy,
    #[serde(rename = "4-enter-name")]
    #[sqlx(rename = "4-enter-name")]
    EnterName,
    #[serde(rename = "101-step2bca")]
    #[sqlx(rename = "101-step2bca")]
    Step2BCA,
    #[serde(rename = "102-displaybca")]
    #[sqlx(rename = "102-displaybca")]
    DisplayBCA,
    #[serde(rename = "103-afterbcacompleted")]
    #[sqlx(rename = "103-afterbcacompleted")]
    AfterBCACompleted,
    #[serde(rename = "111-step2bp")]
    #[sqlx(rename = "111-step2bp")]
    Step2BP,
    #[serde(rename = "112-displaybp")]
    #[sqlx(rename = "112-displaybp")]
    DisplayBP,
    #[serde(rename = "121-step2spo2")]
    #[sqlx(rename = "121-step2spo2")]
    Step2SPO2,
    #[serde(rename = "122-displayspo2")]
    #[sqlx(rename = "122-displayspo2")]
    DisplaySPO2,
    #[serde(rename = "0-cannotgeneratereportlocally")]
    #[sqlx(rename = "0-cannotgeneratereportlocally")]
    CannotGenerateReportLocally,
    #[serde(rename = "0-finish")]
    #[sqlx(rename = "0-finish")]
    Finish,
    #[serde(rename = "0-generatereport")]
    #[sqlx(rename = "0-generatereport")]
    GenerateReport,
    #[serde(rename = "0-systemreset")]
    #[sqlx(rename = "0-systemreset")]
    SystemReset,
    #[serde(rename = "0-systemtimeout")]
    #[sqlx(rename = "0-systemtimeout")]
    SystemTimeout,
    #[serde(rename = "5-enter-dob")]
    #[sqlx(rename = "5-enter-dob")]
    Dob,
    #[serde(rename = "6-gender")]
    #[sqlx(rename = "6-gender")]
    Gender,
    #[serde(rename = "7-exercise")]
    #[sqlx(rename = "7-exercise")]
    Exercise,
    #[serde(rename = "8-heightweightstart")]
    #[sqlx(rename = "8-heightweightstart")]
    HeightWeightStart,
    #[serde(rename = "9-displayheightweight")]
    #[sqlx(rename = "9-displayheightweight")]
    DisplayHeightWeight,
    #[serde(rename = "10-startbca")]
    #[sqlx(rename = "10-startbca")]
    StartBCA,
    #[serde(rename = "11-startbp")]
    #[sqlx(rename = "11-startbp")]
    StartBP,
    #[serde(rename = "12-startspo2")]
    #[sqlx(rename = "12-startspo2")]
    StartsPO2,
    #[serde(rename = "13-displaypulsescore")]
    #[sqlx(rename = "13-displaypulsescore")]
    DisplayPulseScore,
    #[serde(rename = "14-provide-email-address")]
    #[sqlx(rename = "14-provide-email-address")]
    Address,
    #[serde(rename = "15-report-sent-in-email")]
    #[sqlx(rename = "15-report-sent-in-email")]
    Email,
    #[serde(rename = "16-report-is-printing")]
    #[sqlx(rename = "16-report-is-printing")]
    Printing,
    #[serde(rename = "80-beforeheightweight")]
    #[sqlx(rename = "80-beforeheightweight")]
    BeforeHeightWeight,
    #[serde(rename = "91-editheightweight")]
    #[sqlx(rename = "91-editheightweight")]
    EditHeightWeight,
    #[serde(rename = "14-finishandthankyou")]
    #[sqlx(rename = "14-finishandthankyou")]
    FinishAndThankYou,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInteraction {
    screen_id: Screen,
    started_at: DateTime<Utc>,
    ended_at: DateTime<Utc>,
    has_been_skipped: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    retaked_at: Option<Vec<DateTime<Utc>>>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "feedback", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Feedback {
    Happy,
    Unhappy,
    Satisfactory,
    VeryHappy,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Female,
    Male,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthData {
    user_gender: Gender,
    user_weight: f64,
    user_height: f64,
    user_sys: i32,
    user_dia: i32,
    user_pulse: i32,
    spo2: i32,
    sugar: i32,
    fvc1: f64,
    fvc2: f64,
    fev1: f64,
    fev2: f64,
    kiosk_location: String,
    invoice_number: String,
    report_fee: i32,
    report_date: DateTime<Utc>,
    feedback: Feedback,
    age: i32,
    status: bool,
    insurance: bool,
    bmc: f64,
    bmi: f64,
    mineral: f64,
    bfm: f64,
    bcm: f64,
    protein: f64,
    smm: f64,
    icw: f64,
    ecw: f64,
    vfa: f64,
    whr: f64,
    pbf: f64,
    bwa: f64,
    bmr: f64,
    dci: f64,
    ideal_weight: f64,
    ideal_pbf: i32,
    smm_percentage: f64,
    lbm_percentage: f64,
    ideal_protein: f64,
    ideal_icw: f64,
    ideal_ecw: f64,
    min_smm: i32,
    max_smm: i32,
    ideal_smm: i32,
    pulse_score: f64,
    weightlower: i32,
    weighthigher: i32,
    icllower: f64,
    iclhigher: f64,
    ecllower: f64,
    eclhigher: f64,
    fatlower: f64,
    fathigher: f64,
    bmclower: f64,
    proteinlower: f64,
    proteinhigher: f64,
    bmchigher: f64,
}
