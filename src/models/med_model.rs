use serde::{Deserialize, Serialize};
use triton_grow::{helper::csv_parser::CSVParse, network::input::Input};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MedModel{
    id: f32,
    diagnosis: String,
    radius_mean: f32,
    texture_mean: f32,
    perimeter_mean: f32,
    area_mean: f32,
    smoothness_mean: f32,
    compactness_mean: f32,
    concavity_mean: f32,
    concave_points_mean: f32,
    symmetry_mean: f32,
    fractal_dimension_mean: f32,
    radius_se: f32,
    texture_se: f32,
    perimeter_se: f32,
    area_se: f32,
    smoothness_se: f32,
    compactness_se: f32,
    concavity_se: f32,
    concave_points_se: f32,
    symmetry_se: f32,
    fractal_dimension_se: f32,
    radius_worst: f32,
    texture_worst: f32,
    perimeter_worst: f32,
    area_worst: f32,
    smoothness_worst: f32,
    compactness_worst: f32,
    concavity_worst: f32,
    concave_points_worst: f32,
    symmetry_worst: f32,
    fractal_dimension_worst: f32
}

impl MedModel{
    pub fn get_from_path(path: &str) -> Result<Vec<MedModel>, Box<dyn std::error::Error>> {
        let mut string_parsed: Vec<Vec<String>> = vec![];
        string_parsed.parse_elem(path)?;
        let mut float_parsed: Vec<Vec<f32>> = vec![];
        float_parsed.parse_elem(path)?;
        let mut response: Vec<MedModel> = vec![];
        for i in 0..string_parsed.len(){
            let new_model: MedModel = MedModel { id: float_parsed[i][0], diagnosis: string_parsed[i][1].clone(), radius_mean: float_parsed[i][2], texture_mean: float_parsed[i][3], perimeter_mean: float_parsed[i][4], area_mean: float_parsed[i][5], smoothness_mean: float_parsed[i][6], compactness_mean: float_parsed[i][7], concavity_mean: float_parsed[i][8], concave_points_mean: float_parsed[i][9], symmetry_mean: float_parsed[i][0], fractal_dimension_mean: float_parsed[i][0], radius_se: float_parsed[i][10], texture_se: float_parsed[i][11], perimeter_se: float_parsed[i][12], area_se: float_parsed[i][13], smoothness_se: float_parsed[i][14], compactness_se: float_parsed[i][15], concavity_se: float_parsed[i][0], concave_points_se: float_parsed[i][0], symmetry_se: float_parsed[i][16], fractal_dimension_se: float_parsed[i][17], radius_worst: float_parsed[i][18], texture_worst: float_parsed[i][0], perimeter_worst: float_parsed[i][0], area_worst: float_parsed[i][0], smoothness_worst: float_parsed[i][19], compactness_worst: float_parsed[i][20], concavity_worst: float_parsed[i][21], concave_points_worst: float_parsed[i][23], symmetry_worst: float_parsed[i][24], fractal_dimension_worst: float_parsed[i][25] };
            response.push(new_model);
        }
        Ok(response)
    }
}

impl Input for MedModel{
    fn to_param(&self) -> Vec<f32> {
        let mut malignent: f32 = 0.0;
        if self.diagnosis.eq_ignore_ascii_case("m") {
            malignent = 1.0;
        }
        vec![malignent,
        self.smoothness_mean,
        self.radius_mean/40.0,
        self.texture_mean/40.0,
        self.fractal_dimension_mean/100000.0]
    }
}
