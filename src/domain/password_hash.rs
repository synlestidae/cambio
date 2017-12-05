struct PasswordHash {
    private_hash: String;
}

impl From<String> for Option<PasswordHash> {
    from(src_str: String) -> Option<PasswordHash> {
        PasswordHash { 
            private_hash: src_string
        };
    }
}
