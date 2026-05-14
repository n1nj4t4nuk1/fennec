use std::sync::Arc;

use kernel::source::application::find_source::source_finder::SourceFinder;
use kernel::source::domain::errors::source_repository_error::SourceRepositoryError;
use kernel::source::domain::repositories::source_repository::SourceRepository;

use crate::src::mocks::source_repository_mock::SourceRepositoryMock;
use crate::src::source::domain::entities::mothers::source_mother::SourceMother;
use crate::src::source::domain::value_objects::mothers::source_id_mother::SourceIdMother;

fn make_finder(repo: Arc<SourceRepositoryMock>) -> SourceFinder {
    let repo: Arc<dyn SourceRepository> = repo;
    SourceFinder::new(repo)
}

#[tokio::test]
async fn it_returns_not_found_when_source_does_not_exist() {
    let repo = Arc::new(SourceRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(SourceIdMother::random()).await;

    assert!(matches!(result, Err(SourceRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_source_when_it_exists() {
    let source = SourceMother::random();
    let expected_id = source.id().clone();
    let repo = Arc::new(SourceRepositoryMock::that_returns_source(source));
    let finder = make_finder(repo);

    let result = finder.execute(expected_id.clone()).await.unwrap();

    assert_eq!(result.id().value(), expected_id.value());
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(SourceRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(SourceIdMother::random()).await;

    assert!(matches!(result, Err(SourceRepositoryError::Unexpected(_))));
}
