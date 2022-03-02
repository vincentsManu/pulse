use super::{errors::UserSessionError, Feedback, Gender, HealthData};
use sqlx::PgPool;

#[tracing::instrument(name = "get health data from db", skip(pool))]
pub async fn get_health_data(pool: &PgPool) -> Result<Vec<HealthData>, UserSessionError> {
    let hd: Vec<HealthData> = sqlx::query_as!(
        HealthData,
        r#"
            SELECT
                user_gender as "user_gender: Gender",
                user_weight,
                user_height,
                user_sys,
                user_dia,
                user_pulse,
                spo2,
                sugar,
                fvc1,
                fvc2,
                fev1,
                fev2,
                kiosk_location,
                invoice_number,
                report_fee,
                report_date,
                feedback as "feedback: Feedback",
                age,
                hd.health_data_status as status,
                insurance,
                bmc,
                bmi,
                mineral,
                bfm,
                bcm,
                protein,
                smm,
                icw,
                ecw,
                vfa,
                whr,
                pbf,
                bwa,
                bmr,
                dci,
                ideal_weight,
                ideal_pbf,
                smm_percentage,
                lbm_percentage,
                ideal_protein,
                ideal_icw,
                ideal_ecw,
                min_smm,
                max_smm,
                ideal_smm,
                pulse_score,
                weightlower,
                weighthigher,
                icllower,
                iclhigher,
                ecllower,
                eclhigher,
                fatlower,
                fathigher,
                bmclower,
                proteinlower,
                proteinhigher,
                bmchigher
            FROM health_data hd
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(hd)
}
