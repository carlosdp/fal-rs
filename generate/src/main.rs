use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ModelGroup {
    key: String,
    label: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Model {
    id: String,
    title: String,
    category: String,
    #[serde(default)]
    tags: Vec<String>,
    short_description: String,
    thumbnail_url: String,
    model_url: String,
    stream_url: Option<String>,
    date: String,
    machine_type: Option<String>,
    license_type: Option<String>,
    group: Option<ModelGroup>,
    #[serde(default)]
    result_comparison: bool,
    #[serde(default)]
    highlighted: bool,
    pricing_info_override: Option<String>,
    credits_required: Option<i32>,
    endpoint_id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AppDataMetadata {
    openapi: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AppData {
    app_name: String,
    metadata: AppDataMetadata,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let models: Vec<Model> = client
        .get("https://fal.ai/api/models")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    let model = models.first().unwrap();

    let (owner, alias) = model
        .endpoint_id
        .split_once("/")
        .expect(&format!("could not split endpoint: {}", &model.endpoint_id));

    let app_data: AppData = client
        .get(format!(
            "https://fal.ai/api/models/app-data?owner={owner}&alias={alias}"
        ))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();

    println!("{:?}", app_data);
}
