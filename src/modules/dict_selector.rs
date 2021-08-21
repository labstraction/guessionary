
static ITA_DICT: &'static str = include_str!("../../assets/ita.txt");

pub fn get_dict(country: &str) -> Vec<(i32, &str)>{
    ITA_DICT.split(",").enumerate().map(|(u,s)|(u as i32, s)).collect()
}
