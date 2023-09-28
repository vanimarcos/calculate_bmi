use crate::height_gender::HeightGender;

fn calculate_bmi_male(height:f32) -> f32 {  
    const TWENTY_TWO: i32 = 22;
    TWENTY_TWO as f32 * height.powf(2.0) 
}

fn calculate_bmi_female(height:f32) -> f32 {
    const TWENTY_TWO: i32 = 22;
    const FEMALE_DEDUCT:f32 = 0.1;
    TWENTY_TWO as f32 * (height - FEMALE_DEDUCT).powf(2.0)
}

pub fn calculate_bmi(value: HeightGender) -> f32 {
    match value.gender {
        'M' => calculate_bmi_male(value.height),
        'F' => calculate_bmi_female(value.height),
         _  => panic!("Ocorreu um erro ao selecionar o genero")
    } 
}