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
        "Total Cholesterol score":1,
        "HDL score":0,
        "CVD Risk":"10.0 %",
        "Heart age/vascular age":"73 y/o",
        "Smoker score":smoke_score,
        "SBP score":0
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
    if person.smoker == true { 
        if person.sex.eq("Women") {
            4;
        } else {
            3;
        }
    }
    0
}