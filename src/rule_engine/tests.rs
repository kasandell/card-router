#[cfg(test)]
mod tests {
    use crate::test_helper::initialize_user;

    #[actix_web::test]
    async fn test_filter_rules() {
        crate::test::init();
    }
}