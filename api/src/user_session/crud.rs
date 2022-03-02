use super::{errors::UserSessionError, UserSession};
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[tracing::instrument(name = "save new user session in the database", skip(pool, user_session))]
pub async fn insert_user_session(
    pool: &PgPool,
    user_session: &UserSession,
) -> Result<(), UserSessionError> {
    let mut transaction = pool.begin().await?;

    sqlx::query!(
        r#"
    INSERT INTO user_session (
        id,
        has_consented,
        has_accepted_terms_conditions,
        has_been_printed,
        has_been_emailed,
        locale,
        created_at
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        user_session.id,
        user_session.has_consented,
        user_session.has_accepted_terms_conditions,
        user_session.has_been_printed,
        user_session.has_been_emailed,
        user_session.locale as _,
        user_session.created_at,
    )
    .execute(&mut transaction)
    .await?;

    // user interactions
    if let Some(uis) = &user_session.user_interactions {
        let mut retakes: Vec<(uuid::Uuid, DateTime<Utc>)> = Vec::new();
        for ui in uis {
            let ui_id = uuid::Uuid::new_v4();
            sqlx::query!(
                r#"
        INSERT INTO user_interaction (
            id,
            user_session_id,
            screen_id,
            started_at,
            ended_at,
            has_been_skipped
        )
        VALUES ($1, $2, $3, $4, $5, $6)
            "#,
                ui_id,
                user_session.id,
                ui.screen_id as _,
                ui.started_at,
                ui.ended_at,
                ui.has_been_skipped,
            )
            .execute(&mut transaction)
            .await?;

            if let Some(rtas) = &ui.retaked_at {
                rtas.iter().for_each(|rt| {
                    retakes.push((ui_id, *rt));
                })
            }
        }

        for (user_interaction_id, retaked_at) in retakes {
            sqlx::query!(
                r#"
        INSERT INTO retake (
            user_interaction_id,
            retaked_at
        )
        VALUES ($1, $2)
            "#,
                user_interaction_id,
                retaked_at,
            )
            .execute(&mut transaction)
            .await?;
        }
    }

    // health data
    if let Some(hd) = &user_session.health_data {
        sqlx::query!(
            r#"
    INSERT INTO health_data (
        user_session_id,
        user_gender,
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
        feedback,
        age,
        health_data_status,
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
    )
    VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        $12,
        $13,
        $14,
        $15,
        $16,
        $17,
        $18,
        $19,
        $20,
        $21,
        $22,
        $23,
        $24,
        $25,
        $26,
        $27,
        $28,
        $29,
        $30,
        $31,
        $32,
        $33,
        $34,
        $35,
        $36,
        $37,
        $38,
        $39,
        $40,
        $41,
        $42,
        $43,
        $44,
        $45,
        $46,
        $47,
        $48,
        $49,
        $50,
        $51,
        $52,
        $53,
        $54,
        $55,
        $56,
        $57,
        $58,
        $59
    )
    "#,
            user_session.id,
            hd.user_gender as _,
            hd.user_weight,
            hd.user_height,
            hd.user_sys,
            hd.user_dia,
            hd.user_pulse,
            hd.spo2,
            hd.sugar,
            hd.fvc1,
            hd.fvc2,
            hd.fev1,
            hd.fev2,
            hd.kiosk_location,
            hd.invoice_number,
            hd.report_fee,
            hd.report_date,
            hd.feedback as _,
            hd.age,
            hd.status,
            hd.insurance,
            hd.bmc,
            hd.bmi,
            hd.mineral,
            hd.bfm,
            hd.bcm,
            hd.protein,
            hd.smm,
            hd.icw,
            hd.ecw,
            hd.vfa,
            hd.whr,
            hd.pbf,
            hd.bwa,
            hd.bmr,
            hd.dci,
            hd.ideal_weight,
            hd.ideal_pbf,
            hd.smm_percentage,
            hd.lbm_percentage,
            hd.ideal_protein,
            hd.ideal_icw,
            hd.ideal_ecw,
            hd.min_smm,
            hd.max_smm,
            hd.ideal_smm,
            hd.pulse_score,
            hd.weightlower,
            hd.weighthigher,
            hd.icllower,
            hd.iclhigher,
            hd.ecllower,
            hd.eclhigher,
            hd.fatlower,
            hd.fathigher,
            hd.bmclower,
            hd.proteinlower,
            hd.proteinhigher,
            hd.bmchigher,
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(())
}

#[tracing::instrument(name = "get all user sessions from the database", skip(pool))]
pub async fn get_all_user_session(pool: &PgPool) -> Result<Vec<UserSession>, UserSessionError> {
    let rows = sqlx::query!(
        r#"
    SELECT
        row_to_json(usj)  as "user_session: String"
    FROM
        (
            SELECT
                us.id,
                us.has_consented,
                us.has_accepted_terms_conditions,
                us.has_been_printed,
                us.has_been_emailed,
                us.locale,
                us.created_at,
                (
                    SELECT
                        row_to_json(hdj) as "health_data"
                    FROM
                        (
                            SELECT
                                hd.user_session_id,
                                hd.user_gender,
                                hd.user_weight,
                                hd.user_height,
                                hd.user_sys,
                                hd.user_dia,
                                hd.user_pulse,
                                hd.spo2,
                                hd.sugar,
                                hd.fvc1,
                                hd.fvc2,
                                hd.fev1,
                                hd.fev2,
                                hd.kiosk_location,
                                hd.invoice_number,
                                hd.report_fee,
                                hd.report_date,
                                hd.feedback,
                                hd.age,
                                hd.health_data_status as status,
                                hd.insurance,
                                hd.bmc,
                                hd.bmi,
                                hd.mineral,
                                hd.bfm,
                                hd.bcm,
                                hd.protein,
                                hd.smm,
                                hd.icw,
                                hd.ecw,
                                hd.vfa,
                                hd.whr,
                                hd.pbf,
                                hd.bwa,
                                hd.bmr,
                                hd.dci,
                                hd.ideal_weight,
                                hd.ideal_pbf,
                                hd.smm_percentage,
                                hd.lbm_percentage,
                                hd.ideal_protein,
                                hd.ideal_icw,
                                hd.ideal_ecw,
                                hd.min_smm,
                                hd.max_smm,
                                hd.ideal_smm,
                                hd.pulse_score,
                                hd.weightlower,
                                hd.weighthigher,
                                hd.icllower,
                                hd.iclhigher,
                                hd.ecllower,
                                hd.eclhigher,
                                hd.fatlower,
                                hd.fathigher,
                                hd.bmclower,
                                hd.proteinlower,
                                hd.proteinhigher,
                                hd.bmchigher
                            FROM
                                health_data hd
                            WHERE
                                hd.user_session_id = us.id
                        ) hdj
                ) as "health_data",
                (
                    SELECT
                        array_to_json(array_agg(row_to_json(uij))) as "user_interactions"
                    FROM
                        (
                            SELECT
                                ui.screen_id,
                                ui.started_at,
                                ui.ended_at,
                                (
                                    SELECT
                                        array_to_json(array_agg(rt.retaked_at))
                                    FROM
                                    retake rt
                                WHERE
                                    rt.user_interaction_id = ui.id
                                ) as "retaked_at",
                                ui.has_been_skipped
                            FROM
                                user_interaction ui
                            WHERE ui.user_session_id = us.id
                        ) uij
                )  as "user_interactions"
            FROM
                user_session us
        ) usj
    "#,
    )
    .fetch_all(pool)
    .await?;

    let user_sessions: Vec<UserSession> = rows
        .into_iter()
        .filter_map(|r| r.user_session)
        .map(|jus| {
            let u: UserSession = serde_json::from_str(&jus).unwrap();
            u
        })
        .collect();

    Ok(user_sessions)
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
