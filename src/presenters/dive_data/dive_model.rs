use crate::models::dive_model::DiveModel;

pub fn select_dive_model() -> DiveModel {
        println!("\nZHL16 Bulhmann model selected");
        DiveModel::create_zhl16_dive_model()
}