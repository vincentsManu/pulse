use std::collections::HashMap;

use super::Stats;
use sqlx::PgPool;

#[tracing::instrument(name = "get all stats from db", skip(pool))]
pub async fn get_all_stats_per_kiosk_location(
    pool: &PgPool,
) -> eyre::Result<HashMap<String, Stats>> {
    let all_kiosk_locations = get_all_kiosk_locations(pool).await?;
    let mut res = HashMap::new();

    let all_kiosk_locations_stats =
        get_db(pool, &GetFilters { kiosk_locations: all_kiosk_locations.clone() }).await?;
    res.insert(String::from(""), all_kiosk_locations_stats);

    for kl in all_kiosk_locations {
        let kiosk_location_stats =
            get_db(pool, &GetFilters { kiosk_locations: vec![kl.clone()] }).await?;
        res.insert(kl, kiosk_location_stats);
    }

    Ok(res)
}

#[derive(Debug, Clone)]
pub struct GetFilters {
    kiosk_locations: Vec<String>,
}

#[tracing::instrument(name = "get stats from db", skip(pool))]
pub async fn get_db(pool: &PgPool, filters: &GetFilters) -> eyre::Result<Stats> {
    let stats_record = sqlx::query!(r#"
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
                    LEFT JOIN user_session us
                        ON TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD') = TO_CHAR(d AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD')
                    LEFT JOIN health_data hd
                        ON hd.user_session_id = us.id
                        AND hd.kiosk_location = ANY($1)  
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
            "#,
            &filters.kiosk_locations[..],
        ).fetch_one(pool)
            .await?;

    let stats: Stats = serde_json::from_value(stats_record.res.unwrap()).unwrap();

    Ok(stats)
}

#[tracing::instrument(name = "get all kiosk locations from db", skip(pool))]
pub async fn get_all_kiosk_locations(pool: &PgPool) -> eyre::Result<Vec<String>> {
    let kiosk_locations = sqlx::query!(r#"SELECT DISTINCT kiosk_location FROM health_data hd"#,)
        .fetch_all(pool)
        .await?;

    Ok(kiosk_locations.iter().map(|kl| kl.kiosk_location.to_owned()).collect())
}

// filter on
//     start user_session_created_at
//     end user_session_created_at
//     location

// #"
// SELECT TO_JSON(a) FROM (
// -- Traffic
//     SELECT
// -- total
//     (
//         SELECT TO_JSON(p) FROM (
//             SELECT
//                 COUNT(*) AS "total_count"
//             FROM user_session us
//         ) p
//     ) as "total_users",
// -- per hour
//     (
//         SELECT JSON_AGG(p) FROM (
//             SELECT
//                 EXTRACT(HOUR FROM us.created_at AT TIME ZONE 'Asia/Singapore') AS "hour",
//                 COUNT(*) AS "count"
//             FROM user_session us
//             GROUP BY EXTRACT(HOUR FROM us.created_at AT TIME ZONE 'Asia/Singapore')
//         ) p
//     ) as "users_per_hour",
// -- per day
//     (
//         SELECT JSON_AGG(p) FROM (
//             SELECT
//                 TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD') AS "date",
//                 COUNT(*) AS "count"
//             FROM user_session us
//             GROUP BY TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'YYYY-MM-DD')
//         ) p
//     ) as "users_per_day",
// -- per day of the week
//     (
//         SELECT JSON_AGG(p) FROM (
//             SELECT
//                 TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'Day') AS "weekday",
//                 COUNT(*) AS "count"
//             FROM user_session us
//             GROUP BY TO_CHAR(us.created_at AT TIME ZONE 'Asia/Singapore', 'Day')
//         ) p
//     ) as "users_per_weekday",
// -- user interactions
// -- % of user has_consented to share data
//     (
//         SELECT TO_JSON(p) FROM (
//             SELECT
//                 COUNT(*)*100 / SUM(
//                     CASE
//                         WHEN has_consented IS FALSE THEN 0
//                         ELSE 1
//                     END
//                 ) AS "consented_%"
//             FROM user_session us
//         ) p
//     ) as "consented_%",
// -- % of user has_accepted_terms_conditions to share data
//     (
//         SELECT TO_JSON(p) FROM (
//             SELECT
//                 COUNT(*)*100 / SUM(
//                     CASE
//                         WHEN has_accepted_terms_conditions IS FALSE THEN 0
//                         ELSE 1
//                     END
//                 ) AS "agreed_%"
//             FROM user_session us
//         ) p
//     ) as "agreed_%",
// -- median time spent on each screen
//     (
//         SELECT JSON_AGG(p) FROM (
//             SELECT
//                 ui.screen_id,
//                 TRUNC(
//                     PERCENTILE_DISC(0.5)
//                     WITHIN GROUP (
//                         ORDER BY EXTRACT(EPOCH FROM (ui.ended_at - ui.started_at))
//                     )
//                 ) AS "median_duration"
//             FROM user_interaction ui
//             GROUP BY ui.screen_id
//         ) p
//     ) as "median_duration",
// -- median retake for screens
//     (
//         SELECT JSON_AGG(p) FROM (
//             SELECT
//                 screen_id,
//                 TRUNC(
//                     PERCENTILE_DISC(0.5)
//                     WITHIN GROUP (ORDER BY retakes)
//                 ) as "median_retakes"
//             FROM (
//                 SELECT
//                     ui.screen_id,
//                     ui.id,
//                     SUM(
//                         CASE
//                             WHEN retaked_at IS NULL THEN 0
//                             ELSE 1
//                         END
//                     ) AS "retakes"
//                 FROM user_interaction ui
//                 LEFT JOIN retake r ON r.user_interaction_id = ui.id
//                 GROUP BY ui.screen_id, ui.id
//             ) r
//             GROUP BY screen_id
//         ) p
//     ) as "median_retakes",
//     -- feedback distribution
//     (
//         SELECT JSON_AGG(p) FROM (
//             SELECT
//                 hd.feedback,
//                 COUNT(*) AS "count"
//             FROM health_data hd
//             GROUP BY hd.feedback
//         ) p
//     ) as "feedback_count"
// ) a
// "#
