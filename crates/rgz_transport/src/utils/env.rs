use std::env;

pub(crate) fn non_negative_env_var(env_var: &str, default_value: u16) -> u16 {
    return if let Ok(str_val) = env::var(env_var) {
        if let Ok(val) = str_val.parse::<u16>() {
            val
        } else {
            eprintln!(
                "Invalid value for environment variable [{}]. Using default value [{}]",
                str_val, default_value
            );
            default_value
        }
    } else {
        default_value
    };
}
