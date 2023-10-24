use anyhow::{bail, Result};
use regex::Regex;

const MAX_NAME_LENGTH: usize = u16::MAX as usize;

fn is_valid_namespace(ns: &str) -> bool {
    // An empty namespace is valid, so take a shortcut here.
    if ns.is_empty() {
        return true;
    }

    // Too long string is not valid.
    if ns.len() > MAX_NAME_LENGTH {
        return false;
    }

    // "/" is not valid.
    if ns == "/" {
        return false;
    }

    // If the topic name has a '~' is not valid.
    if ns.contains("~") {
        return false;
    }

    // If the topic name has a white space is not valid.
    if ns.contains(' ') {
        return false;
    }

    // It is not allowed to have two consecutive slashes.
    if ns.contains("//") {
        return false;
    }

    // It the topic name has a '@' is not valid.
    if ns.contains('@') {
        return false;
    }

    // If the topic name has a ':=' is not valid.
    if ns.contains(":=") {
        return false;
    }

    true
}

fn is_valid_partition(partition: &str) -> bool {
    // A valid namespace is also a valid partition.
    is_valid_namespace(partition)
}

pub(crate) fn is_valid_topic(topic: &str) -> bool {
    is_valid_namespace(topic) && !topic.is_empty()
}

pub(crate) fn fully_qualified_name(partition: &str, ns: &str, topic: &str) -> Result<String> {
    // Sanity check, first things first.
    if !is_valid_partition(partition) || !is_valid_namespace(ns) || !is_valid_topic(topic) {
        bail!("Invalid partition, namespace or topic");
    }

    let mut partition = partition.to_string();
    let mut ns = ns.to_string();
    let mut topic = topic.to_string();

    // If partition is not empty and does not start with slash, add it.
    if !partition.is_empty() && partition.chars().nth(0) != Some('/') {
        partition.insert(0, '/');
    }
    // If the partition contains a trailing slash, remove it.
    if !partition.is_empty() && partition.ends_with('/') {
        partition.pop();
    }
    // If the namespace does not contain a trailing slash, append it.
    if ns.is_empty() || ns.chars().last() != Some('/') {
        ns.push('/');
    }

    // If the namespace does not start with slash, add it.
    if ns.is_empty() || ns.chars().nth(0) != Some('/') {
        ns.insert(0, '/');
    }

    // If the topic ends in "/", remove it.
    if !topic.is_empty() && topic.ends_with('/') {
        topic.pop();
    }

    let mut name = String::new();

    // If the topic does starts with '/' is considered an absolute topic and the
    // namespace will not be prefixed.
    if !topic.is_empty() && topic.chars().nth(0) == Some('/') {
        name.push_str(&topic);
    } else {
        name.push_str(&ns);
        name.push_str(&topic);
    }

    // Add the partition prefix.
    name.insert_str(0, &format!("@{}@", partition));

    // Too long string is not valid.
    if name.len() > MAX_NAME_LENGTH {
        bail!("The fully qualified name is too long");
    }

    Ok(name)
}

fn decompose_fully_qualified_topic(fully_qualified_name: &str) -> Result<(String, String)> {
    let first_at = fully_qualified_name.find('@');
    let last_at = fully_qualified_name.rfind('@');

    if first_at != Some(0) || first_at == last_at || last_at == Some(fully_qualified_name.len() - 1)
    {
        bail!("Invalid fully qualified name");
    }

    let possible_partition = &fully_qualified_name[first_at.unwrap() + 1..last_at.unwrap()];
    let possible_topic = &fully_qualified_name[last_at.unwrap() + 1..];

    if !is_valid_partition(possible_partition) || !is_valid_topic(possible_topic) {
        bail!("Invalid partition or topic");
    }

    Ok((possible_partition.to_string(), possible_topic.to_string()))
}

fn as_valid_topic(topic: &str) -> Result<String> {
    // Substitute spaces with _
    let re_space = Regex::new(r"\s").unwrap();
    let valid_topic = re_space.replace_all(&topic, "_").to_string();

    // Remove special characters and combinations
    let re_special_chars = Regex::new(r"@|~|//|:=").unwrap();
    let mut valid_topic = re_special_chars.replace_all(&valid_topic, "").to_string();

    if !valid_topic.is_empty() && valid_topic.chars().nth(0) != Some('/') {
        valid_topic.insert(0, '/');
    }

    if !is_valid_topic(&valid_topic) {
        bail!("Invalid topic name [{}]", topic);
    }

    Ok(valid_topic)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_namespace() {
        let valid_ns = "/example/namespace";
        let invalid_ns = "/~invalid~namespace";
        assert!(is_valid_namespace(valid_ns));
        assert!(!is_valid_namespace(invalid_ns));
    }

    #[test]
    fn test_fully_qualified_name() {
        let partition = "my_partition";
        let ns = "/example/namespace";
        let topic = "my_topic";

        assert_eq!(
            "@/my_partition@/example/namespace/my_topic",
            fully_qualified_name(partition, ns, topic).unwrap()
        );
    }

    #[test]
    fn test_decompose_fully_qualified_topic() {
        let fully_qualified_name = "@my_partition@/example/namespace/my_topic";
        let (partition, namespace_and_topic) =
            decompose_fully_qualified_topic(fully_qualified_name).unwrap();

        assert_eq!(partition, "my_partition");
        assert_eq!(namespace_and_topic, "/example/namespace/my_topic");
    }

    #[test]
    fn test_as_valid_topic() {
        let namespace_and_topic = "//example/namespace/my topic";
        let valid_topic = as_valid_topic(&namespace_and_topic).unwrap();

        assert_eq!(valid_topic, "/example/namespace/my_topic");
    }
}
