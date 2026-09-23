use framboid::account::profile::credentials::{Credential, CredentialKind};
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

use crate::store::varve::profile::{
    credential::education::{EducationRow, status::Status},
    location::Location
};

impl EducationRow {
    queries! {
        foreign = credential;

        param = education: &Credential;

        table = "educations";

        names = ["degree", "discipline", "location", "school", "status"];

        binds = [
            match education.kind() {
                CredentialKind::Education { degree, .. } => degree,
            },
            match education.kind() {
                CredentialKind::Education { discipline, .. } => discipline,
            },
            match education.kind() {
                CredentialKind::Education { location, .. } => Location::from(location),
            },
            match education.kind() {
                CredentialKind::Education { school, .. } => school,
            },
            match education.kind() {
                CredentialKind::Education { status, .. } => Status::from(status),
            }
        ];
    }
}
