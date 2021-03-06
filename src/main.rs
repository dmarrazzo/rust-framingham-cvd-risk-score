use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use warp::Filter;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    #[serde(alias = "Age")]
    age: usize,
    #[serde(alias = "Sex")]
    sex: String,
    #[serde(alias = "Smoker?")]
    smoker: bool,
    #[serde(alias = "Systolic blood pressure")]
    systolic_blood_pressure: usize,
    #[serde(alias = "On SBP treatment?")]
    on_sbp_treatment: bool,
    #[serde(alias = "Total Cholesterol")]
    total_cholesterol: usize,
    #[serde(alias = "HDL Cholesterol")]
    hdl_cholesterol: usize,
    #[serde(alias = "Diabetic?")]
    diabetic: bool,
}

#[tokio::main(max_threads = 10_000)]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let score = warp::post()
        .and(warp::path("framingham"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(evaluate_score);

    let routes = hello.or(score);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

fn json_body() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn evaluate_score(person: Person) -> Result<impl warp::Reply, warp::Rejection> {

    let age_score = age_scoring(&person);
    let smoke_score = smoke_scoring(&person);
    let diabetic_score = diabetic_scoring(&person);
    let sbp_score = sbp_scoring(&person);
    let total_cholesterol_score = total_cholesterol_scoring(&person);
    let hdl_score = hdl_scoring(&person);
    let framingham_score = age_score + hdl_score + total_cholesterol_score + sbp_score + diabetic_score;
    let heart_age = heart_age(&person.sex, framingham_score);
    let cvd_risk = cvd_risk(&person.sex, framingham_score);

    let result = json!({
        "Age": person.age,
        "Sex": person.sex,
        "Smoker?": person.smoker,
        "Systolic blood pressure": person.systolic_blood_pressure,
        "On SBP treatment?": person.on_sbp_treatment,
        "Total Cholesterol": person.total_cholesterol,
        "HDL Cholesterol": person.hdl_cholesterol,
        "Diabetic?": person.diabetic,
        "Age score": age_score,
        "Diabetic score": diabetic_score,
        "Framingham score": framingham_score,
        "Total Cholesterol score": total_cholesterol_score,
        "HDL score": hdl_score,
        "CVD Risk":cvd_risk,
        "Heart age/vascular age":heart_age,
        "Smoker score":smoke_score,
        "SBP score": sbp_score
    });

    Ok(warp::reply::json(&result))
}

fn age_scoring(person : &Person) -> f64 {
    if person.age < 35 {
        return 0.0;
    } else if person.age < 40 {
        return 2.0;
    } else if person.sex.eq("Men") {
        if person.age < 45 {
            return 5.0;
        } else if person.age < 50 {
            return 6.0;
        } else if person.age < 55 {
            return 8.0;
        } else if person.age < 60 {
            return 10.0;
        } else if person.age < 65 {
            return 11.0;
        } else if person.age < 70 {
            return 12.0;
        } else if person.age < 75 {
            return 14.0;
        } else {
            return 15.0;
        }
    } else if person.sex.eq("Women") {
        if person.age < 45 {
            return 4.0;
        } else if person.age < 50 {
            return 5.0;
        } else if person.age < 55 {
            return 7.0;
        } else if person.age < 60 {
            return 8.0;
        } else if person.age < 65 {
            return 9.0;
        } else if person.age < 70 {
            return 10.0;
        } else if person.age < 75 {
            return 11.0;
        } else {
            return 12.0;
        }
    }
    0.0
}

fn smoke_scoring(person : &Person) -> f64 {
    if person.smoker == true { 
        if person.sex.eq("Women") {
            3.0;
        } else {
            4.0;
        }
    }
    0.0
}

fn diabetic_scoring(person : &Person) -> f64 {
    if person.diabetic == true { 
        if person.sex.eq("Women") {
            4.0;
        } else {
            3.0;
        }
    }
    0.0
}

fn sbp_scoring(person : &Person) -> f64 {

    if !person.on_sbp_treatment {
        if person.systolic_blood_pressure >= 120 &&  person.systolic_blood_pressure < 130 {
            return 0.0;
        } else if person.systolic_blood_pressure >= 130 &&  person.systolic_blood_pressure < 140 {
            return 1.0;
        }
    } else {
        if person.systolic_blood_pressure >= 120 &&  person.systolic_blood_pressure < 130 {
            return 2.0;
        } else if person.systolic_blood_pressure >= 130 &&  person.systolic_blood_pressure < 140 {
            return 3.0;
        }
    }

    if person.sex.eq("Men") {
        if !person.on_sbp_treatment {
            if person.systolic_blood_pressure < 120 {
                return -2.0;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 160 {
                return 2.0;
            } else if person.systolic_blood_pressure >= 160 {
                return 3.0;
            }
        } else {
            if person.systolic_blood_pressure < 120 {
                return 0.0;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 160 {
                return 4.0;
            } else if person.systolic_blood_pressure >= 160 {
                return 5.0;
            }
        }    
    } else {
        if !person.on_sbp_treatment {
            if person.systolic_blood_pressure < 120 {
                return -3.0;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 150 {
                return 2.0;
            } else if person.systolic_blood_pressure >= 150 &&  person.systolic_blood_pressure < 160 {
                return 4.0;
            } else if person.systolic_blood_pressure >= 160 {
                return 5.0;
            }
        } else {
            if person.systolic_blood_pressure < 120 {
                return -1.0;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 150 {
                return 5.0;
            } else if person.systolic_blood_pressure >= 150 &&  person.systolic_blood_pressure < 160 {
                return 6.0;
            } else if person.systolic_blood_pressure >= 160 {
                return 7.0;
            }
        }    
    }
    return 0.0;
}

fn total_cholesterol_scoring(person : &Person) -> f64 {

    if person.total_cholesterol < 160 {
        return 0.0;
    } else if person.total_cholesterol >= 160 && person.total_cholesterol < 200 {
        return 1.0;
    }

    if person.sex.eq("Men") {
        if person.total_cholesterol >= 200 && person.total_cholesterol < 240 {
            return 2.0;
        } else if person.total_cholesterol >= 240 && person.total_cholesterol < 280 {
            return 3.0;
        } else {
            return 4.0;
        }
    } else {
        if person.total_cholesterol >= 200 && person.total_cholesterol < 240 {
            return 3.0;
        } else if person.total_cholesterol >= 240 && person.total_cholesterol < 280 {
            return 4.0;
        } else {
            return 5.0;
        }
    }
}

fn hdl_scoring(person : &Person) -> f64 {

    if person.hdl_cholesterol >= 60 {
        return -2.0;
    } else if person.hdl_cholesterol >= 50 && person.hdl_cholesterol < 60 {
        return -1.0;
    } else if person.hdl_cholesterol >= 45 && person.hdl_cholesterol < 50 {
        return 0.0;
    } else if person.hdl_cholesterol >= 35 && person.hdl_cholesterol < 45 {
        return 1.0;
    } else if person.hdl_cholesterol < 35 {
        return 2.0;
    }

    return 0.0;
}

fn heart_age (sex : &String, framingham_score : f64) -> &str {
    if framingham_score >= 2.0 && framingham_score < 3.0 {
        return "34 y/o";
    } else if framingham_score >= 3.0 && framingham_score < 4.0 {
        return "36 y/o";
    }

    if sex.eq("Men") {
        if framingham_score >= 0.0 && framingham_score < 1.0 {
            return "30 y/o";
        } else if framingham_score >= 1.0 && framingham_score < 2.0 {
            return "32 y/o";
        } else if framingham_score >= 4.0 && framingham_score < 5.0 {
            return "38 y/o";
        } else if framingham_score >= 5.0 && framingham_score < 6.0 {
            return "40 y/o";
        } else if framingham_score >= 6.0 && framingham_score < 7.0 {
            return "42 y/o";
        } else if framingham_score >= 7.0 && framingham_score < 8.0 {
            return "45 y/o";
        } else if framingham_score >= 8.0 && framingham_score < 9.0 {
            return "48 y/o";
        } else if framingham_score >= 9.0 && framingham_score < 10.0 {
            return "51 y/o";
        } else if framingham_score >= 10.0 && framingham_score < 11.0 {
            return "54 y/o";
        } else if framingham_score >= 11.0 && framingham_score < 12.0 {
            return "57 y/o";
        } else if framingham_score >= 12.0 && framingham_score < 13.0 {
            return "60 y/o";
        } else if framingham_score >= 13.0 && framingham_score < 14.0 {
            return "64 y/o";
        } else if framingham_score >= 14.0 && framingham_score < 15.0 {
            return "68 y/o";
        } else if framingham_score >= 15.0 && framingham_score < 16.0 {
            return "72 y/o";
        } else if framingham_score >= 16.0 && framingham_score < 17.0 {
            return "76 y/o";
        } else if framingham_score >= 17.0 {
            return ">80 y/o";
        }
    } else {
        if framingham_score >= 1.0 && framingham_score < 2.0 {
            return "31 y/o";
        } else if framingham_score >= 4.0 && framingham_score < 5.0 {
            return "39 y/o";
        } else if framingham_score >= 5.0 && framingham_score < 6.0 {
            return "42 y/o";
        } else if framingham_score >= 6.0 && framingham_score < 7.0 {
            return "45 y/o";
        } else if framingham_score >= 7.0 && framingham_score < 8.0 {
            return "48 y/o";
        } else if framingham_score >= 8.0 && framingham_score < 9.0 {
            return "51 y/o";
        } else if framingham_score >= 9.0 && framingham_score < 10.0 {
            return "55 y/o";
        } else if framingham_score >= 10.0 && framingham_score < 11.0 {
            return "59 y/o";
        } else if framingham_score >= 11.0 && framingham_score < 12.0 {
            return "64 y/o";
        } else if framingham_score >= 12.0 && framingham_score < 13.0 {
            return "68 y/o";
        } else if framingham_score >= 13.0 && framingham_score < 14.0 {
            return "73 y/o";
        } else if framingham_score >= 14.0 && framingham_score < 15.0 {
            return "79 y/o";
        } else if framingham_score >= 15.0 {
            return ">80 y/o";
        }
    }
    
    return "<30 y/o";
}

fn cvd_risk (sex : &String, framingham_score : f64) -> &str {
    if sex.eq("Men") {
        if framingham_score <= -3.0 {
            return "<1 %";
        } else if framingham_score > -3.0 && framingham_score <= -2.0 {
            return "1.1 %";
        } else if framingham_score > -2.0 && framingham_score <= -1.0 {
            return "1.4 %";
        } else if framingham_score > -1.0 && framingham_score <= -0.0 {
            return "1.6 %";
        } else if framingham_score > 0.0 && framingham_score <= 1.0 {
            return "1.9 %";
        } else if framingham_score > 1.0 && framingham_score <= 2.0 {
            return "2.3 %";
        } else if framingham_score > 2.0 && framingham_score <= 3.0 {
            return "2.8 %";
        } else if framingham_score > 3.0 && framingham_score <= 4.0 {
            return "3.3 %";
        } else if framingham_score > 4.0 && framingham_score <= 5.0 {
            return "3.9 %";
        } else if framingham_score > 5.0 && framingham_score <= 6.0 {
            return "4.7 %";
        } else if framingham_score > 6.0 && framingham_score <= 6.0 {
            return "5.6 %";
        } else if framingham_score > 7.0 && framingham_score <= 8.0 {
            return "6.7 %";
        } else if framingham_score > 8.0 && framingham_score <= 9.0 {
            return "7.9 %";
        } else if framingham_score > 9.0 && framingham_score <= 10.0 {
            return "9.4 %";
        } else if framingham_score > 10.0 && framingham_score <= 11.0 {
            return "11.2 %";
        } else if framingham_score > 11.0 && framingham_score <= 12.0 {
            return "13.2 %";
        } else if framingham_score > 12.0 && framingham_score <= 13.0 {
            return "15.6 %";
        } else if framingham_score > 13.0 && framingham_score <= 14.0 {
            return "18.4 %";
        } else if framingham_score > 14.0 && framingham_score <= 15.0 {
            return "21.6 %";
        } else if framingham_score > 15.0 && framingham_score <= 16.0 {
            return "25.3 %";
        } else if framingham_score > 16.0 && framingham_score <= 17.0 {
            return "29.4 %";
        } else if framingham_score > 17.0 {
            return ">30 %";
        }
    } else {
        if framingham_score <= -2.0 {
            return "<1 %";
        } else if framingham_score > -2.0 && framingham_score <= -1.0 {
            return "1.0 %";
        } else if framingham_score > -1.0 && framingham_score <= 0.0 {
            return "1.2 %";
        } else if framingham_score > 0.0 && framingham_score <= 1.0 {
            return "1.5 %";
        } else if framingham_score > 1.0 && framingham_score <= 2.0 {
            return "1.7 %";
        } else if framingham_score > 2.0 && framingham_score <= 3.0 {
            return "2.0 %";
        } else if framingham_score > 3.0 && framingham_score <= 4.0 {
            return "2.4 %";
        } else if framingham_score > 4.0 && framingham_score <= 5.0 {
            return "2.8 %";
        } else if framingham_score > 5.0 && framingham_score <= 6.0 {
            return "3.3 %";
        } else if framingham_score > 6.0 && framingham_score <= 7.0 {
            return "3.9 %";
        } else if framingham_score > 7.0 && framingham_score <= 8.0 {
            return "4.5 %";
        } else if framingham_score > 8.0 && framingham_score <= 9.0 {
            return "5.3 %";
        } else if framingham_score > 9.0 && framingham_score <= 9.0 {
            return "6.3 %";
        } else if framingham_score > 10.0 && framingham_score <= 11.0 {
            return "7.3 %";
        } else if framingham_score > 11.0 && framingham_score <= 12.0 {
            return "8.6 %";
        } else if framingham_score > 12.0 && framingham_score <= 13.0 {
            return "10.0 %";
        } else if framingham_score > 13.0 && framingham_score <= 14.0 {
            return "11.7 %";
        } else if framingham_score > 14.0 && framingham_score <= 15.0 {
            return "13.7 %";
        } else if framingham_score > 15.0 && framingham_score <= 16.0 {
            return "15.9 %";
        } else if framingham_score > 16.0 && framingham_score <= 17.0 {
            return "18.5 %";
        } else if framingham_score > 17.0 && framingham_score <= 18.0 {
            return "21.5 %";
        } else if framingham_score > 18.0 && framingham_score <= 19.0 {
            return "24.8 %";
        } else if framingham_score > 19.0 && framingham_score <= 20.0 {
            return "28.5 %";
        } else if framingham_score > 20.0 {
           return  "<30 %";
        }
    }
    return "<1 %";
}