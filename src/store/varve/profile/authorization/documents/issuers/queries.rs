use crate::store::varve::profile::{authorization::documents::issuers::IssuerRow, location::Location};
use framboid::account::profile::authorization::document::issuer::Issuer;
use sqlx::{Error, PgPool, Postgres, QueryBuilder, query_as};

impl IssuerRow {
    queries! {
        foreign = document;

        param = issuer: &Issuer;

        table = "issuers";

        names = ["name", "jurisdiction"];

        binds = [issuer.name(), Location::from(issuer.jurisdiction())];
    }
}