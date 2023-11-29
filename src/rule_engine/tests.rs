#[cfg(test)]
mod tests {
    use crate::test_helper::initialize_user;

    #[actix_web::test]
    async fn test_fulter_rules() {
        crate::test::init();
        let user = initialize_user();
        user.delete_self().expect("should delete");
    }
}