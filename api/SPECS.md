# Kiosk payload

## Sample POST payload from the kiosk (JSON)

```json
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
          }
```

## Data migrations (SQL)

```sql

CREATE TABLE IF NOT EXISTS user_session
(
    id              uuid PRIMARY KEY,
    has_consented   BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TYPE GENDER AS ENUM ('male', 'female');

CREATE TYPE FEEDBACK AS ENUM ('happy');

CREATE TABLE IF NOT EXISTS health_data
(
    user_session_id uuid,
    user_gender GENDER NOT NULL,
    user_weight DOUBLE PRECISION NOT NULL DEFAULT 0,
    user_height DOUBLE PRECISION NOT NULL DEFAULT 0,
    user_sys INTEGER NOT NULL DEFAULT 0,
    user_dia INTEGER NOT NULL DEFAULT 0,
    user_pulse INTEGER NOT NULL DEFAULT 0,
    spo2 INTEGER NOT NULL DEFAULT 0,
    sugar INTEGER NOT NULL DEFAULT 0,
    fvc1 DOUBLE PRECISION NOT NULL DEFAULT 0,
    fvc2 DOUBLE PRECISION NOT NULL DEFAULT 0,
    fev1 DOUBLE PRECISION NOT NULL DEFAULT 0,
    fev2 DOUBLE PRECISION NOT NULL DEFAULT 0,
    kiosk_location TEXT NOT NULL DEFAULT '',
    invoice_number TEXT NOT NULL DEFAULT '',
    report_fee INTEGER NOT NULL DEFAULT 0,
    report_date DATE NOT NULL DEFAULT CURRENT_DATE,
    feedback FEEDBACK NOT NULL,
    age INTEGER NOT NULL DEFAULT 0,
    health_data_status BOOLEAN NOT NULL DEFAULT FALSE,
    insurance BOOLEAN NOT NULL DEFAULT FALSE,
    bmc DOUBLE PRECISION NOT NULL DEFAULT 0,
    bmi DOUBLE PRECISION NOT NULL DEFAULT 0,
    mineral DOUBLE PRECISION NOT NULL DEFAULT 0,
    bfm DOUBLE PRECISION NOT NULL DEFAULT 0,
    bcm DOUBLE PRECISION NOT NULL DEFAULT 0,
    protein DOUBLE PRECISION NOT NULL DEFAULT 0,
    smm DOUBLE PRECISION NOT NULL DEFAULT 0,
    icw DOUBLE PRECISION NOT NULL DEFAULT 0,
    ecw DOUBLE PRECISION NOT NULL DEFAULT 0,
    vfa DOUBLE PRECISION NOT NULL DEFAULT 0,
    whr DOUBLE PRECISION NOT NULL DEFAULT 0,
    pbf DOUBLE PRECISION NOT NULL DEFAULT 0,
    bwa DOUBLE PRECISION NOT NULL DEFAULT 0,
    bmr DOUBLE PRECISION NOT NULL DEFAULT 0,
    dci DOUBLE PRECISION NOT NULL DEFAULT 0,
    ideal_weight DOUBLE PRECISION NOT NULL DEFAULT 0,
    ideal_pbf INTEGER NOT NULL DEFAULT 0,
    smm_percentage DOUBLE PRECISION NOT NULL DEFAULT 0,
    lbm_percentage DOUBLE PRECISION NOT NULL DEFAULT 0,
    ideal_protein DOUBLE PRECISION NOT NULL DEFAULT 0,
    ideal_icw DOUBLE PRECISION NOT NULL DEFAULT 0,
    ideal_ecw DOUBLE PRECISION NOT NULL DEFAULT 0,
    min_smm INTEGER NOT NULL DEFAULT 0,
    max_smm INTEGER NOT NULL DEFAULT 0,
    ideal_smm INTEGER NOT NULL DEFAULT 0,
    pulse_score DOUBLE PRECISION NOT NULL DEFAULT 0,
    weightlower INTEGER NOT NULL DEFAULT 0,
    weighthigher INTEGER NOT NULL DEFAULT 0,
    icllower DOUBLE PRECISION NOT NULL DEFAULT 0,
    iclhigher DOUBLE PRECISION NOT NULL DEFAULT 0,
    ecllower DOUBLE PRECISION NOT NULL DEFAULT 0,
    eclhigher DOUBLE PRECISION NOT NULL DEFAULT 0,
    fatlower DOUBLE PRECISION NOT NULL DEFAULT 0,
    fathigher DOUBLE PRECISION NOT NULL DEFAULT 0,
    bmclower DOUBLE PRECISION NOT NULL DEFAULT 0,
    proteinlower DOUBLE PRECISION NOT NULL DEFAULT 0,
    proteinhigher DOUBLE PRECISION NOT NULL DEFAULT 0,
    bmchigher DOUBLE PRECISION NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT health_data_pk PRIMARY KEY(user_session_id),
    CONSTRAINT health_data_us_fk FOREIGN KEY (user_session_id) REFERENCES user_session(id)
);
ALTER TYPE FEEDBACK ADD VALUE 'unhappy';
ALTER TYPE FEEDBACK ADD VALUE 'satisfactory';
ALTER TYPE FEEDBACK ADD VALUE 'veryhappy';

ALTER TABLE health_data DROP COLUMN created_at;

ALTER TABLE user_session ADD COLUMN has_accepted_terms_conditions BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS user_interaction
(
    id                  uuid PRIMARY KEY,
    user_session_id     uuid NOT NULL,
    screen_id           INTEGER NOT NULL,
    started_at          TIMESTAMPTZ NOT NULL,
    ended_at            TIMESTAMPTZ NOT NULL,
    CONSTRAINT user_interaction_us_fk FOREIGN KEY (user_session_id) REFERENCES user_session(id)
);

CREATE TABLE IF NOT EXISTS retake
(
    user_interaction_id uuid NOT NULL,
    retaked_at          TIMESTAMPTZ NOT NULL,
    CONSTRAINT retake_ui_fk FOREIGN KEY (user_interaction_id) REFERENCES user_interaction(id)
);
CREATE OR REPLACE FUNCTION notify_user_session_insert ()
    RETURNS TRIGGER
    AS $$
DECLARE
    ROW RECORD;
    output text;
BEGIN
    PERFORM
        pg_notify('user_session_inserted', CAST(NEW.id AS text));
    RETURN NULL;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER trigger_user_session_insert
    AFTER INSERT OR UPDATE OR DELETE ON user_session
    FOR EACH ROW
    EXECUTE PROCEDURE notify_user_session_insert ();


ALTER TABLE user_interaction
    ADD COLUMN has_been_skipped BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE user_session
    ADD COLUMN has_been_printed BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE user_session
    ADD COLUMN has_been_emailed BOOLEAN NOT NULL DEFAULT FALSE;


ALTER TABLE health_data
    DROP COLUMN report_date;

ALTER TABLE health_data
    ADD COLUMN report_date TIMESTAMPTZ NOT NULL DEFAULT NOW();


CREATE TYPE SCREEN AS ENUM (
    'welcome',
    'termsandconditions',
    'privacy',
    'entername'
);

ALTER TABLE user_interaction
    DROP COLUMN screen_id;

ALTER TABLE user_interaction
    ADD COLUMN screen_id SCREEN NOT NULL DEFAULT 'welcome';


ALTER TYPE SCREEN RENAME VALUE 'welcome' TO '1-howitworks';

ALTER TYPE SCREEN RENAME VALUE 'termsandconditions' TO '2-termsandconditions';

ALTER TYPE SCREEN RENAME VALUE 'privacy' TO '3-privacy';

ALTER TYPE SCREEN RENAME VALUE 'entername' TO '4-enter-name';

ALTER TYPE SCREEN
    ADD VALUE '101-step2bca';

ALTER TYPE SCREEN
    ADD VALUE '102-displaybca';

ALTER TYPE SCREEN
    ADD VALUE '103-afterbcacompleted';

ALTER TYPE SCREEN
    ADD VALUE '111-step2bp';

ALTER TYPE SCREEN
    ADD VALUE '112-displaybp';

ALTER TYPE SCREEN
    ADD VALUE '121-step2spo2';

ALTER TYPE SCREEN
    ADD VALUE '122-displayspo2';

ALTER TYPE SCREEN
    ADD VALUE '0-cannotgeneratereportlocally';

ALTER TYPE SCREEN
    ADD VALUE '0-finish';

ALTER TYPE SCREEN
    ADD VALUE '0-generatereport';

ALTER TYPE SCREEN
    ADD VALUE '0-systemreset';

ALTER TYPE SCREEN
    ADD VALUE '0-systemtimeout';

ALTER TYPE SCREEN
    ADD VALUE '5-enter-dob';

ALTER TYPE SCREEN
    ADD VALUE '6-gender';

ALTER TYPE SCREEN
    ADD VALUE '7-exercise';

ALTER TYPE SCREEN
    ADD VALUE '8-heightweightstart';

ALTER TYPE SCREEN
    ADD VALUE '9-displayheightweight';

ALTER TYPE SCREEN
    ADD VALUE '10-startbca';

ALTER TYPE SCREEN
    ADD VALUE '11-startbp';

ALTER TYPE SCREEN
    ADD VALUE '12-startspo2';

ALTER TYPE SCREEN
    ADD VALUE '13-displaypulsescore';

ALTER TYPE SCREEN
    ADD VALUE '14-provide-email-address';

ALTER TYPE SCREEN
    ADD VALUE '15-report-sent-in-email';

ALTER TYPE SCREEN
    ADD VALUE '16-report-is-printing';

ALTER TYPE SCREEN
    ADD VALUE '80-beforeheightweight';

ALTER TYPE SCREEN
    ADD VALUE '91-editheightweight';


CREATE TYPE LOCALE AS ENUM (
    'en_US',
    'zh_Hans_HK',
    'zh_Hant_HK'
);

ALTER TABLE user_session
    ADD COLUMN locale LOCALE NOT NULL DEFAULT 'en_US';


ALTER TYPE SCREEN
    ADD VALUE '0-welcome';

ALTER TYPE SCREEN
    ADD VALUE '14-finishandthankyou';


ALTER TYPE SCREEN
    ADD VALUE '1-importantnotice';


CREATE INDEX health_data_kiosk_location_index ON health_data (kiosk_location);
```

## Data model (in rust)

```rust
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
```

# Stats query

$1 argument being the kiosk_location

```sql
    SELECT TO_JSON(a) as res FROM (
        -- Traffic
            SELECT
        -- total
            (
                SELECT * FROM (
                    SELECT
                        COUNT(*) AS "total_count"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)
                ) p
            ) as "total_users",     
        -- median test duration
            (
                SELECT
                    COALESCE(TRUNC(
                        PERCENTILE_DISC(0.5)
                        WITHIN GROUP (ORDER BY duration_s)
                    ), 0) AS "median_duration_s"
                FROM (
                    SELECT
                        EXTRACT(EPOCH FROM (MAX(ui.ended_at) - MIN(ui.started_at))) as "duration_s"
                    FROM user_interaction ui
                    INNER JOIN health_data hd
                        ON hd.user_session_id = ui.user_session_id
                        AND hd.kiosk_location = ANY($1)                    
                    AND EXISTS(
                        SELECT 1
                        FROM user_interaction uix
                        WHERE ui.user_session_id = uix.user_session_id
                        AND uix.screen_id = '0-generatereport'
                    )
                    group by ui.user_session_id
                ) p
            ) as "median_duration_s",
        -- % completion
            (
                SELECT
                    COALESCE(CASE 
                        WHEN entername.count = 0 THEN 0
                        ELSE genreport.count*100 / entername.count
                    END, 0) AS "completion_percent"
                FROM
                (
                    SELECT
                        COUNT(*) AS count
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)                    
                ) entername,
                (
                    SELECT
                        COUNT(*) AS count
                    FROM user_interaction ui
                    INNER JOIN health_data hd
                        ON hd.user_session_id = ui.user_session_id
                        AND hd.kiosk_location = ANY($1)                      
                    AND screen_id = ('0-generatereport')
                ) genreport
            ) as "completion_percent",
        -- % of user has_consented to share data
            (
                SELECT * FROM (
                    SELECT
                        COALESCE(100*SUM(
                            CASE
                                WHEN has_consented IS FALSE THEN 0
                                ELSE 1
                            END
                        )/COUNT(*), 0) AS "consented_percent"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)                      
                ) p
            ) as "consented_percent",
        -- % of user has_accepted_terms_conditions to share data
            (
                SELECT * FROM (
                    SELECT
                        COALESCE(100*SUM(
                            CASE
                                WHEN has_accepted_terms_conditions IS FALSE THEN 0
                                ELSE 1
                            END
                        )/COUNT(*), 0) AS "agreed_percent"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)                      
                ) p
            ) as "agreed_percent", 
        -- % of user has_been_printed
            (
                SELECT * FROM (
                    SELECT
                        COALESCE(100*SUM(
                            CASE
                                WHEN has_been_printed IS FALSE THEN 0
                                ELSE 1
                            END
                        )/COUNT(*), 0) AS "has_been_printed_percent"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)                      
                ) p
            ) as "has_been_printed_percent", 
        -- % of user has_been_emailed
            (
                SELECT * FROM (
                    SELECT
                        COALESCE(100*SUM(
                            CASE
                                WHEN has_been_emailed IS FALSE THEN 0
                                ELSE 1
                            END
                        )/COUNT(*), 0) AS "has_been_emailed_percent"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)                      
                ) p
            ) as "has_been_emailed_percent",  
        -- feedback distribution
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        CASE
                            WHEN hd.feedback = 'veryhappy' THEN 'great'
                            WHEN hd.feedback = 'happy' THEN 'good'
                            WHEN hd.feedback = 'satisfactory' THEN 'ok'
                            WHEN hd.feedback = 'unhappy' THEN 'poor'
                        END AS "feedback",  
                        COUNT(*) AS "count"
                    FROM health_data hd
                    WHERE hd.kiosk_location = ANY($1)
                    GROUP BY hd.feedback
                ) p
            ) as "users_per_feedback",      
        -- locale distribution
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        us.locale,
                        COUNT(*) AS "count"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)                      
                    GROUP BY us.locale
                ) p
            ) as "users_per_locale",   
        -- gender distribution
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        hd.user_gender as gender,
                        COUNT(*) AS "count"
                    FROM health_data hd
                    WHERE hd.kiosk_location = ANY($1)    
                    GROUP BY hd.user_gender
                ) p
            ) as "users_per_gender",          
        -- per age group
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (        
                    SELECT
                        ac.age_group,
                        ac.count
                    FROM (
                        SELECT
                            CASE
                                WHEN hd.age < 25 THEN '< 25'
                                WHEN hd.age >= 25 AND hd.age < 35 THEN '25-35'
                                WHEN hd.age >= 35 AND hd.age < 45 THEN '35-45'
                                WHEN hd.age >= 45 AND hd.age < 55 THEN '45-55'				
                                WHEN hd.age >= 55 AND hd.age <= 65 THEN '55-65'						
                                WHEN hd.age > 65 THEN '> 65'								
                            END AS age_group,
                            count(*) AS count
                        FROM health_data hd
                        WHERE hd.kiosk_location = ANY($1)    
                        GROUP BY age_group
                    ) ac
                    ORDER BY ARRAY_POSITION(ARRAY['< 25','25-35','35-45','45-55','55-65','> 65'], ac.age_group)
                ) p
            ) as "users_per_age_group",       
        -- bmi distribution   
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        ug.weight_status,
                        ug.count
                    FROM (
                        SELECT
                            CASE
                                WHEN hd.bmi < 18.5 THEN 'underweight'
                                WHEN hd.bmi >= 18.5 AND hd.bmi < 25 THEN 'healthy'                		
                                WHEN hd.bmi >= 25 AND hd.bmi < 30 THEN 'overweight'                		
                                WHEN hd.bmi > 30 THEN 'obese'
                            END AS weight_status,                		                		
                            COUNT(*) AS "count"
                        FROM health_data hd
                        WHERE hd.kiosk_location = ANY($1)    
                        GROUP BY weight_status
                    ) ug
                    ORDER BY ARRAY_POSITION(ARRAY['underweight','healthy','overweight','obese'], ug.weight_status)
                ) p
            ) as "users_per_weight_status",                                                      
        -- per hour
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        EXTRACT(HOUR FROM us.created_at AT TIME ZONE 'Asia/Singapore') AS "hour",
                        COUNT(*) AS "count"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)  
                    GROUP BY EXTRACT(HOUR FROM us.created_at AT TIME ZONE 'Asia/Singapore')
                    ORDER BY EXTRACT(HOUR FROM us.created_at AT TIME ZONE 'Asia/Singapore')
                ) p
            ) as "users_per_hour",
        -- per day
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        TO_CHAR(d AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD') as "date",
                        COUNT(us.id) AS "count"
                    FROM generate_series(
                        date('2021-12-01'), 
                        CURRENT_DATE, 
                        '1 day'
                    ) d
                    LEFT JOIN (user_session us INNER JOIN health_data hd ON hd.user_session_id = us.id AND hd.kiosk_location = ANY($1))
                        ON TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD') = TO_CHAR(d AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD')
                    GROUP BY date
                    ORDER BY date
                ) p
            ) as "users_per_day",
        -- per day of the week
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        TRIM(TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'day')) AS "weekday",
                        COUNT(*) AS "count"
                    FROM user_session us
                    INNER JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)  
                    GROUP BY extract(isodow from us.created_at AT TIME ZONE 'Asia/Singapore'), TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'day')
                    ORDER BY extract(isodow from us.created_at AT TIME ZONE 'Asia/Singapore')
                ) p
            ) as "users_per_weekday",
        -- user interactions
        -- median time spent on each screen
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        ui.screen_id,
                        TRUNC(
                            PERCENTILE_DISC(0.5)
                            WITHIN GROUP (
                                ORDER BY EXTRACT(EPOCH FROM (ui.ended_at - ui.started_at))
                            )
                        ) AS "median_duration_s"
                    FROM user_interaction ui
                    INNER JOIN health_data hd
                        ON hd.user_session_id = ui.user_session_id
                        AND hd.kiosk_location = ANY($1)  
                    GROUP BY ui.screen_id
                    ORDER BY ui.screen_id
                ) p
            ) as "median_duration_s_per_screen",
        -- median retake for screens
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        screen_id,
                        TRUNC(
                            PERCENTILE_DISC(0.99)
                            WITHIN GROUP (ORDER BY retakes)
                        ) as "median_retakes"
                    FROM (
                        SELECT
                            ui.screen_id,
                            ui.id,
                            SUM(
                                CASE
                                    WHEN retaked_at IS NULL THEN 0
                                    ELSE 1
                                END
                            ) AS "retakes"
                        FROM user_interaction ui
                        INNER JOIN health_data hd
                            ON hd.user_session_id = ui.user_session_id
                            AND hd.kiosk_location = ANY($1)  
                        LEFT JOIN retake r ON r.user_interaction_id = ui.id
                        GROUP BY ui.screen_id, ui.id
                    ) r
                    GROUP BY screen_id
                    ORDER BY screen_id
                ) p
            ) as "median_retakes_per_screen",
        -- skipped % per screen retake for screens
            (
                SELECT COALESCE(JSON_AGG(p), '[]') FROM (
                    SELECT
                        screen_id,
                        SUM(
                            CASE
                                WHEN has_been_skipped IS TRUE THEN 1
                                ELSE 0
                            END
                        )*100/COUNT(*) AS "has_been_skipped_percent"
                    FROM user_interaction ui
                    INNER JOIN health_data hd
                        ON hd.user_session_id = ui.user_session_id
                        AND hd.kiosk_location = ANY($1)  
                    GROUP BY screen_id
                    ORDER BY screen_id
                ) p
            ) as "has_been_skipped_percent_per_screen"
        ) a
```