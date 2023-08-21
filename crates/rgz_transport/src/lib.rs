use rgz_msgs::StringMsg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let msg = StringMsg {
            header: None,
            data: "Hello, World!".to_string(),
        };

        assert_eq!(msg.data, "Hello, World!");
    }
}
