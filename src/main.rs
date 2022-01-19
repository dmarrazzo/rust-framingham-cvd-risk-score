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
        "Framingham score":13,
        "Total Cholesterol score": total_cholesterol_score,
        "HDL score": hdl_score,
        "CVD Risk":"10.0 %",
        "Heart age/vascular age":"73 y/o",
        "Smoker score":smoke_score,
        "SBP score": sbp_score
    });

    Ok(warp::reply::json(&result))
}

fn age_scoring(person : &Person) -> usize {
    if person.age < 35 {
        return 0;
    } else if person.age < 40 {
        return 2;
    } else if person.sex.eq("Men") {
        if person.age < 45 {
            return 5;
        } else if person.age < 50 {
            return 6;
        } else if person.age < 55 {
            return 8;
        } else if person.age < 60 {
            return 10;
        } else if person.age < 65 {
            return 11;
        } else if person.age < 70 {
            return 12;
        } else if person.age < 75 {
            return 14;
        } else {
            return 15;
        }
    } else if person.sex.eq("Women") {
        if person.age < 45 {
            return 4;
        } else if person.age < 50 {
            return 5;
        } else if person.age < 55 {
            return 7;
        } else if person.age < 60 {
            return 8;
        } else if person.age < 65 {
            return 9;
        } else if person.age < 70 {
            return 10;
        } else if person.age < 75 {
            return 11;
        } else {
            return 12;
        }
    }
    0
}

fn smoke_scoring(person : &Person) -> usize {
    if person.smoker == true { 
        if person.sex.eq("Women") {
            3;
        } else {
            4;
        }
    }
    0
}

fn diabetic_scoring(person : &Person) -> usize {
    if person.diabetic == true { 
        if person.sex.eq("Women") {
            4;
        } else {
            3;
        }
    }
    0
}

fn sbp_scoring(person : &Person) -> isize {

    if !person.on_sbp_treatment {
        if person.systolic_blood_pressure >= 120 &&  person.systolic_blood_pressure < 130 {
            0;
        } else if person.systolic_blood_pressure >= 130 &&  person.systolic_blood_pressure < 140 {
            1;
        }
    } else {
        if person.systolic_blood_pressure >= 120 &&  person.systolic_blood_pressure < 130 {
            2;
        } else if person.systolic_blood_pressure >= 130 &&  person.systolic_blood_pressure < 140 {
            3;
        }
    }

    if person.sex.eq("Men") {
        if !person.on_sbp_treatment {
            if person.systolic_blood_pressure < 120 {
                return -2;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 160 {
                2;
            } else if person.systolic_blood_pressure >= 160 {
                3;
            }
        } else {
            if person.systolic_blood_pressure < 120 {
                return 0;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 160 {
                4;
            } else if person.systolic_blood_pressure >= 160 {
                5;
            }
        }    
    } else {
        if !person.on_sbp_treatment {
            if person.systolic_blood_pressure < 120 {
                return -3;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 150 {
                2;
            } else if person.systolic_blood_pressure >= 150 &&  person.systolic_blood_pressure < 160 {
                4;
            } else if person.systolic_blood_pressure >= 160 {
                5;
            }
        } else {
            if person.systolic_blood_pressure < 120 {
                return -1;
            } else if person.systolic_blood_pressure >= 140 &&  person.systolic_blood_pressure < 150 {
                5;
            } else if person.systolic_blood_pressure >= 150 &&  person.systolic_blood_pressure < 160 {
                6;
            } else if person.systolic_blood_pressure >= 160 {
                7;
            }
        }    
    }
    return 0;
}

fn total_cholesterol_scoring(person : &Person) -> isize {

    if person.total_cholesterol < 160 {
        0;
    } else if person.total_cholesterol >= 160 && person.total_cholesterol < 200 {
        1;
    }

    if person.sex.eq("Men") {
        if person.total_cholesterol >= 200 && person.total_cholesterol < 240 {
            2;
        } else if person.total_cholesterol >= 240 && person.total_cholesterol < 280 {
            3;
        } else {
            4;
        }
    } else {
        if person.total_cholesterol >= 200 && person.total_cholesterol < 240 {
            3;
        } else if person.total_cholesterol >= 240 && person.total_cholesterol < 280 {
            4;
        } else {
            5;
        }
    }

    return 0;
}

fn hdl_scoring(person : &Person) -> isize {

    if person.hdl_cholesterol >= 60 {
        return -2;
    } else if person.hdl_cholesterol >= 50 && person.hdl_cholesterol < 60 {
        return -1;
    } else if person.hdl_cholesterol >= 45 && person.hdl_cholesterol < 50 {
        return 0;
    } else if person.hdl_cholesterol >= 35 && person.hdl_cholesterol < 45 {
        return 1;
    } else if person.hdl_cholesterol < 35 {
        return 2;
    }

    return 0;
}